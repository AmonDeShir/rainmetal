use crate::ai_driver::{AiDriver, AiDriverDestination, BIG_TRADING_PLAN_VALUE, FUEL_RESERVE, HUGE_TRADING_PLAN_VALUE, MEDIUM_TRADING_PLAN_VALUE, RENT_COST_MONTHLY, SMALL_TRADING_PLAN_VALUE};
use crate::ai_trader::TradingPlanByValue;
use crate::driver::Fuel;
use crate::goap_inspector::{ui_show_ai_plans, DebugPlannerState};
use crate::local_economy::LocalEconomy;
use crate::location::{Location, Money};
use crate::memory::Memory;
use crate::storage::{ItemContainer, Storage};
use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;
use rainmetal_goap::prelude::*;
use rand::prelude::*;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Component, Clone, Default, ActionComponent)]
pub struct SetDestinationToFuelStation(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct GoToDestination(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct DiscoverAction(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct ExitCityAction(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct RefuelAction(ActionState);

#[derive(Component, Clone, Default, ActionComponent)]
pub struct FindTradePlansAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct DoTradingAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct RemoveTradingGoalAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct SetTradeGoalToSmallAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct SetTradeGoalToMediumAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct SetTradeGoalToBigAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct SetTradeGoalToHugeAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct ChiloutAction(ActionState);

#[derive(Component, Default, Clone, ActionComponent)]
pub struct UpdateTradingPathAction(ActionState);

#[derive(Clone, Default, Eq, Hash, PartialEq, Debug, Component, PlannerState)]
pub struct State {
    fuel_cost: Option<u64>,
    money: u64,
    fuel: OrderedFloat<f64>,
    all_locations: u64,
    know_locations: u64,
    travel_destination: Option<Entity>,
    fuel_station: Option<Entity>,
    inside_city: Option<Entity>,
    trading_goal: Option<u64>,
    has_small_trade_plan: bool,
    has_medium_trade_plan: bool,
    has_big_trade_plan: bool,
    has_huge_trade_plan: bool,
    has_updated_trading_path: bool,
    is_chilling: bool,
}

impl State {
    pub fn has_money(&self, price: &Option<u64>) -> bool {
        Some(self.money) >= *price
    }

    pub fn inside_city(&self, location: &Option<Entity>) -> bool {
        self.inside_city.is_some() && self.inside_city == *location
    }

    pub fn can_go_to(&self, location: &Option<Entity>) -> bool {
        location.is_some() && self.travel_destination != *location
    }
}

#[derive(Component)]
pub struct LocationsBeforeDiscover(pub u64);

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

        let travel_destination = self.travel_destination
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
            ui.label("All Locations");
            ui.label(&self.all_locations.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Know Locations");
            ui.label(&self.know_locations.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Inside City");
            ui.label(&inside_city);
        });

        ui.horizontal(|ui| {
            ui.label("Travel Destination");
            ui.label(&travel_destination);
        });

        ui.horizontal(|ui| {
            ui.label("Trading Goal");
            ui.label(&self.trading_goal.map_or("None".to_string(), |v| v.to_string()));
        });

        ui.horizontal(|ui| {
            ui.label("Small Trading Plan");
            ui.label(&self.has_small_trade_plan.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Medium Trading Plan");
            ui.label(&self.has_medium_trade_plan.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Big Trading Plan");
            ui.label(&self.has_big_trade_plan.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Huge Trading Plan");
            ui.label(&self.has_huge_trade_plan.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Huge Trading Plan");
            ui.label(&self.has_huge_trade_plan.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Has updated trading path");
            ui.label(&self.has_updated_trading_path.to_string());
        });
    }
}

pub fn update_state_memory(mut query: Query<(&mut State, &Transform, &Memory), Changed<Memory>>, locations: Query<&Transform, With<Location>>) {
    for (mut state, transform, memory) in query.iter_mut() {
        let near_station = memory.nearest_location_with(&transform.translation, Box::new(|location| location.storage.quantity("fuel") > 0));

        if let Some((station, entity)) = near_station {
            state.fuel_cost = Some(station.prices.sell_price("fuel") as u64);
            state.fuel_station = Some(entity);
        }

        state.know_locations = memory.locations.len() as u64;
        state.all_locations = locations.iter().len() as u64;
    }
}

pub fn update_state_fuel(mut query: Query<(&mut State, &Fuel), Changed<Fuel>>) {
    for (mut state, fuel) in query.iter_mut() {
        state.fuel.0 = fuel.0;
    }
}

pub fn update_state_money(mut query: Query<(&mut State, &Money), Changed<Money>>) {
    for (mut state, money) in query.iter_mut() {
        state.money = money.0 as u64;
    }
}

pub fn update_state_trading_plans(mut query: Query<(&mut State, &TradingPlanByValue), Changed<TradingPlanByValue>>) {
    for (mut state, plans) in query.iter_mut() {
        state.has_small_trade_plan = plans.small.is_some();
        state.has_medium_trade_plan = plans.medium.is_some();
        state.has_big_trade_plan = plans.big.is_some();
        state.has_huge_trade_plan = plans.huge.is_some();


        if plans.all_none() {
            state.trading_goal = None;
            state.has_updated_trading_path = false;
        }
        else {
            state.has_updated_trading_path = true;
        }
    }
}

pub fn setup_planner_systems(mut app: &mut App) {
    app.add_systems(Update, ui_show_ai_plans::<State>);
    init_planner::<State>(&mut app);
}

pub fn setup_driver_ai(query: Query<(Entity, &Money, &Fuel), Added<AiDriver>>, locations: Query<&Location>, mut commands: Commands) {
    for (entity, money, fuel) in query.iter() {
        let rent_goal = Goal::<State>::new("rent")
            .with_static_priority(3)
            .with_requirement(Arc::new(|s| s.money >= RENT_COST_MONTHLY * 2))
            .with_distance(Arc::new(|s, d| d.add(&s.money, &(RENT_COST_MONTHLY * 2))));

        let fuel_goal = Goal::<State>::new("fuel")
            .with_static_priority(5)
            .with_requirement(Arc::new(|s| s.fuel >= OrderedFloat::from(FUEL_RESERVE)))
            .with_distance(Arc::new(|s, d| d.add(&s.fuel.0, &FUEL_RESERVE)));

        let discover_goal = Goal::<State>::new("discover")
            .with_static_priority(1)
            .with_requirement(Arc::new(|s| s.know_locations >= s.all_locations))
            .with_distance(Arc::new(|s, d| d.add_eq(&s.know_locations, &s.all_locations)));

        let chillout = Goal::<State>::new("chillout")
            .with_static_priority(0)
            .with_requirement(Arc::new(|s| s.is_chilling))
            .with_distance(Arc::new(|s, d| d.add_eq(&s.know_locations, &s.all_locations)));


        let refuel_action = RefuelAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.fuel += 1.0; s.fuel_cost.as_ref().map(|cost| s.money -= cost); s }))
            .with_precondition(Arc::new(|s| s.inside_city(&s.fuel_station) && s.has_money(&s.fuel_cost)))
            .with_static_cost(1);

        let go_to_destination = GoToDestination::new::<State>()
            .with_effect(Arc::new(|mut s| { s.inside_city = s.travel_destination; s }))
            .with_precondition(Arc::new(|s| s.inside_city.is_none() && s.travel_destination.is_some()))
            .with_static_cost(2);

        let set_destination_to_fuel_station = SetDestinationToFuelStation::new::<State>()
            .with_effect(Arc::new(|mut s| { s.travel_destination = s.fuel_station; s }))
            .with_precondition(Arc::new(|s| s.can_go_to(&s.fuel_station)))
            .with_static_cost(2);

        let exit_city_action = ExitCityAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.inside_city = None; s }))
            .with_precondition(Arc::new(|s| s.inside_city.is_some()))
            .with_static_cost(1);

        let discover_action = DiscoverAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.fuel_station = Some(Entity::PLACEHOLDER); s.know_locations += 1; s }))
            .with_precondition(Arc::new(|s| s.know_locations < s.all_locations && s.inside_city.is_none()))
            .with_static_cost(10);

        let update_trading_path_action = UpdateTradingPathAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.has_updated_trading_path = true; s }))
            .with_precondition(Arc::new(|s| !s.has_updated_trading_path && s.know_locations >= 3))
            .with_static_cost(8);

        let find_trade_plans_action = FindTradePlansAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.has_small_trade_plan = true; s }))
            .with_precondition(Arc::new(|s|s.has_small_trade_plan == false &&  s.has_updated_trading_path && s.know_locations >= 3))
            .with_static_cost(1);

        let remove_trading_goal_action = RemoveTradingGoalAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.trading_goal = None; s }))
            .with_precondition(Arc::new(|s| s.trading_goal.is_some()))
            .with_static_cost(1);

        let set_trade_goal_to_small_action = SetTradeGoalToSmallAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.trading_goal = Some(SMALL_TRADING_PLAN_VALUE); s }))
            .with_precondition(Arc::new(|s| s.trading_goal.is_none() && s.has_small_trade_plan))
            .with_static_cost(20);

        let set_trade_goal_to_medium_action = SetTradeGoalToMediumAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.trading_goal = Some(MEDIUM_TRADING_PLAN_VALUE); s }))
            .with_precondition(Arc::new(|s| s.trading_goal.is_none() && s.has_medium_trade_plan))
            .with_static_cost(12);

        let set_trade_goal_to_big_action = SetTradeGoalToBigAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.trading_goal = Some(BIG_TRADING_PLAN_VALUE); s }))
            .with_precondition(Arc::new(|s| s.trading_goal.is_none() && s.has_big_trade_plan))
            .with_static_cost(5);

        let set_trade_goal_to_huge_action = SetTradeGoalToHugeAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.trading_goal = Some(HUGE_TRADING_PLAN_VALUE); s }))
            .with_precondition(Arc::new(|s| s.trading_goal.is_none() && s.has_huge_trade_plan))
            .with_static_cost(2);

        let do_trading_action = DoTradingAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.trading_goal.map(|goal| s.money += goal); s }))
            .with_precondition(Arc::new(|s| s.trading_goal.is_some() && s.money <= u32::MAX as u64))
            .with_static_cost(7);

        let chillout_action = ChiloutAction::new::<State>()
            .with_effect(Arc::new(|mut s| { s.is_chilling = true; s }))
            .with_precondition(Arc::new(|s| s.money >= RENT_COST_MONTHLY && !s.is_chilling && s.inside_city.is_some()))
            .with_static_cost(8);


        let planner = Planner::new(
            vec![discover_goal, rent_goal, fuel_goal, chillout],
            vec![
                (Arc::new(DiscoverAction::default()), discover_action),
                (Arc::new(RefuelAction::default()), refuel_action),
                (Arc::new(GoToDestination::default()), go_to_destination),
                (Arc::new(SetDestinationToFuelStation::default()), set_destination_to_fuel_station),
                (Arc::new(ExitCityAction::default()), exit_city_action),
                (Arc::new(FindTradePlansAction::default()), find_trade_plans_action),
                (Arc::new(UpdateTradingPathAction::default()), update_trading_path_action),
                (Arc::new(RemoveTradingGoalAction::default()), remove_trading_goal_action),
                (Arc::new(SetTradeGoalToSmallAction::default()), set_trade_goal_to_small_action),
                (Arc::new(SetTradeGoalToMediumAction::default()), set_trade_goal_to_medium_action),
                (Arc::new(SetTradeGoalToBigAction::default()), set_trade_goal_to_big_action),
                (Arc::new(SetTradeGoalToHugeAction::default()), set_trade_goal_to_huge_action),
                (Arc::new(DoTradingAction::default()), do_trading_action),
                (Arc::new(ChiloutAction::default()), chillout_action),
            ]
        );

        let state = State {
            fuel_cost: None,
            all_locations: locations.iter().len() as u64,
            know_locations: 0,
            fuel_station: None,
            inside_city: None,
            travel_destination: None,
            fuel: OrderedFloat::from(fuel.0),
            money: money.0 as u64,
            trading_goal: None,
            has_small_trade_plan: false,
            has_medium_trade_plan: false,
            has_big_trade_plan: false,
            has_huge_trade_plan: false,
            has_updated_trading_path: false,
            is_chilling: false,
        };

        commands.entity(entity).insert((planner, state));
    }
}

pub fn handle_set_destination_to_fuel_station_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), (With<AiDriver>, Without<AiDriverDestination>, With<SetDestinationToFuelStation>)>
) {
    for (entity, mut state) in query.iter_mut() {
        let Some(location) = state.fuel_station else {
            continue
        };

        state.travel_destination = Some(location);
        commands.entity(entity).remove::<SetDestinationToFuelStation>();
    }
}

pub fn handle_go_to_destination_action(
    mut commands: Commands,
    query: Query<(Entity, &Memory, &State), (With<AiDriver>, Without<AiDriverDestination>, With<GoToDestination>)>
) {
    for (entity, memory, state) in query.iter() {
        let Some(location) = state.travel_destination else {
            continue
        };

        let Some(data) = memory.locations.get(&location) else {
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

pub fn handle_go_to_destination_action_finish(
    trigger: Trigger<OnRemove, AiDriverDestination>,
    mut query: Query<(&Transform, &Memory, &mut State), (With<AiDriver>, With<GoToDestination>)>,
    mut commands: Commands
) {
    let Ok((transform, memory, mut state)) = query.get_mut(trigger.entity()) else {
        return
    };

    let Some(location) = state.travel_destination else {
        return
    };

    let Some(data) = memory.locations.get(&location) else {
        return;
    };

    if data.value.position.xy().distance(transform.translation.xy()) <= 0.5 {
        state.inside_city = Some(location);
        commands.entity(trigger.entity()).remove::<GoToDestination>();
    }
    else {
        commands.entity(trigger.entity()).insert(AiDriverDestination(data.value.position.xy()));
    }
}

pub fn handle_discover_action(
    mut commands: Commands,
    query: Query<(Entity, &Memory), (With<AiDriver>, Without<AiDriverDestination>, With<DiscoverAction>)>,
    locations: Query<&Transform, With<Location>>,
) {
    for (entity, memory) in query.iter() {
        let Some(target) = locations.iter().choose(&mut thread_rng()) else {
            return
        };

        commands.entity(entity).insert_if_new(LocationsBeforeDiscover(memory.locations.len() as u64));
        commands.entity(entity).insert(AiDriverDestination(target.translation.xy()));
    }
}

pub fn handle_discover_action_finish(
    trigger: Trigger<OnRemove, AiDriverDestination>,
    query: Query<(&State, &LocationsBeforeDiscover), (With<AiDriver>, With<DiscoverAction>)>,
    locations: Query<&Transform, With<Location>>,
    mut commands: Commands
) {
    let Ok((state, old_location_count)) = query.get(trigger.entity()) else {
        return
    };

    if state.know_locations > old_location_count.0 || state.know_locations >= state.all_locations {
        commands.entity(trigger.entity()).remove::<DiscoverAction>();
        commands.entity(trigger.entity()).remove::<LocationsBeforeDiscover>();

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
            commands.entity(npc).remove::<RefuelAction>();
        }

        if city_storage.remove_one("fuel").is_none() {
            commands.entity(npc).remove::<RefuelAction>();
        }

        money.0 -= price;
        city_money.0 += price;
        fuel.0 += 1.0;

        commands.entity(npc).remove::<RefuelAction>();
    }
}

pub fn handle_find_trade_plans_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State, &TradingPlanByValue), (With<AiDriver>, With<FindTradePlansAction>)>
) {
    for (entity, mut state, plans) in query.iter_mut() {
        if plans.small.is_some() {
            state.has_small_trade_plan = true;
            commands.entity(entity).remove::<FindTradePlansAction>();
        }

        if plans.medium.is_some() {
            state.has_medium_trade_plan = true;
            commands.entity(entity).remove::<FindTradePlansAction>();
        }

        if plans.big.is_some() {
            state.has_big_trade_plan = true;
            commands.entity(entity).remove::<FindTradePlansAction>();
        }

        if plans.huge.is_some() {
            state.has_huge_trade_plan = true;
            commands.entity(entity).remove::<FindTradePlansAction>();
        }
    }
}

pub fn handle_update_trading_path_action(
    mut commands: Commands,
    query: Query<(Entity, &Memory), (With<AiDriver>, Without<AiDriverDestination>, With<UpdateTradingPathAction>)>,
) {
    for (entity, memory) in query.iter() {
        let Some(target) = memory.locations.iter().choose(&mut thread_rng()) else {
            return
        };

        commands.entity(entity).insert(AiDriverDestination(target.1.value.position.xy()));
    }
}

pub fn handle_update_trading_path_action_finish(
    trigger: Trigger<OnRemove, AiDriverDestination>,
    query: Query<(Entity, &State, &Memory), (With<AiDriver>, With<UpdateTradingPathAction>)>,
    mut commands: Commands
) {
    let Ok((entity, state, memory)) = query.get(trigger.entity()) else {
        return
    };

    if state.has_updated_trading_path {
        commands.entity(trigger.entity()).remove::<UpdateTradingPathAction>();
    }
    else {
        let Some(target) = memory.locations.iter().choose(&mut thread_rng()) else {
            return
        };

        commands.entity(entity).insert(AiDriverDestination(target.1.value.position.xy()));
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TradingPhase {
    BUY,
    SELL,
}

#[derive(Component, Debug)]
pub struct CurrentTradingData {
    pub phase: TradingPhase,
    pub item: String,
}

pub fn handle_remove_trading_goal_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), With<RemoveTradingGoalAction>>,
) {
    for (entity, mut state) in query.iter_mut() {
        state.trading_goal = None;
        commands.entity(entity).remove::<RemoveTradingGoalAction>();
    }
}

pub fn handle_set_trading_goal_to_small_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), With<SetTradeGoalToSmallAction>>,
) {
    for (entity, mut state) in query.iter_mut() {
        state.trading_goal = Some(SMALL_TRADING_PLAN_VALUE);
        commands.entity(entity).remove::<SetTradeGoalToSmallAction>();
    }
}

pub fn handle_set_trading_goal_to_medium_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), With<SetTradeGoalToMediumAction>>,
) {
    for (entity, mut state) in query.iter_mut() {
        state.trading_goal = Some(MEDIUM_TRADING_PLAN_VALUE);
        commands.entity(entity).remove::<SetTradeGoalToMediumAction>();
    }
}

pub fn handle_set_trading_goal_to_big_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), With<SetTradeGoalToBigAction>>,
) {
    for (entity, mut state) in query.iter_mut() {
        state.trading_goal = Some(BIG_TRADING_PLAN_VALUE);
        commands.entity(entity).remove::<SetTradeGoalToBigAction>();
    }
}

pub fn handle_set_trading_goal_to_huge_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut State), With<SetTradeGoalToHugeAction>>,
) {
    for (entity, mut state) in query.iter_mut() {
        state.trading_goal = Some(HUGE_TRADING_PLAN_VALUE);
        commands.entity(entity).remove::<SetTradeGoalToHugeAction>();
    }
}

pub fn handle_init_phase_of_do_trading_action(
    mut commands: Commands,
    query: Query<(Entity, &TradingPlanByValue, Option<&CurrentTradingData>), With<DoTradingAction>>,
) {
    for (entity, plans, data) in query.iter() {
        let Some(plan) = &plans.small else {
            continue
        };

        if let Some(data) = data {
            if data.item == plan.item {
                continue
            }
        }

        commands.entity(entity).insert(CurrentTradingData {
            item: plan.item.clone(),
            phase: TradingPhase::BUY,
        });
    }
}

pub fn handle_buy_phase_of_do_trading_action(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut State, &mut CurrentTradingData, &TradingPlanByValue, &mut Money, &mut Storage), With<DoTradingAction>>,
    mut locations: Query<(&Transform, &mut LocalEconomy, &mut Money, &mut Storage), Without<DoTradingAction>>,
) {
    for (entity, transform, mut state, mut trading_data, plans, mut money, mut npc_storage) in query.iter_mut() {
        if trading_data.phase != TradingPhase::BUY {
            continue
        }

        let plan = match state.trading_goal {
            Some(SMALL_TRADING_PLAN_VALUE) => &plans.small,
            Some(MEDIUM_TRADING_PLAN_VALUE) => &plans.medium,
            Some(BIG_TRADING_PLAN_VALUE) => &plans.big,
            Some(HUGE_TRADING_PLAN_VALUE) => &plans.huge,
            _ => {
                cancel_trading_action(&entity, &mut state, &mut commands);
                continue
            }
        };

        let Some(plan) = &plan else {
            cancel_trading_action(&entity, &mut state, &mut commands);
            continue
        };

        if npc_storage.quantity(&plan.item) >= plan.count as i32 {
            trading_data.phase = TradingPhase::SELL;
            continue;
        }

        let Ok((town_location, town_economy, mut town_money, mut town_storage)) = locations.get_mut(plan.from) else {
            cancel_trading_action(&entity, &mut state, &mut commands);
            continue
        };

        if town_location.translation.xy().distance(transform.translation.xy()) > 0.5 {
            commands.entity(entity).insert(AiDriverDestination(town_location.translation.xy()));
            continue
        }

        let price = town_economy.sell_price(&plan.item) as i64;

        if trade_item(&plan.item, &price, &mut money, &mut npc_storage, &mut town_money, &mut town_storage).is_none() {
            cancel_trading_action(&entity, &mut state, &mut commands);
        }
    }
}

pub fn handle_sell_phase_of_do_trading_action(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut State, &mut CurrentTradingData, &TradingPlanByValue, &mut Money, &mut Storage), With<DoTradingAction>>,
    mut locations: Query<(&Transform, &mut LocalEconomy, &mut Money, &mut Storage), Without<DoTradingAction>>,
) {
    for (entity, transform, mut state, trading_data, plans, mut money, mut npc_storage) in query.iter_mut() {
        if trading_data.phase != TradingPhase::SELL {
            continue
        }

        let plan = match state.trading_goal {
            Some(SMALL_TRADING_PLAN_VALUE) => &plans.small,
            Some(MEDIUM_TRADING_PLAN_VALUE) => &plans.medium,
            Some(BIG_TRADING_PLAN_VALUE) => &plans.big,
            Some(HUGE_TRADING_PLAN_VALUE) => &plans.huge,
            _ => {
                cancel_trading_action(&entity, &mut state, &mut commands);
                continue
            }
        };

        let Some(plan) = &plan else {
            cancel_trading_action(&entity, &mut state, &mut commands);
            continue
        };

        let Ok((town_location, town_economy, mut town_money, mut town_storage)) = locations.get_mut(plan.to) else {
            cancel_trading_action(&entity, &mut state, &mut commands);
            continue
        };

        if town_location.translation.xy().distance(transform.translation.xy()) > 0.5 {
            commands.entity(entity).insert(AiDriverDestination(town_location.translation.xy()));
            continue
        }

        if npc_storage.quantity(&plan.item) == 0 {
            state.trading_goal = None;
            commands.entity(entity).remove::<CurrentTradingData>();
            commands.entity(entity).remove::<DoTradingAction>();

            continue;
        }

        let price = town_economy.buy_price(&plan.item) as i64;

        if trade_item(&plan.item, &price, &mut town_money, &mut town_storage, &mut money, &mut npc_storage).is_none() {
            cancel_trading_action(&entity, &mut state, &mut commands);
        }
    }
}

fn cancel_trading_action(entity: &Entity, state: &mut State, commands: &mut Commands) {
    state.trading_goal = None;
    state.has_small_trade_plan = false;
    state.has_medium_trade_plan = false;
    state.has_big_trade_plan = false;
    state.has_huge_trade_plan = false;
    commands.entity(*entity).remove::<DoTradingAction>();
}

fn trade_item(item: &str, price: &i64, buyer_money: &mut Money, buyer_storage: &mut Storage, seller_money: &mut Money, seller_storage: &mut Storage) -> Option<()> {
    if buyer_money.0 < *price {
        return None
    }

    if seller_storage.remove_one(item).is_none() {
        return None
    }

    buyer_money.0 -= price;
    seller_money.0 += price;
    buyer_storage.add_one(item);

    Some(())
}

#[derive(Component)]
pub struct ChilloutTimer(pub Timer);

pub fn handle_init_chillout_action(
    mut commands: Commands,
    query: Query<Entity, (With<ChiloutAction>, Without<ChilloutTimer>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(ChilloutTimer(Timer::from_seconds(20.0, TimerMode::Repeating)));
    }
}

pub fn handle_chillout_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Money, &State, &mut ChilloutTimer), With<ChiloutAction>>,
    mut town_query: Query<&mut Money, (Without<ChiloutAction>, With<Location>)>,
    time: Res<Time>,
) {
    for (entity, mut money, state, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());

        let Some(town) = &state.inside_city else {
            continue
        };

        let Ok(mut town_money) = town_query.get_mut(*town)  else {
            continue
        };

        if timer.0.just_finished() {
            commands.entity(entity).remove::<ChiloutAction>();
            money.0 = (money.0 - RENT_COST_MONTHLY as i64).max(0);
            town_money.0 += RENT_COST_MONTHLY as i64;
        }
    }
}