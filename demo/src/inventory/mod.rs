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
}

impl ItemDefinition {
    pub fn new(name: &str, price: i32) -> ItemDefinition {
        ItemDefinition  {
            name: name.to_string(),
            price,
        }
    }
}

#[derive(Asset, TypePath, RonAsset, Deserialize)]
pub struct ItemList {
    pub items: HashMap<String, ItemDefinition>,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
struct ItemListHandle(Handle<ItemList>);

#[derive(Component, Reflect)]
pub struct Inventory {
    pub items: HashMap<String, i32>,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }
}

impl Inventory {
    pub fn quantity(&self, name: &str) -> i32 {
        self.items.get(name).cloned().unwrap_or(0)
    }

    pub fn add(&mut self, name: &str) {
        self.items.insert(name.to_string(), self.quantity(name) + 1);
    }

    pub fn remove(&mut self, name: &str) -> Option<()> {
        if self.quantity(name) > 0 {
            self.items.remove(name);
            return Some(());
        }

        None
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ItemList>::default());
        app.init_resource::<ItemListHandle>();
        app.add_systems(Startup, setup);
        app.add_systems(Startup, ui_init);
        app.register_type::<Inventory>();
        app.register_type::<ItemListHandle>();
        app.add_systems(Update, ui_show_items);
    }
}

fn setup(server: Res<AssetServer>, mut items: ResMut<ItemListHandle>) {
    items.0 = server.load("items.ron");
}