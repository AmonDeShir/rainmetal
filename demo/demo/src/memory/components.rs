use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::local_economy::LocalEconomy;
use crate::radar::TrackedByRadar;
use crate::storage::Storage;
use num_traits::float::Float;

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
}

#[derive(Clone)]
pub struct CharacterData {
    pub current_position: Vec3,
    pub destination: Option<Vec3>,
}

#[derive(Clone)]
pub struct LocationData {
    pub prices: LocalEconomy,
    pub position: Vec3,
    pub storage: Storage,
}

#[derive(Component, Default)]
#[require(TrackedByRadar)]
pub struct Memory {
    pub locations: HashMap<Entity, Memo<LocationData>>,
    pub characters: HashMap<Entity, Memo<CharacterData>>,
}

impl Memory {
    pub fn nearest_location_with(&self, position: &Vec3, condition: Box<dyn Fn(&LocationData) -> bool>) -> Option<(&LocationData, Entity)> {
        let mut result = None;
        let mut smallest_distance = f32::infinity();

        for (entity, location) in self.locations.iter() {
            let distance = location.value.position.distance(*position);

            if distance < smallest_distance && condition(&location.value) {
                smallest_distance = distance;
                result = Some((&location.value, *entity));
            }
        }

        result
    }

    pub fn nearest_location(&self, position: &Vec3) -> Option<(&LocationData, Entity)> {
        self.nearest_location_with(position, Box::new(|_| true))
    }
}