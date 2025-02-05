use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::storage::Storage;
use crate::needs::Needs;

#[derive(Component, Default, Clone)]
#[require(Storage, Needs)]
pub struct LocalEconomy {
    /** Prices at witch the entity is willing to sell items */
    pub sell_price: HashMap<String, i32>,
    /** Prices at which the entity is willing to buy an items  */
    pub buy_price: HashMap<String, i32>,
}

impl LocalEconomy {
    /** Gets the price at witch the entity is willing to sell the item, returns zero if entity doesn't want to sell the item */
    pub fn buy_price(&self, item: &str) -> i32 {
        self.buy_price.get(item).cloned().unwrap_or(0)
    }

    /** Gets the price at witch the entity is willing to buy the item, returns zero if entity doesn't want to buy the item */
    pub fn sell_price(&self, item: &str) -> i32 {
        self.sell_price.get(item).cloned().unwrap_or(0)
    }
}
