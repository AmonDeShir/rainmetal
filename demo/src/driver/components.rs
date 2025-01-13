use crate::storage::Storage;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Storage)]
pub struct Driver;
