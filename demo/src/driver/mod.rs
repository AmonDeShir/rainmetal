mod components;
mod inspector;
mod systems;

use bevy::prelude::*;
pub use components::*;
use inspector::ui_show_picked_driver;

pub struct DriverPlugin;

impl Plugin for DriverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_picked_driver);
    }
}
