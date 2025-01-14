use crate::driver::Driver;
use bevy::prelude::*;

#[derive(Component)]
#[require(Driver)]
pub struct AiDriver;

#[derive(Component)]
pub struct AiDriverDestination(pub Vec2);
