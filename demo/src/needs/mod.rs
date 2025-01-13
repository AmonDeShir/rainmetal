use bevy::utils::HashMap;
use bevy::prelude::*;

use crate::storage::ItemContainer;

#[derive(Component)]
pub struct Needs {
    pub items: HashMap<String, i32>,
}

impl Default for Needs {
    fn default() -> Self {
        Needs {
            items: HashMap::new(),
        }
    }
}

impl ItemContainer for Needs {
    fn quantity(&self, name: &str) -> i32 {
        self.items.get(name).cloned().unwrap_or(0)
    }

    fn set(&mut self, name: &str, quantity: i32) {
        self.items.insert(name.to_string(), quantity);
    }


    fn add(&mut self, name: &str, quantity: i32) {
        self.set(name, self.quantity(name) + quantity);
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

    fn remove(&mut self, name: &str, quantity: i32, force: bool) -> Option<()> {
        if self.quantity(name) > 0 || force {
            self.set(name, (self.quantity(name) - quantity).max(0));

            return Some(());
        }

        None
    }
}