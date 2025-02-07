mod components;
mod systems;
mod goap;
mod inspector;

use bevy::prelude::*;
use bevy_dogoap::prelude::*;
pub use components::*;
use systems::{force_ai_travel, travel_to_destination};
use crate::ai_driver::goap::*;
use crate::ai_driver::inspector::ui_show_ai_plans;
use crate::driver::Fuel;
use crate::location::Money;

pub const SPEED: f32 = 1.0;
pub const POINT_TO_KM: f64 = 0.01;
pub const FUEL_CONSUMPTION_PER_KILOMETER: f64 = 0.06;

pub struct AiDriverPlugin;

impl Plugin for AiDriverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_ai_plans);
        app.add_observer(force_ai_travel);
        app.add_systems(Update, travel_to_destination);
        app.add_systems(Update, setup_driver_ai);
        app.add_systems(Update, handle_go_to_near_city_action);
        app.add_observer(handle_go_to_near_city_action_finish);
        app.add_systems(Update, handle_refuel_action);
        app.add_systems(Update, handle_exit_city_action);
        app.add_systems(Update, handle_discover_action);
        app.add_observer(handle_discover_action_finish);
        app.add_systems(Update, update_fuel_cost);
        app.add_observer(start_work_action);
        app.add_systems(Update, handle_work_action);

        register_components!(app, vec![Fuel, Money, InsideCityDatum, KnowAllLocations, FuelCost, KnowAnyLocation]);
        register_actions!(app, vec![GoToNearCityAction, ExitCityAction, EarnMoneyAction, RefuelAction, DiscoverAction]);
    }
}
