use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::local_economy::LocalEconomy;
use crate::radar::TrackedByRadar;

#[derive(Clone)]
pub struct Memo<T> where T: Clone {
    pub value: T,
    pub time: f32,
}

impl<T> Memo<T> where T: Clone {
    pub fn is_newer_than(&self, memory: &Memo<T>) -> bool {
        self.time <= memory.time
    }

    pub fn new(value: T, time: f32) -> Self {
        Self { value, time }
    }

    pub fn update(&mut self, memory: &Memo<T>) {
        if self.is_newer_than(memory) {
            self.time = memory.time;
            self.value = memory.value.clone();
        }
    }
}

#[derive(Clone)]
pub struct TravelPlan {
    pub current_position: Vec3,
    pub destination: Option<Vec3>,
}

#[derive(Component, Default)]
#[require(TrackedByRadar)]
pub struct Memory {
    pub city_prices: HashMap<String, Memo<LocalEconomy>>,
    pub npc_positions: HashMap<Entity, Memo<TravelPlan>>,
}
