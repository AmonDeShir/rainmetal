use bevy::prelude::{Component, Reflect};
use bevy_dogoap::prelude::*;

#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Smelter;

#[derive(Component)]
pub struct Mushroom;

#[derive(Component)]
pub struct Ore;

#[derive(Component)]
pub struct Merchant;

// UI elements
#[derive(Component)]
pub struct NeedsText;


#[derive(Clone, Default, Reflect, Copy, EnumDatum)]
pub enum Location {
    #[default]
    House,
    Outside,
    Mushroom,
    Ore,
    Smelter,
    Merchant,
}
