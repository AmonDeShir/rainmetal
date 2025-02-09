mod components;
mod systems;
mod goap;

use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
pub use components::*;
use crate::ai_driver::goap::setup_driver_ai;
use crate::ai_driver::systems::{collect_rent, force_ai_travel, travel_to_destination};
use crate::ai_driver::goap::*;

pub const SPEED: f32 = 1.0;
pub const POINT_TO_KM: f64 = 0.01;
pub const FUEL_CONSUMPTION_PER_KILOMETER: f64 = 0.06;
pub const RENT_COST_MONTHLY: i64 = 120;
pub const FUEL_RESERVE: f64 = 30.0;

pub struct AiDriverPlugin;

impl Plugin for AiDriverPlugin {
    fn build(&self, mut app: &mut App) {
        setup_planner_systems(&mut app);

        //app.add_observer(force_ai_travel);
        app.add_systems(Update, travel_to_destination);
        app.add_systems(Update, setup_driver_ai);
        app.add_systems(Update, travel_to_destination);
        app.add_systems(Update, travel_to_destination);

        app.add_systems(Update, update_state_memory);
        app.add_systems(Update, update_state_fuel);
        app.add_systems(Update, update_state_money);

        app.add_systems(Update, handle_go_to_near_city_action);
        app.add_observer(handle_go_to_near_city_action_finish);
        app.add_systems(Update, handle_refuel_action);
        app.add_systems(Update, handle_exit_city_action);
        app.add_systems(Update, handle_discover_action);
        app.add_observer(handle_discover_action_finish);
        app.add_observer(start_work_action);
        app.add_systems(Update, handle_work_action);

        app.add_systems(Update, collect_rent.run_if(on_timer(Duration::from_secs(120))));
    }
}