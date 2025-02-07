use std::sync::Arc;
use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use num_traits::float::Float;
use rand::prelude::*;
use crate::ai_driver::{AiDriver, AiDriverDestination};
use crate::driver::Fuel;
use crate::local_economy::LocalEconomy;
use crate::location::{Location, Money};
use crate::memory::{LocationData, Memory};
use crate::storage::{ItemContainer, Storage};

const RENT_COST_MONTHLY: i64 = 100;
const FUEL_RESERVE: f64 = 30.0;

#[derive(Component, Clone, Default, ActionComponent)]
pub struct GoToNearCityAction;

#[derive(Component, Clone, Default, ActionComponent)]
pub struct DiscoverAction;

#[derive(Component, Clone, Default, ActionComponent)]
pub struct ExitCityAction;

#[derive(Component, Clone, Default, ActionComponent)]
pub struct RefuelAction;

#[derive(Component, Default, Clone, ActionComponent)]
pub struct EarnMoneyAction;

#[derive(Component)]
pub struct WorkTimer(Timer);

#[derive(Component, Clone, DatumComponent)]
pub struct InsideCityDatum(pub bool);

#[derive(Component, Clone)]
pub struct InsideCity(pub Entity);

#[derive(Component, Clone, DatumComponent)]
pub struct FuelCost(pub i64);

#[derive(Component, Clone, DatumComponent)]
pub struct KnowAnyLocation(pub bool);

#[derive(Component, Clone, DatumComponent)]
pub struct KnowAllLocations(pub bool);

impl Default for WorkTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Once))
    }
}

impl Money {
    pub fn is_more_than<T: DatumKey>() -> (String, Arc<dyn Fn(&LocalState) -> Compare + Send + Sync>) {
        (Money::key(), Arc::new(move |state: &LocalState| {
            let value = state.data.get(&T::key()).expect(format!("No {:?} in the state", T::key()).as_str());

            Compare::GreaterThanEquals(*value)
        }))
    }
}

pub fn setup_driver_ai(query: Query<(Entity, &Money, &Fuel), Added<AiDriver>>, mut commands: Commands) {
    for (entity, money, fuel) in query.iter() {
        let rent_goal = Goal::from_reqs(&[Money::is_more(RENT_COST_MONTHLY)]).with_priority(3);
        let fuel_goal = Goal::from_reqs(&[Fuel::is_more(FUEL_RESERVE)]).with_priority(5);
        let discover_goal = Goal::from_reqs(&[KnowAllLocations::is(true)]).with_priority(0);

        let refuel_action = RefuelAction::new()
            .add_mutator(Fuel::increase(1.0))
            .add_precondition(InsideCityDatum::is(true))
            .add_dynamic_precondition(Money::is_more_than::<FuelCost>())
            .set_cost(1);

        let go_to_near_city = GoToNearCityAction::new()
            .add_mutator(InsideCityDatum::set(true))
            .add_precondition(InsideCityDatum::is(false))
            .add_precondition(KnowAnyLocation::is(true))
            .set_cost(2);

        let exit_city_action = ExitCityAction::new()
            .add_mutator(InsideCityDatum::set(false))
            .add_precondition(InsideCityDatum::is(true))
            .set_cost(1);

        let discover_action = DiscoverAction::new()
            .add_mutator(KnowAllLocations::set(true))
            .add_mutator(KnowAnyLocation::set(true))
            .add_precondition(KnowAllLocations::is(false))
            .add_precondition(InsideCityDatum::is(false))
            .set_cost(10);

        let earn_money = EarnMoneyAction::new()
            .add_mutator(Money::increase(50))
            .add_precondition(InsideCityDatum::is(true))
            .set_cost(9);

        let (planner, components) = create_planner!({
            actions: [
                (RefuelAction, refuel_action),
                (GoToNearCityAction, go_to_near_city),
                (ExitCityAction, exit_city_action),
                (DiscoverAction, discover_action),
                (EarnMoneyAction, earn_money),
            ],
            state: [money, fuel, KnowAnyLocation(false), KnowAllLocations(false), FuelCost(10), InsideCityDatum(false)],
            goals: [discover_goal, rent_goal, fuel_goal],
        });

        commands.entity(entity).insert((planner, components));
    }
}

pub fn handle_go_to_near_city_action(
    mut commands: Commands,
    query: Query<(Entity, &Memory, &Transform), (With<AiDriver>, Without<AiDriverDestination>, With<GoToNearCityAction>)>
) {
    for (entity, memory, transform) in query.iter() {
        let Some((location, _)) = find_nearest_location(memory, &transform.translation) else {
            continue;
        };

        commands.entity(entity).insert(AiDriverDestination(location.position.xy()));
    }
}

pub fn handle_exit_city_action(
    mut commands: Commands,
    mut query: Query<(Entity, &mut InsideCityDatum), (With<AiDriver>, With<ExitCityAction>)>,
) {
    for (entity, mut inside_city) in query.iter_mut() {
        inside_city.0 = false;
        commands.entity(entity).remove::<InsideCity>();
        commands.entity(entity).remove::<ExitCityAction>();
    }
}

pub fn handle_go_to_near_city_action_finish(
    trigger: Trigger<OnRemove, AiDriverDestination>,
    mut query: Query<(&Memory, &Transform, &mut InsideCityDatum), (With<AiDriver>, With<GoToNearCityAction>)>,
    mut commands: Commands
) {
    let Ok((memory, transform, mut inside_city)) = query.get_mut(trigger.entity()) else {
        return
    };

    let Some((location, city)) = find_nearest_location(memory, &transform.translation) else {
        return;
    };

    if location.position.xy().distance(transform.translation.xy()) <= 0.5 {
        inside_city.0 = true;
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
    mut query: Query<(&Memory, &mut KnowAnyLocation, &mut KnowAllLocations), (With<AiDriver>, With<DiscoverAction>)>,
    locations: Query<&Transform, With<Location>>,
    mut commands: Commands
) {
    let Ok((memory, mut know_any_location, mut know_all_locations)) = query.get_mut(trigger.entity()) else {
        return
    };

    let know_locations = memory.locations.iter().count();

    if know_locations > 0 {
        know_any_location.0 = true;
    }

    if know_locations >= locations.iter().count() {
        know_all_locations.0 = true;
        commands.entity(trigger.entity()).remove::<DiscoverAction>();
        return;
    }
    else {
        know_all_locations.0 = false;

        let Some(target) = locations.iter().choose(&mut thread_rng()) else {
            return
        };

        commands.entity(trigger.entity()).insert(AiDriverDestination(target.translation.xy()));
    }
}

fn find_nearest_location<'t>(memory: &'t Memory, position: &Vec3) -> Option<(&'t LocationData, Entity)> {
    let mut result = None;
    let mut smallest_distance = f32::infinity();

    for (entity, location) in memory.locations.iter() {
        let distance = location.value.position.distance(*position);
        
        if distance < smallest_distance {
            smallest_distance = distance;
            result = Some((&location.value, *entity));
        }
    }

    result
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

pub fn update_fuel_cost(mut query: Query<(&Memory, &Transform, &mut FuelCost), (With<AiDriver>, Changed<Memory>)>) {
    for (memory, transform, mut cost) in query.iter_mut() {
        let Some((location, _)) = find_nearest_location(memory, &transform.translation) else {
            return;
        };

        cost.0 = location.prices.sell_price("fuel") as i64;
    }
}