use bevy::prelude::*;
use bevy::utils::HashMap;
use inspector::ui_show_picked_location;
use crate::inventory::Inventory;

mod systems;
mod inspector;

#[derive(Default)]
pub struct Market {
    pub storage: Inventory,
    pub prices: HashMap<String, i32>
}

#[derive(Component)]
pub struct Location {
    pub name: String,
    pub storage: Inventory,
    pub market: Market,
    pub population: i32,
}

impl Location {
    pub fn new(name: &str, population: i32) -> Location {
        Location {
            name: name.to_string(),
            storage: Inventory::default(),
            market: Market::default(),
            population
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location::new("unnamed location", 0)
    }
}


pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_picked_location);
    }
}