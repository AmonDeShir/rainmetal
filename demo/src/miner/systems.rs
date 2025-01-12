use crate::components::NeedsText;
use crate::miner::actions::{EatAction, GoToHouseAction, GoToMerchantAction, GoToMushroomAction, GoToOreAction, GoToOutsideAction, GoToSmelterAction, MineOreAction, SellMetalAction, SleepAction, SmeltOreAction};
use crate::miner::states::{Energy, GoldAmount, HasMetal, HasOre, Hunger};
use bevy::hierarchy::Children;
use bevy::prelude::{Entity, Query, Res, Text2dWriter, Time, With};
use rand::Rng;


// Increases hunger and decreases energy over time
pub fn over_time_needs_change(time: Res<Time>, mut query: Query<(&mut Hunger, &mut Energy)>) {
    let mut rng = rand::thread_rng();
    for (mut hunger, mut energy) in query.iter_mut() {
        // Increase hunger
        let r = rng.gen_range(10.0..20.0);
        let val: f64 = r * time.delta_secs_f64();
        hunger.0 += val;
        if hunger.0 > 100.0 {
            hunger.0 = 100.0;
        }

        // Decrease energy
        let r = rng.gen_range(1.0..10.0);
        let val: f64 = r * time.delta_secs_f64();
        energy.0 -= val;
        if energy.0 < 0.0 {
            energy.0 = 0.0;
        }
    }
}

pub fn print_current_local_state(
    query: Query<(
        Entity,
        &Hunger,
        &Energy,
        &HasOre,
        &HasMetal,
        &GoldAmount,
        &Children,
    )>,
    q_actions: Query<(
        Option<&SleepAction>,
        Option<&EatAction>,
        Option<&MineOreAction>,
        Option<&SmeltOreAction>,
        Option<&SellMetalAction>,
        Option<&GoToHouseAction>,
        Option<&GoToOutsideAction>,
        Option<&GoToMushroomAction>,
        Option<&GoToOreAction>,
        Option<&GoToSmelterAction>,
        Option<&GoToMerchantAction>,
    )>,
    // action_query: Query<&dyn ActionComponent>,
    q_child: Query<Entity, With<NeedsText>>,
    mut text_writer: Text2dWriter,
) {
    for (entity, hunger, energy, has_ore, has_metal, gold_amount, children) in query.iter() {
        let hunger = hunger.0;
        let energy = energy.0;
        let has_ore = has_ore.0;
        let has_metal = has_metal.0;
        let gold_amount = gold_amount.0;

        let mut current_action = "Idle";

        let (
            sleep,
            eat,
            mine,
            smelting,
            selling_metal,
            go_to_house,
            go_to_outside,
            go_to_mushroom,
            go_to_ore,
            go_to_smelter,
            go_to_merchant,
        ) = q_actions.get(entity).unwrap();

        if sleep.is_some() {
            current_action = "Sleeping";
        }

        if eat.is_some() {
            current_action = "Eating";
        }

        if mine.is_some() {
            current_action = "Mining";
        }

        if smelting.is_some() {
            current_action = "Smelting ore";
        }

        if selling_metal.is_some() {
            current_action = "Selling metal";
        }

        if go_to_house.is_some() {
            current_action = "Going to house";
        }

        if go_to_outside.is_some() {
            current_action = "Going to outside";
        }

        if go_to_mushroom.is_some() {
            current_action = "Going to mushroom";
        }

        if go_to_ore.is_some() {
            current_action = "Going to ore";
        }

        if go_to_smelter.is_some() {
            current_action = "Going to smelter";
        }

        if go_to_merchant.is_some() {
            current_action = "Going to merchant";
        }

        for &child in children.iter() {
            let text = q_child.get(child).unwrap();
            *text_writer.text(text, 0) =
                format!("{current_action}\nGold: {gold_amount}\nHunger: {hunger:.0}\nEnergy: {energy:.0}\nHas Ore? {has_ore}\nHas Metal? {has_metal}");
        }
    }
}
