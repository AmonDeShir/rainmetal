use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use crate::components::Location;

#[derive(Component, Clone, DatumComponent)]
pub struct Hunger(pub f64);

#[derive(Component, Clone, DatumComponent)]
pub struct Energy(pub f64);

#[derive(Component, Clone, EnumComponent)]
pub struct AtLocation(pub Location);

#[derive(Component, Clone, DatumComponent)]
pub struct HasOre(pub bool);

#[derive(Component, Clone, DatumComponent)]
pub struct HasMetal(pub bool);

#[derive(Component, Clone, DatumComponent)]
pub struct GoldAmount(pub i64);