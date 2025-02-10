use crate::ai_driver::{AiDriver, AiDriverDestination, FUEL_RESERVE, RENT_COST_MONTHLY};
use crate::driver::Fuel;
use crate::local_economy::LocalEconomy;
use crate::location::{Location, Money};
use crate::memory::Memory;
use crate::storage::{ItemContainer, Storage};
use bevy::prelude::*;
use rainmetal_goap::prelude::*;
use rand::prelude::*;
use std::hash::Hash;
use std::sync::Arc;
use bevy_inspector_egui::egui::Ui;
use crate::goap_inspector::{ui_show_ai_plans, DebugPlannerState};

#[derive(Component, Clone, Default, ActionComponent)]
pub struct GoToFuelStation(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct DiscoverAction(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct ExitCityAction(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct RefuelAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct EarnMoneyAction(ActionState);

#[derive(Component)]
pub struct WorkTimer(Timer);

impl Default for WorkTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Once))
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq, Debug, Component, PlannerState)]
pub struct State {
    fuel_cost: Option<i64>,
    money: i64,
    fuel: OrderedFloat<f64>,
    know_all_locations: bool,
    fuel_station: Option<Entity>,
    inside_city: Option<Entity>,
}

impl State {
    pub fn has_money(&self, price: &Option<i64>) -> bool {
        Some(self.money) >= *price
    }

    pub fn inside_city(&self, location: &Option<Entity>) -> bool {
        self.inside_city.is_some() && self.inside_city == *location
    }
}

impl DebugPlannerState for State {
    fn show_egui(&self, names: &Query<&Name>, ui: &mut Ui) {
        let fuel_station = self.fuel_station
            .map(|station| names.get(station).ok())
            .flatten()
            .map_or("None".to_string(), |name| name.to_string());

        let inside_city = self.inside_city
            .map(|city| names.get(city).ok())
            .flatten()
            .map_or("None".to_string(), |name| name.to_string());

        ui.horizontal(|ui| {
            ui.label("Fuel Cost");
            ui.label(&self.fuel_cost.map_or("None".to_string(), |fuel_cost| fuel_cost.to_string()));
        });

        ui.horizontal(|ui| {
            ui.label("Money");
            ui.label(&self.money.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Fuel");
            ui.label(format!("{:.02}", self.fuel.0));
        });

        ui.horizontal(|ui| {
            ui.label("Fuel Station");
            ui.label(&fuel_station);
        });

        ui.horizontal(|ui| {
            ui.label("Know All Locations");
            ui.label(&self.know_all_locations.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Inside City");
            ui.label(&inside_city);
        });
    }
}

pub fn update_state_memory(mut query: Query<(&mut State, &Transform, &Memory), Changed<Memory>>, locations: Query<&Transform, With<Location>>) {
    for (mut state, transform, memory) in query.iter_mut() {
        let near_station = memory.nearest_location_with(&transform.translation, Box::new(|location| location.storage.quantity("fuel") > 0));

        if let Some((station, entity)) = near_station {
            state.fuel_cost = Some(station.prices.sell_price("fuel") as i64);
            state.fuel_station = Some(entity);
        }

        state.know_all_locations = memory.locations.len() == locations.iter().len();
    }
}

pub fn update_state_fuel(mut query: Query<(&mut State, &Fuel), Changed<Fuel>>) {
    for (mut state, fuel) in query.iter_mut() {
        state.fuel.0 = fuel.0;
    }
}

pub fn update_state_money(mut query: Query<(&mut State, &Money), Changed<Money>>) {
    for (mut state, money) in query.iter_mut() {
        state.money = money.0;
    }
}

pub fn setup_planner_systems(mut app: &mut App) {
    app.add_systems(Update, ui_show_ai_plans::<State>);
    init_planner::<State>(&mut app);
}

pub fn setup_driver_ai(query: Query<(Entity, &Money, &Fuel), Added<AiDriver>>, mut commands: Commands) {
    for (entity, money, fuel) in query.iter() {
        let rent_goal = Goal::<State>::new("rent")
            .with_static_priority(3)
            .with_requirement(Arc::new(|s| s.money >= RENT_COST_MONTHLY))
            .with_distance(Arc::new(|s, d| d.add(&s.money, &RENT_COST_MONTHLY)));

        let fuel_goal = Goal::<State>::new("fuel")
            .with_static_priority(5)
            .with_requirement(Arc::new(|s| s.fuel >= OrderedFloat::from(FUEL_RESERVE)))
            .with_distance(Arc::new(|s, d| d.add(&s.fuel.0, &FUEL_RESERVE)));

        let discover_goal = Goal::<State>::new("discover")
            .with_static_priority(0)
            .with_requirement(Arc::new(|s| s.know_all_locations))
            .with_distance(Arc::new(|s, d| d.add_eq(&s.know_all_locations, &true)));

        let refuel_action = RefuelAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.fuel += 1.0; s }))
            .with_precondition(Arc::new(|s| s.inside_city(&s.fuel_station) && s.has_money(&s.fuel_cost)))
            .with_static_cost(1);

        let go_to_fuel_station = GoToFuelStation::new::<State>()
            .with_effect(Arc::new(|mut s| { s.inside_city = s.fuel_station; s }))
            .with_precondition(Arc::new(|s| s.inside_city.is_none() && s.fuel_station.is_some()))
            .with_static_cost(2);

        let exit_city_action = ExitCityAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.inside_city = None; s }))
            .with_precondition(Arc::new(|s| s.inside_city.is_some()))
            .with_static_cost(1);

        let discover_action = DiscoverAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.fuel_station = Some(Entity::PLACEHOLDER); s.know_all_locations = true; s }))
            .with_precondition(Arc::new(|s| !s.know_all_locations && s.inside_city.is_none()))
            .with_static_cost(10);

        let earn_money_action = EarnMoneyAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.money += 50; s }))
            .with_precondition(Arc::new(|s| s.inside_city.is_some()))
            .with_static_cost(9);

        let planner = Planner::new(
            vec![discover_goal, rent_goal, fuel_goal],
            vec![
                (Arc::new(DiscoverAction::default()), discover_action),
                (Arc::new(RefuelAction::default()), refuel_action),
                (Arc::new(GoToFuelStation::default()), go_to_fuel_station),
                (Arc::new(ExitCityAction::default()), exit_city_action),
                (Arc::new(EarnMoneyAction::default()), earn_money_action),
            ]
        );

        let state = State {
            fuel_cost: None,
            know_all_locations: false,
            fuel_station: None,
            inside_city: None,
            fuel: OrderedFloat::from(fuel.0),
            money: money.0,
        };

        commands.entity(entity).insert((planner, state));
    }
}

pub fn handle_go_to_fuel_station_action(
    mut commands: Commands,
    query: Query<(Entity, &Memory, &State), (With<AiDriver>, Without<AiDriverDestination>, With<GoToFuelStation>)>
) {
    for (entity, memory, state) in query.iter() {
        let Some(station) = state.fuel_station else {
            continue
        };

        let Some(data) = memory.locations.get(&station) else {
            continue
        };

        commands.entity(entity).insert(AiDriverDestination(data.value.position.xy()));
    }
}

pub fn handle_exit_city_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), (With<AiDriver>, With<ExitCityAction>)>,
) {
    for (entity, mut state) in query.iter_mut() {
        state.inside_city = None;
        commands.entity(entity).remove::<ExitCityAction>();
    }
}

pub fn handle_go_to_fuel_station_action_finish(
    trigger: Trigger<OnRemove, AiDriverDestination>,
    mut query: Query<(&Transform, &Memory, &mut State), (With<AiDriver>, With<GoToFuelStation>)>,
    mut commands: Commands
) {
    let Ok((transform, memory, mut state)) = query.get_mut(trigger.entity()) else {
        return
    };

    let Some(station) = state.fuel_station else {
        return
    };

    let Some(data) = memory.locations.get(&station) else {
        return;
    };

    if data.value.position.xy().distance(transform.translation.xy()) <= 0.5 {
        state.inside_city = Some(station);
        commands.entity(trigger.entity()).remove::<GoToFuelStation>();
    }
    else {
        commands.entity(trigger.entity()).insert(AiDriverDestination(data.value.position.xy()));
    }
}

pub fn handle_discover_action(
    mut commands: Commands,
    query: Query<Entity, (With<AiDriver>, Without<AiDriverDestination>, With<DiscoverAction>)>,
    locations: Query<&Transform, With<Location>>,
) {
    for entity in query.iter() {
        let Some(target) = locations.iter().choose(&mut thread_rng()) else {
            return
        };

        commands.entity(entity).insert(AiDriverDestination(target.translation.xy()));
    }
}

pub fn handle_discover_action_finish(
    trigger: Trigger<OnRemove, AiDriverDestination>,
    query: Query<&State, (With<AiDriver>, With<DiscoverAction>)>,
    locations: Query<&Transform, With<Location>>,
    mut commands: Commands
) {
    let Ok(state) = query.get(trigger.entity()) else {
        return
    };

    if state.know_all_locations {
        commands.entity(trigger.entity()).remove::<DiscoverAction>();
        return;
    }
    else {
        let Some(target) = locations.iter().choose(&mut thread_rng()) else {
            return
        };

        commands.entity(trigger.entity()).insert(AiDriverDestination(target.translation.xy()));
    }
}

pub fn handle_refuel_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Fuel, &mut Money, &State), (With<AiDriver>, With<RefuelAction>)>,
    mut cities: Query<(&mut Storage, &mut Money, &LocalEconomy), (With<Location>, Without<AiDriver>)>
) {
    for (npc, mut fuel, mut money, state) in query.iter_mut() {
        let Some(city) = state.inside_city else {
            continue
        };

        let Ok((mut city_storage, mut city_money, city_economy)) = cities.get_mut(city) else {
            continue
        };

        let price = city_economy.sell_price("fuel") as i64;

        if money.0 < price {
            return;
        }

        if city_storage.remove_one("fuel").is_none() {
            return;
        }

        money.0 -= price;
        city_money.0 += price;
        fuel.0 += 1.0;

        commands.entity(npc).remove::<RefuelAction>();
    }
}

pub fn start_work_action(trigger: Trigger<OnInsert, EarnMoneyAction>, mut commands: Commands) {
    commands.entity(trigger.entity()).insert_if_new(WorkTimer::default());
}

pub fn handle_work_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Money, &mut WorkTimer), (With<AiDriver>, With<EarnMoneyAction>)>,
    time: Res<Time>,
) {
    for (npc, mut money, mut work_timer) in query.iter_mut() {
        work_timer.0.tick(time.delta());

        if work_timer.0.just_finished() {
            money.0 += 50;
            commands.entity(npc).remove::<EarnMoneyAction>();
            commands.entity(npc).remove::<WorkTimer>();
        }
    }
}