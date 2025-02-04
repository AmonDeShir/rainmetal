use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::ai_driver::{FUEL_CONSUMPTION_PER_KILOMETER, POINT_TO_KM};
use crate::local_economy::LocalEconomy;
use crate::memory::Memory;

#[derive(Clone)]
pub struct TradingPlan {
    pub from: String,
    pub to: String,
    pub item: String,
    pub distance: f32,
    pub start_position: Vec3,
    pub profit: f32,
    pub min_count: u32,
    pub capital_required: i32
}

impl TradingPlan {
    pub fn value(&self) -> f32 {
        self.profit / self.distance
    }
}

#[derive(Component, Default)]
#[require(Memory)]
pub struct TradingPlans(pub Vec<TradingPlan>);

impl TradingPlans {
    pub fn sort_by_profit_per_distance(&mut self) {
        // unwrap because, value function never returns NAN
        self.0.sort_by(|a, b| a.value().partial_cmp(&b.value()).unwrap());
    }

    pub fn find_plan(&mut self, player_position: Vec3, player_gold: i32, fuel_cost: i32) -> Option<&TradingPlan> {
        for plan in self.0.iter() {
            if plan.capital_required < player_gold {
                // Calculating travel costs is computationally complex, so we only count it if the plan is affordable
                let travel_cost = calculate_travel_cost(player_position, plan.start_position, fuel_cost).round() as i32;

                if plan.capital_required + travel_cost < player_gold {
                    return Some(&plan);
                }
            }
        }

        None
    }
}

fn calculate_travel_cost(player_position: Vec3, target_position: Vec3, fuel_cost: i32) -> f64 {
    let distance = player_position.distance(target_position) as f64;

    distance * POINT_TO_KM * FUEL_CONSUMPTION_PER_KILOMETER * fuel_cost as f64
}