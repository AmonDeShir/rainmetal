use crate::driver::{Driver, Fuel};
use crate::location::Money;

use bevy::prelude::*;

#[derive(Component)]
#[require(Driver, Fuel, Money)]
pub struct AiDriver;

#[derive(Component)]
pub struct AiDriverDestination(pub Vec2);
