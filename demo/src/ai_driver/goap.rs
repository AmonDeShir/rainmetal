use bevy::prelude::*;
use bevy_dogoap::prelude::*;

const RENT_COST_MONTHLY: i64 = 100;

#[derive(Component, Reflect, Clone, DatumComponent)]
struct Gold(i64);


struct DriveAction;

struct BuyItemAction;

struct SellItemAction;


fn setup_goap(mut commands: Commands) {
    let  rent_goal = Goal::from_reqs(&[Gold::is_more(RENT_COST_MONTHLY)]);

}

