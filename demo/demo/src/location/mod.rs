mod inspector;
mod components;
mod systems;

use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use inspector::ui_show_picked_location;
use systems::{calculate_needs, consume_goods, produce_goods};
use crate::storage::Storage;
use crate::local_economy::LocalEconomy;
pub use self::components::*;


pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_show_picked_location);
        app.add_systems(Update, calculate_needs.run_if(on_timer(Duration::from_secs(5))));
        app.add_systems(Update, consume_goods.run_if(on_timer(Duration::from_secs(30))));
        app.add_systems(Update, produce_goods.run_if(on_timer(Duration::from_secs(30))));
    }
}