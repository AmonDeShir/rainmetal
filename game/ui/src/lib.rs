mod systems;
mod routes;
mod components;

use bevy::prelude::*;
use bevy_lunex::UiPlugin;
use crate::components::ComponentsPlugin;
use crate::routes::RoutesPlugin;
use crate::systems::init_ui;

pub struct GameUIPlugin;

impl Plugin for  GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin);
        app.add_plugins(ComponentsPlugin);
        app.add_plugins(RoutesPlugin);

        app.add_systems(Startup, init_ui);
    }
}