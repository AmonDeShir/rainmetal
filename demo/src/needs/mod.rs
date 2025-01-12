use bevy::utils::HashMap;
use bevy::prelude::*;

use crate::storage::ItemContainer;

#[derive(Component, Reflect)]
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