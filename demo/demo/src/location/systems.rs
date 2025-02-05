use bevy::prelude::*;
use crate::{needs::Needs, storage::{ItemContainer, Storage}};

use super::components::*;

pub fn calculate_needs(mut query: Query<(&Location, &mut Needs)>) {
    for (location, mut needs) in query.iter_mut() {
        for (item, consumption) in location.consumption.iter() {
            let surplus_factor =  match location.surplus_factor.get(item) {
                Some(i) => *i,
                None => 1.0
            };

            let value = location.population as f32 * consumption * surplus_factor;
            needs.set(item,value.round() as i32);
        }
    }
}

pub fn consume_goods(mut query: Query<(&Location, &mut Storage)>) {
    for (location, mut store) in query.iter_mut() {
        for (item, consumption) in location.consumption.iter() {
            let value = location.population as f32 * consumption;

            store.remove(item, value.round() as i32, true);
        }
    }
}

pub fn produce_goods(mut query: Query<(&Location, &mut Storage)>) {
    for (location, mut store) in query.iter_mut() {
        for (item, production) in location.production.iter() {
            store.add(item, *production);
        }
    }
}
