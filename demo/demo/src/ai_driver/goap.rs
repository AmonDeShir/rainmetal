use crate::ai_driver::{AiDriver, AiDriverDestination};
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

const RENT_COST_MONTHLY: i64 = 120;
const FUEL_RESERVE: f64 = 30.0;

#[derive(Component, Clone, Default, ActionComponent)]
pub struct GoToNearCityAction(ActionState);

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

#[derive(Component, Clone)]
pub struct InsideCity(pub Entity);

impl Default for WorkTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Once))
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq, Debug, Component, PlannerState)]
pub struct State {
    fuel_cost: i64,
    money: i64,
    fuel: OrderedFloat<f64>,
    know_any_location: bool,
    know_all_locations: bool,
    inside_city: bool,
}

impl DebugPlannerState for State {
    fn show_egui(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Fuel Cost");
            ui.label(&self.fuel_cost.to_string());
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
            ui.label("Know Any Location");
            ui.label(&self.know_any_location.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Know All Locations");
            ui.label(&self.know_all_locations.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Inside City");
            ui.label(&self.inside_city.to_string());
        });
    }
}

pub fn update_state_memory(mut query: Query<(&mut State, &Transform, &Memory), Changed<Memory>>, locations: Query<&Transform, With<Location>>) {
    for (mut state, transform, memory) in query.iter_mut() {
        let near_station = memory.nearest_location_with(&transform.translation, Box::new(|location| location.storage.quantity("fuel") > 0));

        if let Some((station, _)) = near_station {
            state.fuel_cost = station.prices.sell_price("fuel") as i64;
        }

        state.know_all_locations = memory.locations.len() == locations.iter().len();
        state.know_any_location = memory.locations.len() > 0;
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
            .with_precondition(Arc::new(|s| s.inside_city && s.money >= s.fuel_cost))
            .with_static_cost(1);

        let go_to_near_city_action = GoToNearCityAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.inside_city = true; s }))
            .with_precondition(Arc::new(|s| !s.inside_city && s.know_any_location))
            .with_static_cost(2);

        let exit_city_action = ExitCityAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.inside_city = false; s }))
            .with_precondition(Arc::new(|s| s.inside_city))
            .with_static_cost(1);

        let discover_action = DiscoverAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.know_any_location = true; s.know_all_locations = true; s }))
            .with_precondition(Arc::new(|s| !s.know_all_locations && !s.inside_city))
            .with_static_cost(10);

        let earn_money_action = EarnMoneyAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.money += 50; s }))
            .with_precondition(Arc::new(|s| s.inside_city))
            .with_static_cost(9);

        let planner = Planner::new(
            vec![discover_goal, rent_goal, fuel_goal],
            vec![
                (Arc::new(DiscoverAction::default()), discover_action),
                (Arc::new(RefuelAction::default()), refuel_action),
                (Arc::new(GoToNearCityAction::default()), go_to_near_city_action),
                (Arc::new(ExitCityAction::default()), exit_city_action),
                (Arc::new(EarnMoneyAction::default()), earn_money_action),
            ]
        );

        let state = State {
            fuel_cost: 10,
            know_all_locations: false,
            know_any_location: false,
            inside_city: false,
            fuel: OrderedFloat::from(fuel.0),
            money: money.0,
        };

        commands.entity(entity).insert((planner, state));
    }
}

pub fn handle_go_to_near_city_action(
    mut commands: Commands,
    query: Query<(Entity, &Memory, &Transform), (With<AiDriver>, Without<AiDriverDestination>, With<GoToNearCityAction>)>
) {
    for (entity, memory, transform) in query.iter() {
        let Some((location, _)) = memory.nearest_location(&transform.translation) else {
            continue;
        };

        commands.entity(entity).insert(AiDriverDestination(location.position.xy()));
    }
}

pub fn handle_exit_city_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), (With<AiDriver>, With<ExitCityAction>)>,
) {
    for (entity, mut state) in query.iter_mut() {
        state.inside_city = false;
        commands.entity(entity).remove::<InsideCity>();
        commands.entity(entity).remove::<ExitCityAction>();
    }
}

pub fn handle_go_to_near_city_action_finish(
    trigger: Trigger<OnRemove, AiDriverDestination>,
    mut query: Query<(&Memory, &Transform, &mut State), (With<AiDriver>, With<GoToNearCityAction>)>,
    mut commands: Commands
) {
    let Ok((memory, transform, mut state)) = query.get_mut(trigger.entity()) else {
        return
    };

    let Some((location, city)) = memory.nearest_location(&transform.translation) else {
        return;
    };

    if location.position.xy().distance(transform.translation.xy()) <= 0.5 {
        state.inside_city = true;
        commands.entity(trigger.entity()).insert(InsideCity(city));
        commands.entity(trigger.entity()).remove::<GoToNearCityAction>();
    }
    else {
        commands.entity(trigger.entity()).insert(AiDriverDestination(location.position.xy()));
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
    mut query: Query<(&Memory, &mut State), (With<AiDriver>, With<DiscoverAction>)>,
    locations: Query<&Transform, With<Location>>,
    mut commands: Commands
) {
    let Ok((memory, mut state)) = query.get_mut(trigger.entity()) else {
        return
    };

    let know_locations = memory.locations.iter().count();

    if know_locations > 0 {
        state.know_any_location = true;
    }

    if know_locations >= locations.iter().count() {
        state.know_all_locations = true;
        commands.entity(trigger.entity()).remove::<DiscoverAction>();
        return;
    }
    else {
        state.know_all_locations = false;

        let Some(target) = locations.iter().choose(&mut thread_rng()) else {
            return
        };

        commands.entity(trigger.entity()).insert(AiDriverDestination(target.translation.xy()));
    }
}

pub fn handle_refuel_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Fuel, &mut Money, &InsideCity), (With<AiDriver>, With<RefuelAction>)>,
    mut cities: Query<(&mut Storage, &mut Money, &LocalEconomy), (With<Location>, Without<AiDriver>)>
) {
    for (npc, mut fuel, mut money, InsideCity(city)) in query.iter_mut() {
        let Ok((mut city_storage, mut city_money, city_economy)) = cities.get_mut(*city) else {
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