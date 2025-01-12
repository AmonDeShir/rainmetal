use std::collections::HashMap;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, Local, Query, Res, Time, Timer, Transform, With, Without};
use bevy_dogoap::prelude::Planner;
use rand::Rng;
use crate::components::{House, Merchant, Mushroom, Ore, Smelter};
use crate::{action_with_progress};
use crate::components::Location;
use crate::miner::actions::{EatAction, GoToHouseAction, GoToMerchantAction, GoToMushroomAction, GoToOreAction, GoToOutsideAction, GoToSmelterAction, MineOreAction, SellMetalAction, SleepAction, SmeltOreAction};
use crate::miner::states::{AtLocation, Energy, GoldAmount, HasMetal, HasOre, Hunger};

pub fn go_to_location<T>(
    at_location: &mut AtLocation,
    delta: f32,
    origin: &mut Transform,
    destination: Vec3,
    destination_enum: Location,
    entity: Entity,
    commands: &mut Commands,
) where
    T: Component,
{
    if origin.translation.distance(destination) > 5.0 {
        // We're not quite there yet, move closer
        let direction = (destination - origin.translation).normalize();
        origin.translation += direction * 128.0 * delta;
    } else {
        // We're there!
        at_location.0 = destination_enum;

        // Remove our action to signal we've completed the move
        commands.entity(entity).remove::<T>();
    }
}

pub fn find_closest(origin: Vec3, items: Vec<(Entity, Transform)>) -> Option<(Entity, Vec3)> {
    let mut closest: Option<(Entity, Transform, f32)> = None;
    for (_entity, transform) in &items {
        match closest {
            Some((_m, _t, d)) => {
                let distance = transform.translation.distance(origin);
                if distance < d {
                    closest = Some((*_entity, *transform, distance));
                }
            }
            None => {
                closest = Some((*_entity, *transform, 1000.0));
            }
        }
    }
    match closest {
        Some((e, t, _f)) => Some((e, t.translation)),
        None => None,
    }
}

pub fn handle_go_to_house_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &GoToHouseAction, &mut Transform, &mut AtLocation), Without<House>>,
    q_house: Query<&Transform, With<House>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_house = q_house
            .get_single()
            .expect("There should only be one house!");

        go_to_location::<GoToHouseAction>(
            &mut at_location,
            time.delta_secs(),
            &mut t_entity,
            t_house.translation,
            Location::House,
            entity,
            &mut commands,
        )
    }
}

pub fn handle_go_to_smelter_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &GoToSmelterAction, &mut Transform, &mut AtLocation),
        Without<Smelter>,
    >,
    q_smelter: Query<&Transform, With<Smelter>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_smelter = q_smelter
            .get_single()
            .expect("There should only be one smelter!");

        go_to_location::<GoToSmelterAction>(
            &mut at_location,
            time.delta_secs(),
            &mut t_entity,
            t_smelter.translation,
            Location::Smelter,
            entity,
            &mut commands,
        )
    }
}

pub fn handle_go_to_outside_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &GoToOutsideAction, &mut Transform, &mut AtLocation), Without<House>>,
    q_house: Query<&Transform, With<House>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_house = q_house
            .get_single()
            .expect("There should only be one house!");

        // Outside is slightly to the left of the house... Fight me
        let offset = Vec3::new(-30.0, 0.0, 0.0);
        let new_pos = t_house.translation + offset;

        go_to_location::<GoToOutsideAction>(
            &mut at_location,
            time.delta_secs(),
            &mut t_entity,
            new_pos,
            Location::Outside,
            entity,
            &mut commands,
        )
    }
}

pub fn handle_go_to_merchant_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &GoToMerchantAction, &mut Transform, &mut AtLocation),
        Without<Merchant>,
    >,
    q_destination: Query<&Transform, With<Merchant>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let t_destination = q_destination
            .get_single()
            .expect("There should only be one merchant!");

        go_to_location::<GoToMerchantAction>(
            &mut at_location,
            time.delta_secs(),
            &mut t_entity,
            t_destination.translation,
            Location::Merchant,
            entity,
            &mut commands,
        )
    }
}



pub fn handle_go_to_mushroom_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &GoToMushroomAction, &mut Transform, &mut AtLocation),
        Without<Mushroom>,
    >,
    q_mushrooms: Query<(Entity, &Transform), With<Mushroom>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_mushrooms.iter().map(|(e, t)| (e, *t)).collect();
        let mushroom = find_closest(origin, items);

        let mushroom = match mushroom {
            Some(v) => v,
            None => panic!("No mushroom could be found, HOW?!"),
        };

        go_to_location::<GoToMushroomAction>(
            &mut at_location,
            time.delta_secs(),
            &mut t_entity,
            mushroom.1,
            Location::Mushroom,
            entity,
            &mut commands,
        )
    }
}

pub fn handle_go_to_ore_action(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &GoToOreAction, &mut Transform, &mut AtLocation), Without<Ore>>,
    q_world_resource: Query<(Entity, &Transform), With<Ore>>,
) {
    for (entity, _action, mut t_entity, mut at_location) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> =
            q_world_resource.iter().map(|(e, t)| (e, *t)).collect();
        let closest = find_closest(origin, items);

        let closest = match closest {
            Some(v) => v,
            None => panic!("No closest could be found, HOW?!"),
        };

        go_to_location::<GoToOreAction>(
            &mut at_location,
            time.delta_secs(),
            &mut t_entity,
            closest.1,
            Location::Ore,
            entity,
            &mut commands,
        )
    }
}

pub fn handle_eat_action(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &EatAction,
            &mut Transform,
            &mut Hunger,
            &mut AtLocation,
        ),
        Without<Mushroom>,
    >,
    q_mushrooms: Query<(Entity, &Transform), With<Mushroom>>,
) {
    for (entity, _action, t_entity, mut hunger, mut at_location) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_mushrooms.iter().map(|(e, t)| (e, *t)).collect();
        let mushroom = find_closest(origin, items);

        println!("Eating mushroom we found at {:?}", mushroom);

        let mushroom = match mushroom {
            Some(v) => v,
            None => panic!("No mushroom could be found, HOW?!"),
        };

        hunger.0 -= 50.0;

        commands.entity(entity).remove::<EatAction>();
        commands.entity(mushroom.0).despawn_recursive();

        at_location.0 = Location::Outside;
    }
}

pub fn handle_sleep_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &SleepAction, &mut Energy, &mut Planner)>,
) {
    let mut rng = rand::thread_rng();
    for (entity, _action, mut energy, mut planner) in query.iter_mut() {
        // Stop planning while we sleep, so we regain all the energy we can
        planner.always_plan = false;

        let r = rng.gen_range(5.0..20.0);
        let val: f64 = r * time.delta_secs_f64();
        energy.0 += val;
        if energy.0 >= 100.0 {
            commands.entity(entity).remove::<SleepAction>();

            // We can manually control actions as well if needed, here we make sure to go outside
            // after we finish sleeping
            commands.entity(entity).insert(GoToOutsideAction);
            energy.0 = 100.0;

            // Enable continuous planning again after we've done sleeping
            planner.always_plan = true;
        }
    }
}


pub fn handle_mine_ore_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &MineOreAction,
            &mut Transform,
            &mut HasOre,
            &mut AtLocation,
            &mut Energy,
        ),
        Without<Ore>,
    >,
    q_ores: Query<(Entity, &Transform), With<Ore>>,
    mut mining_progress: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, t_entity, mut has_ore, mut at_location, mut energy) in query.iter_mut() {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_ores.iter().map(|(e, t)| (e, *t)).collect();
        let closest = find_closest(origin, items);

        let closest = match closest {
            Some(v) => v,
            None => panic!("No ore could be found, HOW?!"),
        };

        action_with_progress(
            &mut mining_progress,
            closest.0,
            &time,
            2.0,
            |is_completed| {
                if is_completed {
                    has_ore.0 = true;
                    at_location.0 = Location::Outside;

                    commands.entity(entity).remove::<MineOreAction>();
                    commands.entity(closest.0).despawn_recursive();
                } else {
                    let mut rng = rand::thread_rng();

                    // Mining consumes energy!
                    let r = rng.gen_range(5.0..10.0);
                    let val: f64 = r * time.delta_secs_f64();
                    energy.0 -= val;
                    // If we're running out of energy before finishing, stop mining for now
                    if energy.0 <= 0.0 {
                        commands.entity(entity).remove::<MineOreAction>();
                        energy.0 = 0.0
                    }
                }
            },
        );
    }
}

pub fn handle_smelt_ore_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &SmeltOreAction,
            &mut Transform,
            &mut Energy,
            &mut HasOre,
            &mut HasMetal,
            &mut AtLocation,
        ),
        Without<Smelter>,
    >,
    q_smelters: Query<(Entity, &Transform), With<Smelter>>,
    mut progress: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, t_entity, mut energy, mut has_ore, mut has_metal, mut at_location) in
        query.iter_mut()
    {
        let origin = t_entity.translation;
        let items: Vec<(Entity, Transform)> = q_smelters.iter().map(|(e, t)| (e, *t)).collect();
        let closest = find_closest(origin, items);

        let closest = match closest {
            Some(v) => v,
            None => panic!("No ore could be found, HOW?!"),
        };

        action_with_progress(&mut progress, closest.0, &time, 5.0, |is_completed| {
            if is_completed {
                has_metal.0 = true;

                has_ore.0 = false;

                at_location.0 = Location::Outside;

                commands.entity(entity).remove::<SmeltOreAction>();
            } else {
                let mut rng = rand::thread_rng();
                // Smelting consumes even more energy!
                let r = rng.gen_range(10.0..15.0);
                let val: f64 = r * time.delta_secs_f64();
                energy.0 -= val;
                if energy.0 <= 0.0 {
                    commands.entity(entity).remove::<SmeltOreAction>();
                    energy.0 = 0.0
                }
            }
        });
    }
}

pub fn handle_sell_metal_action(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &SellMetalAction,
            &mut Transform,
            &mut HasMetal,
            &mut GoldAmount,
            &mut AtLocation,
        ),
        Without<Smelter>,
    >,
    mut progress: Local<HashMap<Entity, Timer>>,
) {
    for (entity, _action, _t_entity, mut has_metal, mut gold_amount, mut at_location) in
        query.iter_mut()
    {
        action_with_progress(&mut progress, entity, &time, 1.0, |is_completed| {
            if is_completed {
                has_metal.0 = false;

                gold_amount.0 += 1;

                at_location.0 = Location::Outside;

                commands.entity(entity).remove::<SellMetalAction>();
            } else {
                // Do nothing in particular while we perform the selling
            }
        });
    }
}