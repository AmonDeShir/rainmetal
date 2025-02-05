mod components;
mod systems;
mod inspector;

use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use crate::ai_trader::inspector::ui_show_trading_plans;
use crate::ai_trader::systems::update_trading_plans;

pub use self::components::*;

pub struct AiTraderPlugin;

impl Plugin for AiTraderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_trading_plans);
        app.add_systems(Update, update_trading_plans.run_if(on_timer(Duration::from_secs(10))));
    }
}