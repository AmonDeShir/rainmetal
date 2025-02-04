mod components;
mod systems;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub use components::*;
use systems::calculate_prices;
use crate::local_economy::systems::update_self_economy_memory;

pub struct LocalEconomyPlugin;

impl Plugin for LocalEconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (calculate_prices, update_self_economy_memory).chain().run_if(on_timer(Duration::from_secs(2))));
    }
}