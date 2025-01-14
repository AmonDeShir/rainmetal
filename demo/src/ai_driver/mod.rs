mod components;
mod systems;

use bevy::prelude::*;
pub use components::*;
use systems::{force_ai_travel, travel_to_destination};

pub struct AiDriverPlugin;

impl Plugin for AiDriverPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(force_ai_travel);
        app.add_systems(Update, travel_to_destination);
    }
}
