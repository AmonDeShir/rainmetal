use crate::{needs::Needs, storage::{ItemContainer, ItemList, ItemListHandle, Storage}};

use super::*;


pub fn calculate_prices(mut query: Query<(&mut LocalEconomy, &Storage, &Needs)>, items_handler: Res<ItemListHandle>, assets: Res<Assets<ItemList>>) {
    let Some(ItemList { items }) = assets.get(&items_handler.0) else {
        return;
    };

    for (item, item_data) in items.iter() {
        for mut location in query.iter_mut() {
            let storage = location.1;
            let needs = location.2;

            let demand = needs.quantity(item) as f32;
            let supply = storage.quantity(item) as f32;

            let buy_price = calculate_price(item_data.price as f32, demand, supply, item_data.price_unstability).round().max(0.0) as i32;
            let sell_price = (buy_price as f32 * 1.2).round() as i32;

            location.0.buy_price.insert(item.to_string(), buy_price);
            location.0.sell_price.insert(item.to_string(), sell_price);
        }
    }
}

pub fn calculate_price(basis_price: f32, demand: f32, supply: f32,  price_unstability: f32) -> f32 {
    return basis_price * (1.0 + price_unstability * (1.0 + demand / supply.max(1.0) ).ln());
}