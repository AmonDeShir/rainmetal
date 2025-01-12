mod components;
mod systems;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub use components::*;
use systems::calculate_prices;

pub struct LocalEconomyPlugin;

impl Plugin for LocalEconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, calculate_prices.run_if(on_timer(Duration::from_secs(2))));
    }
}