mod components;
mod systems;
mod goap;

use crate::ai_driver::goap::setup_driver_ai;
use crate::ai_driver::goap::*;
use crate::ai_driver::systems::travel_to_destination;
use bevy::prelude::*;
pub use components::*;

pub use self::goap::CurrentTradingData;

pub const SPEED: f32 = 1.0;
pub const POINT_TO_KM: f64 = 0.01;
pub const FUEL_CONSUMPTION_PER_KILOMETER: f64 = 0.06;
pub const RENT_COST_MONTHLY: u64 = 300;
pub const FUEL_RESERVE: f64 = 10.0;
pub const SMALL_TRADING_PLAN_VALUE: u64 = 30;
pub const MEDIUM_TRADING_PLAN_VALUE: u64 = 150;
pub const BIG_TRADING_PLAN_VALUE: u64 = 500;
pub const HUGE_TRADING_PLAN_VALUE: u64 = 1000;

pub struct AiDriverPlugin;

impl Plugin for AiDriverPlugin {
    fn build(&self, mut app: &mut App) {
        setup_planner_systems(&mut app);

        app.add_systems(Update, travel_to_destination);
        app.add_systems(Update, setup_driver_ai);

        app.add_systems(Update, update_state_memory);
        app.add_systems(Update, update_state_fuel);
        app.add_systems(Update, update_state_money);
        app.add_systems(Update, update_state_trading_plans);


        app.add_systems(Update, handle_refuel_action);
        app.add_systems(Update, handle_set_destination_to_fuel_station_action);

        app.add_systems(Update, handle_go_to_destination_action);
        app.add_observer(handle_go_to_destination_action_finish);
        app.add_systems(Update, handle_exit_city_action);
        app.add_systems(Update, handle_discover_action);
        app.add_observer(handle_discover_action_finish);

        app.add_systems(Update, handle_find_trade_plans_action);
        app.add_systems(Update, handle_update_trading_path_action);
        app.add_observer(handle_update_trading_path_action_finish);

        app.add_systems(Update, handle_remove_trading_goal_action);
        app.add_systems(Update, handle_set_trading_goal_to_small_action);
        app.add_systems(Update, handle_set_trading_goal_to_medium_action);
        app.add_systems(Update, handle_set_trading_goal_to_big_action);
        app.add_systems(Update, handle_set_trading_goal_to_huge_action);

        app.add_systems(Update, (handle_init_chillout_action, handle_chillout_action).chain());

        app.add_systems(Update, (
            handle_init_phase_of_do_trading_action,
            handle_buy_phase_of_do_trading_action,
            handle_sell_phase_of_do_trading_action
        ).chain());
    }
}