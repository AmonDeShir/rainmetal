use std::hash::Hash;
use super::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::local_economy::LocalEconomy;
use crate::radar::EnterRadioTransmissionRadius;

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

pub fn on_memory_share(trigger: Trigger<EnterRadioTransmissionRadius>, mut query: Query<&mut Memory>) {
    let Ok([mut target, source]) = query.get_many_mut([trigger.entity(), trigger.0]) else {
        return
    };

    share_memory_map(&mut target.city_prices, &source.city_prices);
    share_memory_map(&mut target.npc_positions, &source.npc_positions);
}