mod components;
mod systems;

use bevy::prelude::*;

pub use components::*;
use systems::calculate_prices;

pub struct LocalEconomyPlugin;

impl Plugin for LocalEconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, calculate_prices);
    }
}