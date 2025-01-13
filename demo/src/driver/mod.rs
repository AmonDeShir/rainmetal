mod components;
mod systems;

use bevy::prelude::*;
pub use components::*;
use systems::*;

pub struct DriverPlugin;

impl Plugin for DriverPlugin {
    fn build(&self, app: &mut App) {}
}
