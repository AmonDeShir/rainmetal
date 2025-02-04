mod components;
mod systems;
mod inspector;

use bevy::prelude::*;

pub use components::*;

pub struct AiTraderPlugin;

impl Plugin for AiTraderPlugin {
    fn build(&self, app: &mut App) {

    }
}