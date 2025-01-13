use bevy_dogoap::create_planner;
use bevy_dogoap::prelude::*;
use crate::components::Location;
use crate::miner::actions::*;
use crate::miner::states::*;

pub type MinerComponents =  (GoldAmount, Hunger, Energy, AtLocation, HasOre, HasMetal);

pub fn create_planner() -> (Planner, MinerComponents) {
    let gold_goal = Goal::from_reqs(&[GoldAmount::is(3)]);

    let sleep_action = SleepAction::new()
        .add_precondition(Energy::is_less(50.0))
        .add_precondition(AtLocation::is(Location::House))
        .add_mutator(Energy::increase(100.0))
        .set_cost(1);

    let eat_action = EatAction::new()
        .add_precondition(Hunger::is_more(50.0))
        .add_precondition(AtLocation::is(Location::Mushroom))
        .add_mutator(Hunger::decrease(25.0))
        .add_mutator(AtLocation::set(Location::Outside))
        .set_cost(2);

    let mine_ore_action = MineOreAction::new()
        .add_precondition(Energy::is_more(10.0))
        .add_precondition(Hunger::is_less(75.0))
        .add_precondition(AtLocation::is(Location::Ore))
        .add_mutator(HasOre::set(true))
        .set_cost(3);

    let smelt_ore_action = SmeltOreAction::new()
        .add_precondition(Energy::is_more(10.0))
        .add_precondition(Hunger::is_less(75.0))
        .add_precondition(AtLocation::is(Location::Smelter))
        .add_precondition(HasOre::is(true))
        .add_mutator(HasOre::set(false))
        .add_mutator(HasMetal::set(true))
        .set_cost(4);

    let sell_metal_action = SellMetalAction::new()
        .add_precondition(Energy::is_more(10.0))
        .add_precondition(Hunger::is_less(75.0))
        .add_precondition(AtLocation::is(Location::Merchant))
        .add_precondition(HasMetal::is(true))
        .add_mutator(GoldAmount::increase(1))
        .add_mutator(HasMetal::set(false))
        .set_cost(5);

    let go_to_outside_action = GoToOutsideAction::new()
        .add_mutator(AtLocation::set(Location::Outside))
        .set_cost(1);

    let go_to_house_action = GoToHouseAction::new()
        .add_precondition(AtLocation::is(Location::Outside))
        .add_mutator(AtLocation::set(Location::House))
        .set_cost(1);

    let go_to_mushroom_action = GoToMushroomAction::new()
        .add_precondition(AtLocation::is(Location::Outside))
        .add_mutator(AtLocation::set(Location::Mushroom))
        .set_cost(2);

    let go_to_ore_action = GoToOreAction::new()
        .add_precondition(AtLocation::is(Location::Outside))
        .add_mutator(AtLocation::set(Location::Ore))
        .set_cost(3);

    let go_to_smelter_action = GoToSmelterAction::new()
        .add_precondition(AtLocation::is(Location::Outside))
        .add_mutator(AtLocation::set(Location::Smelter))
        .set_cost(4);

    let go_to_merchant_action = GoToMerchantAction::new()
        .add_precondition(AtLocation::is(Location::Outside))
        .add_mutator(AtLocation::set(Location::Merchant))
        .set_cost(5);

    let (mut planner, components) = create_planner!({
        actions: [
            (EatAction, eat_action),
            (SleepAction, sleep_action),
            (MineOreAction, mine_ore_action),
            (SmeltOreAction, smelt_ore_action),
            (SellMetalAction, sell_metal_action),

            (GoToOutsideAction, go_to_outside_action),
            (GoToHouseAction, go_to_house_action),
            (GoToMushroomAction, go_to_mushroom_action),
            (GoToOreAction, go_to_ore_action),
            (GoToSmelterAction, go_to_smelter_action),
            (GoToMerchantAction, go_to_merchant_action),
        ],
        state: [GoldAmount(0), Hunger(25.0), Energy(75.0), AtLocation(Location::Outside), HasOre(false), HasMetal(false)],
        goals: [gold_goal],
    });

    // Don't remove the goal if there is no plan found
    planner.remove_goal_on_no_plan_found = false;
    // Re-calculate our plan constantly
    planner.always_plan = true;
    // Set current goal to be to acquire gold
    planner.current_goal = Some(gold_goal.clone());

    (planner, components)
}