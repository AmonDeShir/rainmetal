use bevy::prelude::*;
use crate::storage::Storage;
use crate::local_economy::LocalEconomy;

#[derive(Component)]
#[require(Name, Storage, LocalEconomy)]
pub struct Location {
    pub population: i32,
}

impl Location {
    pub fn new(population: i32) -> Location {
        Location {
            population
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location::new( 0)
    }
}