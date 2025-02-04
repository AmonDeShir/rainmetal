mod components;
mod systems;
mod inspector;

use bevy::prelude::*;

pub use components::*;
use crate::memory::inspector::ui_show_memory;
use crate::memory::systems::{share_memory_on_enter, share_memory_on_exit};

pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_memory);
        app.add_observer(share_memory_on_enter);
        app.add_observer(share_memory_on_exit);

    }
}