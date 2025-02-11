mod inspector;

use std::hash::{Hash, Hasher};
use bevy::prelude::*;
use bevy::utils::HashMap;
use inspector::{ui_init, ui_show_items};
use ron_asset_manager::prelude::*;
use serde::Deserialize;


#[derive(RonAsset, Deserialize, Debug)]
pub struct ItemDefinition {
    pub name: String,
    pub price: i32,
    pub price_unstability: f32,
}

#[derive(Asset, TypePath, RonAsset, Deserialize)]
pub struct ItemList {
    pub items: HashMap<String, ItemDefinition>,
}

#[derive(Resource, Default)]
pub struct ItemListHandle(pub Handle<ItemList>);

#[derive(Component, Clone, Eq, PartialEq, Debug)]
pub struct Storage {
    pub items: HashMap<String, i32>,
}

impl Hash for Storage  {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.items.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
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
    fn add(&mut self, name: &str, quantity: i32);
    fn remove(&mut self, name: &str, quantity: i32, force: bool) -> Option<()>;
    fn set(&mut self, name: &str, quantity: i32);
    fn add_one(&mut self, name: &str);
    fn remove_one(&mut self, name: &str) -> Option<()>;
}

impl ItemContainer for Storage {
    fn quantity(&self, name: &str) -> i32 {
        self.items.get(name).cloned().unwrap_or(0)
    }

    fn add(&mut self, name: &str, quantity: i32) {
        self.set(name, self.quantity(name) + quantity);
    }


    fn remove(&mut self, name: &str, quantity: i32, force: bool) -> Option<()> {
        if self.quantity(name) > 0 || force {
            self.set(name, (self.quantity(name) - quantity).max(0));

            return Some(());
        }

        None
    }

    fn set(&mut self, name: &str, quantity: i32) {
        self.items.insert(name.to_string(), quantity);
    }

    fn add_one(&mut self, name: &str) {
        self.add(name, 1);
    }

    fn remove_one(&mut self, name: &str) -> Option<()> {
        if self.quantity(name) > 0 {
            self.add(name, -1);

            return Some(());
        }

        None
    }
}

impl Storage {
    pub fn find_most_common(&self) -> Option<String> {
        let mut most_common = None;
        let mut most_common_count = 0;

        for (item, count) in self.items.iter() {
            if count > &most_common_count {
                most_common_count = *count;
                most_common = Some(item.clone());
            }
        }

        most_common
    }
}

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ItemList>::default());
        app.init_resource::<ItemListHandle>();
        app.add_systems(Startup, setup);
        app.add_systems(Startup, ui_init);
        app.add_systems(Update, ui_show_items);
    }
}

fn setup(server: Res<AssetServer>, mut items: ResMut<ItemListHandle>) {
    items.0 = server.load("items.ron");
}