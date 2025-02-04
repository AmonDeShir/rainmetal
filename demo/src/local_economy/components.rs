use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::storage::Storage;
use crate::needs::Needs;

#[derive(Component, Default, Clone)]
#[require(Storage, Needs)]
pub struct LocalEconomy {
    pub sell_price: HashMap<String, i32>,
    pub buy_price: HashMap<String, i32>,
}

impl LocalEconomy {
    fn buy_price(&self, item: &str) -> i32 {
        self.buy_price.get(item).cloned().unwrap_or(0)
    }

    fn sell_price(&self, item: &str) -> i32 {
        self.sell_price.get(item).cloned().unwrap_or(0)
    }
}
