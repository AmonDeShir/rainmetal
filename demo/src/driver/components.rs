use crate::storage::Storage;
use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use crate::radar::TrackedByRadar;
use crate::memory::Memory;

#[derive(Component, Reflect, Clone, DatumComponent)]
pub struct Fuel(pub f64);

impl Default for Fuel {
    fn default() -> Self {
        Self(100.0)
    }
}

#[derive(Component, Default)]
#[require(Storage, Fuel, TrackedByRadar, Memory)]
pub struct Driver;

