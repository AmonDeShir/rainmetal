mod icon;
mod rotate;
mod tab_button;
mod tabs;

use bevy::prelude::*;
pub use icon::*;
pub use rotate::*;
pub use tab_button::*;
pub use tabs::*;

pub struct ComponentsPlugin;
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(IconPlugin)
            .add_plugins(RotatePlugin)
            .add_plugins(TabButtonPlugin)
            .add_plugins(TabsPlugin);
    }
}