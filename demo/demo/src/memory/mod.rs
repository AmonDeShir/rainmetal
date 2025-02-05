mod components;
mod systems;
mod inspector;

use bevy::prelude::*;

pub use components::*;
use crate::memory::inspector::ui_show_memory;
use crate::memory::systems::{init_city_memory, init_driver_position_memory, share_memory_on_enter, share_memory_on_exit, update_driver_position_memory, update_location_economy_memory, update_location_storage_memory};

pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_memory);
        app.add_systems(Update, init_city_memory);
        app.add_systems(Update, init_driver_position_memory);
        app.add_systems(Update, update_location_economy_memory);
        app.add_systems(Update, update_location_storage_memory);
        app.add_systems(Update, update_driver_position_memory);

        app.add_observer(share_memory_on_enter);
        app.add_observer(share_memory_on_exit);

    }
}