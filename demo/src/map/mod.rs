mod assets;
mod systems;

use systems::*;
use assets::*;
use bevy::prelude::*;
use ron_asset_manager::prelude::*;

#[derive(Component)]
pub struct Map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapDataHandle>();

        app.add_plugins(RonAssetPlugin::<MapData>::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, load_map);
    }
}