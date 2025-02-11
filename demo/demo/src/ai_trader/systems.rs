use crate::ai_driver::{BIG_TRADING_PLAN_VALUE, FUEL_CONSUMPTION_PER_KILOMETER, HUGE_TRADING_PLAN_VALUE, MEDIUM_TRADING_PLAN_VALUE, POINT_TO_KM, SMALL_TRADING_PLAN_VALUE};
use crate::ai_trader::components::{AiTrader, TradingPlan, TradingPlans};
use crate::location::{Location, Money};
use crate::memory::Memory;
use bevy::prelude::*;
use crate::ai_trader::TradingPlanByValue;
use crate::driver::Fuel;
use crate::storage::{ItemContainer, Storage};

pub fn update_trading_plans(mut query: Query<(&mut TradingPlans, &Memory), With<AiTrader>>, cities: Query<&Transform, With<Location>>) {
    for (mut plans, memory) in query.iter_mut() {
        plans.0.clear();

        for (start_city, start_city_data) in memory.locations.iter() {
            for (item, buy_price) in start_city_data.value.prices.sell_price.iter() {
                for (end_city, end_city_data) in memory.locations.iter() {
                    if start_city == end_city {
                        continue
                    };

                    let Some((fuel_station, _)) = memory.nearest_location_with(&start_city_data.value.position, Box::new(|city| city.storage.quantity("fuel") > 0)) else {
                        continue
                    };

                    let fuel_price = fuel_station.prices.sell_price("fuel");

                    let Some(sell_price) = end_city_data.value.prices.buy_price.get(item) else {
                        continue
                    };

                    if sell_price <= buy_price {
                        continue
                    }

                    let Ok(start_position) = cities.get(*start_city) else {
                        continue
                    };

                    let Ok(end_position) = cities.get(*end_city) else {
                        continue
                    };

                    let distance = start_position.translation.distance(end_position.translation) as f64;
                    let travel_cost = calculate_travel_cost(distance as f64, fuel_price);
                    let min_count = calculate_minimal_count(travel_cost, *buy_price, *sell_price) as u64;
                    let capital_required = (travel_cost as f64 + *buy_price as f64) * min_count as f64;

                    if (start_city_data.value.storage.quantity(item) as u64) < min_count {
                        continue
                    }

                    if end_city_data.value.money < min_count * (*sell_price as u64) {
                        continue
                    }

                    plans.0.push(TradingPlan {
                        from: start_city.clone(),
                        to: end_city.clone(),
                        item: item.to_string(),
                        start_position: start_position.translation,
                        profit: (sell_price - buy_price) as f64,
                        capital_required: capital_required.round() as u64,
                        distance,
                        count: min_count,
                        sell_price: *sell_price as u64,
                        buy_price: *buy_price as u64,
                        last_updated: f32::min(start_city_data.time, end_city_data.time)
                    });
                }
            }
        }
    }
}

fn calculate_travel_cost(distance: f64, fuel_price: i32) -> i32 {
    (distance * POINT_TO_KM * FUEL_CONSUMPTION_PER_KILOMETER * fuel_price as f64).round() as i32
}

fn calculate_minimal_count(travel_cost: i32, buy_price: i32, sell_price: i32) -> u32 {
    let mut count: u32 = 1;

    if sell_price <= buy_price {
        return 1;
    }

    while (sell_price - buy_price) * (count as i32) < travel_cost {
        count += 1;
    }

    count
}

pub fn update_state_trading_plans(
    mut query: Query<(&mut TradingPlanByValue, &Transform, &Fuel, &Memory, &TradingPlans, &Money, &Storage), Or<(Changed<TradingPlans>, Changed<Money>)>>,
    time: Res<Time>,
) {
    for (mut plans_by_value, transform, fuel, memory, plans, money, storage) in query.iter_mut() {
        let item = storage.find_most_common();

        plans_by_value.set_all_to_none();

        let station = memory.nearest_location_with(&transform.translation, Box::new(|location| location.storage.quantity("fuel") > 0));
        let fuel_price = station.map(|(location, _)| location.prices.sell_price("fuel"));

        if let Some(item) = item {
            for plan in plans.0.iter().filter(|plan| plan.item == item) {
                if plans_by_value.all_set() {
                    break;
                }

                assign_trade_plan(&mut plans_by_value, plan, transform, fuel, &fuel_price, money, &time, storage, memory);
            }
        }

        for plan in plans.0.iter() {
            if plans_by_value.all_set() {
                break;
            }

            assign_trade_plan(&mut plans_by_value, plan, transform, fuel, &fuel_price, money, &time, storage, memory);
        }
    }
}

fn assign_trade_plan(plans_by_value: &mut TradingPlanByValue, plan: &TradingPlan, transform: &Transform, fuel: &Fuel, fuel_price: &Option<i32>, money: &Money, time: &Res<Time>, storage: &Storage, memory: &Memory) {
    if plans_by_value.small.is_none() {
        plans_by_value.small = plan_trade(plan, &transform.translation, &fuel.0, fuel_price, money, &time.delta_secs(), storage, memory, &SMALL_TRADING_PLAN_VALUE);
    }
    if plans_by_value.medium.is_none() {
        plans_by_value.medium = plan_trade(plan, &transform.translation, &fuel.0, fuel_price, money, &time.delta_secs(), storage, memory, &MEDIUM_TRADING_PLAN_VALUE);
    }
    if plans_by_value.big.is_none() {
        plans_by_value.big = plan_trade(plan, &transform.translation, &fuel.0, fuel_price, money, &time.delta_secs(), storage, memory, &BIG_TRADING_PLAN_VALUE);
    }
    if plans_by_value.huge.is_none() {
        plans_by_value.huge = plan_trade(plan, &transform.translation, &fuel.0, fuel_price, money, &time.delta_secs(), storage, memory, &HUGE_TRADING_PLAN_VALUE);
    }
}


fn plan_trade(plan: &TradingPlan, player_position: &Vec3, fuel: &f64, fuel_cost: &Option<i32>, money: &Money, current_time: &f32, storage: &Storage, memory: &Memory, goal: &u64) -> Option<TradingPlan> {
    // plan is older than 20 seconds
    if current_time - plan.last_updated > 20.0 {
        return None;
    }

    let count = ((*goal as f64) / plan.profit).ceil() as u64;
    let additional_items = (count as i64 - storage.quantity(&plan.item) as i64).max(0) as u64;
    let mut capital = additional_items * plan.buy_price;

    if capital > money.0 as u64 {
        return None;
    }

    let Some(from_location) = memory.locations.get(&plan.from) else {
        return None
    };

    let Some(to_location) = memory.locations.get(&plan.to) else {
        return None
    };


    if from_location.value.storage.quantity(&plan.item) < additional_items as i32 {
        return None;
    }

    if to_location.value.money < count * plan.sell_price {
        return None;
    }

    let fuel_consumption= calculate_fuel_consumption(*player_position, plan.start_position);

    if fuel < &fuel_consumption {
        let Some(fuel_cost) = fuel_cost else {
            return None;
        };

        let deficit = fuel_consumption - fuel;
        let additional_fuel_cost = (deficit * (*fuel_cost) as f64).round() as u64;
        capital += additional_fuel_cost;

        if capital > money.0 as u64 {
            return None;
        }
    }

    let mut selected = plan.clone();
    selected.capital_required += capital;
    selected.count = count;

    Some(selected)
}

fn calculate_fuel_consumption(player_position: Vec3, target_position: Vec3) -> f64 {
    let distance = player_position.distance(target_position) as f64;

    distance * POINT_TO_KM * FUEL_CONSUMPTION_PER_KILOMETER
}
