use bevy::prelude::*;
use crate::location::Location;

#[derive(Component)]
#[require(Location)]
pub struct Village;