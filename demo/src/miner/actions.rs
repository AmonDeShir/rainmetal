use bevy::prelude::*;
use bevy_dogoap::prelude::*;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct EatAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct SleepAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct MineOreAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct SmeltOreAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct SellMetalAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToOutsideAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToHouseAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToMushroomAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToOreAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToSmelterAction;

#[derive(Component, Clone, Reflect, Default, ActionComponent)]
pub struct GoToMerchantAction;