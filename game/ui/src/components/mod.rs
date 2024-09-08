mod icon;
mod rotate;

use bevy::prelude::*;
pub use icon::*;
pub use rotate::*;

pub struct ComponentsPlugin;
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CustomButtonPlugin)
            .add_plugins(RotatePlugin);
    }
}