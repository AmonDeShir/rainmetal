use bevy::prelude::*;
use crate::ai_driver::{FUEL_CONSUMPTION_PER_KILOMETER, POINT_TO_KM};
use crate::ai_trader::components::{AiTrader, TradingPlan, TradingPlans};
use crate::location::Location;
use crate::memory::Memory;

pub fn update_trading_plans(mut query: Query<(&mut TradingPlans, &Memory), With<AiTrader>>, cities: Query<&Transform, With<Location>>) {
    for (mut plans, memory) in query.iter_mut() {
        plans.0.clear();

        for (start_city, start_city_data) in memory.locations.iter() {
            for (item, buy_price) in start_city_data.value.prices.sell_price.iter() {
                for (end_city, end_city_data) in memory.locations.iter() {
                    if start_city == end_city {
                        continue
                    };

                    let Some(fuel_price) = start_city_data.value.prices.sell_price.get("fuel") else {
                        continue
                    };

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

                    let distance = start_position.translation.distance(end_position.translation);
                    let travel_cost = calculate_travel_cost(distance as f64, *fuel_price);
                    let min_count = calculate_minimal_count(travel_cost, *buy_price, *sell_price);
                    let capital_required = (travel_cost as f64 + *buy_price as f64) * min_count as f64;

                    plans.0.push(TradingPlan {
                        from: start_city.clone(),
                        to: end_city.clone(),
                        item: item.to_string(),
                        start_position: start_position.translation,
                        profit: (sell_price - buy_price) as f32,
                        min_capital_required: capital_required.round() as i32,
                        distance,
                        min_count,
                    });
                }
            }
        }

        plans.sort_by_profit_per_distance();
    }
}

pub fn calculate_travel_cost(distance: f64, fuel_price: i32) -> i32 {
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