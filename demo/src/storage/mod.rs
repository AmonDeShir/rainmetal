mod inspector;

use bevy::prelude::*;
use bevy::utils::HashMap;
use inspector::{ui_init, ui_show_items};
use ron_asset_manager::prelude::*;
use serde::Deserialize;


#[derive(RonAsset, Deserialize, Reflect, Debug)]
pub struct ItemDefinition {
    pub name: String,
    pub price: i32,
    pub price_unstability: f32,
}

#[derive(Asset, TypePath, RonAsset, Deserialize)]
pub struct ItemList {
    pub items: HashMap<String, ItemDefinition>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ItemListHandle(pub Handle<ItemList>);

#[derive(Component, Reflect)]
pub struct Storage {
    pub items: HashMap<String, i32>,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            items: HashMap::new(),
        }
    }
}

pub trait ItemContainer {
    fn quantity(&self, name: &str) -> i32;
    fn add(&mut self, name: &str);
    fn remove(&mut self, name: &str) -> Option<()>;
}

impl ItemContainer for Storage {
    fn quantity(&self, name: &str) -> i32 {
        self.items.get(name).cloned().unwrap_or(0)
    }

    fn add(&mut self, name: &str) {
        self.items.insert(name.to_string(), self.quantity(name) + 1);
    }

    fn remove(&mut self, name: &str) -> Option<()> {
        if self.quantity(name) > 0 {
            self.items.remove(name);
            return Some(());
        }

        None
    }
}

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ItemList>::default());
        app.init_resource::<ItemListHandle>();
        app.add_systems(Startup, setup);
        app.add_systems(Startup, ui_init);
        app.register_type::<Storage>();
        app.register_type::<ItemListHandle>();
        app.add_systems(Update, ui_show_items);
    }
}

fn setup(server: Res<AssetServer>, mut items: ResMut<ItemListHandle>) {
    items.0 = server.load("items.ron");
}