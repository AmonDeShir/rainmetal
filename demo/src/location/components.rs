use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::storage::Storage;
use crate::local_economy::LocalEconomy;

#[derive(Component)]
#[require(Name, Storage, LocalEconomy)]
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