mod main_menu;

pub use main_menu::*;
use bevy::prelude::*;

pub struct RoutesPlugin;
impl Plugin for RoutesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MyRoutePlugin);
    }
}