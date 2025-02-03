mod components;
mod systems;
mod goap;

use bevy::prelude::*;
pub use components::*;
use systems::{force_ai_travel, travel_to_destination};

pub const SPEED: f32 = 1.0;
pub const ROTATION_SPEED: f32 = 1.0;
pub const POINT_TO_KM: f64 = 0.01;
pub const FUEL_CONSUMPTION_PER_KILOMETER: f64 = 6.4;


pub struct AiDriverPlugin;

impl Plugin for AiDriverPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(force_ai_travel);
        app.add_systems(Update, travel_to_destination);
    }
}
