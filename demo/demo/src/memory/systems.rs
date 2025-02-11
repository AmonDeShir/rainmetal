use super::*;
use crate::radar::{EnterRadioTransmissionRadius, ExitRadioTransmissionRadius};
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::hash::Hash;
use crate::location::{Location, Money};
use crate::ai_driver::AiDriverDestination;
use crate::driver::Driver;
use crate::local_economy::LocalEconomy;
use crate::storage::Storage;

pub fn on_location_removed(trigger: Trigger<OnRemove, Location>, mut query: Query<&mut Memory>) {
    for mut memory in query.iter_mut() {
        memory.locations.remove(&trigger.entity());
    }
}

pub fn on_driver_removed(trigger: Trigger<OnRemove, Driver>, mut query: Query<&mut Memory>) {
    for mut memory in query.iter_mut() {
        memory.characters.remove(&trigger.entity());
    }
}


fn share_memory_map<K: Hash + Eq + Clone, V: Clone>(target: &mut HashMap<K, Memo<V>>, source: &HashMap<K, Memo<V>>) {
    for (key, memory) in source.iter() {
        let target_memo = target.get(key);

        match target_memo {
            Some(target_memory) => {
                if target_memory.is_newer_than(memory) {
                    target.insert(key.clone(), memory.clone());
                }
            }

            None => {
                target.insert(key.clone(), memory.clone());
            }
        };
    }
}

pub fn share_memory_on_enter(trigger: Trigger<EnterRadioTransmissionRadius>, mut query: Query<&mut Memory>) {
    let Ok([mut target, source]) = query.get_many_mut([trigger.entity(), trigger.0]) else {
        return
    };

    share_memory_map(&mut target.locations, &source.locations);
    share_memory_map(&mut target.characters, &source.characters);
}

pub fn share_memory_on_exit(trigger: Trigger<ExitRadioTransmissionRadius>, mut query: Query<&mut Memory>) {
    let Ok([mut target, source]) = query.get_many_mut([trigger.entity(), trigger.0]) else {
        return
    };

    share_memory_map(&mut target.locations, &source.locations);
    share_memory_map(&mut target.characters, &source.characters);
}

pub fn init_city_memory(mut query: Query<(&mut Memory, Entity, &Money, &LocalEconomy, &Storage, &Transform), (With<Location>, Added<Memory>)>, time: Res<Time>) {
    for (mut memory, entity, money, economy, storage, transform) in query.iter_mut() {
        memory.locations.insert(
            entity.clone(),
            Memo::new(LocationData {
                storage: storage.clone(),
                position: transform.translation.clone(),
                prices: economy.clone(),
                money: money.0 as u64,
            }, time.elapsed_secs())
        );
    }
}

pub fn update_location_money_memory(mut query: Query<(&mut Memory, Entity, &Money), (With<Location>, Changed<Money>)>, time: Res<Time>) {
    for (mut memory, entity, money) in query.iter_mut() {
        let Some(memo) = memory.locations.get_mut(&entity) else {
            continue
        };

        memo.time = time.elapsed_secs();
        memo.value.money = money.0 as u64;
    }
}

pub fn update_location_economy_memory(mut query: Query<(&mut Memory, Entity, &LocalEconomy), (With<Location>, Changed<LocalEconomy>)>, time: Res<Time>) {
    for (mut memory, entity, economy) in query.iter_mut() {
        let Some(memo) = memory.locations.get_mut(&entity) else {
            continue
        };

        memo.time = time.elapsed_secs();
        memo.value.prices = economy.clone();
    }
}

pub fn update_location_storage_memory(mut query: Query<(&mut Memory, Entity, &Storage), (With<Location>, Changed<Storage>)>, time: Res<Time>) {
    for (mut memory, entity, storage) in query.iter_mut() {
        let Some(memo) = memory.locations.get_mut(&entity) else {
            continue
        };

        memo.time = time.elapsed_secs();
        memo.value.storage = storage.clone();
    }
}

pub fn init_driver_position_memory(mut query: Query<(&mut Memory, Entity, &Transform, Option<&AiDriverDestination>), (With<Driver>, Added<Memory>)>, time: Res<Time>) {
    for (mut memory, entity, transform, destination) in query.iter_mut() {
        let destination = match destination {
            Some(pos) => Some(Vec3::new(pos.0.x, pos.0.y, 0.0)),
            None => None
        };

        memory.characters.insert(
            entity.clone(),
            Memo::new(CharacterData {
                current_position: transform.translation,
                destination,
            }, time.elapsed_secs())
        );
    }
}

pub fn update_driver_position_memory(mut query: Query<(&mut Memory, Entity, &Transform, Option<&AiDriverDestination>), (With<Driver>, Added<Memory>)>, time: Res<Time>) {
    for (mut memory, entity, transform, destination) in query.iter_mut() {
        let Some(memo) = memory.characters.get_mut(&entity) else {
            continue
        };

        let destination = match destination {
            Some(pos) => Some(Vec3::new(pos.0.x, pos.0.y, 0.0)),
            None => None
        };

        memo.time = time.elapsed_secs();
        memo.value.current_position = transform.translation.clone();
        memo.value.destination = destination;
    }
}