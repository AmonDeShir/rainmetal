mod inspector;
mod components;
mod systems;

use bevy::prelude::*;
use inspector::ui_show_picked_location;
use crate::storage::Storage;
use crate::local_economy::LocalEconomy;
pub use self::components::*;


pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_picked_location);
    }
}