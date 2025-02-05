use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_dogoap::prelude::*;
use crate::storage::Storage;
use crate::local_economy::LocalEconomy;
use crate::memory::Memory;
use crate::radar::TrackedByRadar;

#[derive(Component, Clone, DatumComponent)]
pub struct Money(pub i64);

impl Default for Money {
    fn default() -> Self {
        Money(100)
    }
}

#[derive(Component)]
#[require(Name, Storage, LocalEconomy, Memory, TrackedByRadar, Money)]
pub struct Location {
    pub population: i32,
    pub production: HashMap<String, i32>,
    pub consumption: HashMap<String, f32>,
    pub surplus_factor: HashMap<String, f32>,
}

impl Location {
    pub fn new(population: i32) -> Location {
        Location {
            population,
            production: HashMap::new(),
            consumption: HashMap::new(),
            surplus_factor: HashMap::new(),
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location::new( 0)
    }
}