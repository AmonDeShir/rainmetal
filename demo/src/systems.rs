use std::collections::HashMap;
use bevy::color::palettes::basic::NAVY;
use bevy::color::palettes::css::{AQUAMARINE, GOLD, GREEN_YELLOW, ROSY_BROWN, YELLOW_GREEN};
use bevy::color::Srgba;
use bevy::core::Name;
use bevy::math::{UVec2, Vec2, Vec3};
use bevy::prelude::{Camera2d, Commands, Entity, Gizmos, Local, Query, Res, Time, Timer, TimerMode, Transform, Window, With};
use rand::Rng;
use crate::components::{House, Merchant, Mushroom, Ore, Smelter};
use crate::miner::Miner;

pub fn startup(mut commands: Commands, windows: Query<&Window>) {
    for i in 0..5 {
        Miner::setup(&mut commands, Vec3::ZERO.with_x(100.0 * i as f32))
    }

    commands.spawn((
        Name::new("House"),
        House,
        Transform::from_translation(Vec3::new(100.0, 100.0, 0.0)),
    ));

    commands.spawn((
        Name::new("Smelter"),
        Smelter,
        Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
    ));

    commands.spawn((
        Name::new("Merchant"),
        Merchant,
        Transform::from_translation(Vec3::new(-300.0, -50.0, 0.0)),
    ));

    let window = windows.get_single().expect("Expected only one window! Wth");
    let window_height = window.height() / 2.0;
    let window_width = window.width() / 2.0;

    let mut rng = rand::thread_rng();

    // Begin with three mushrooms our miner can eat
    for _i in 0..3 {
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Mushroom"),
            Mushroom,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }

    // Spawn 10 ores we can mine as well
    for _i in 0..10 {
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Ore"),
            Ore,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }

    // Spawn a camera so we see something
    commands.spawn(Camera2d::default());
}

// Spawn new mushrooms if there are less than 10
pub fn spawn_random_mushroom(
    windows: Query<&Window>,
    mut commands: Commands,
    mushrooms: Query<Entity, With<Mushroom>>,
) {
    if mushrooms.iter().len() < 10 {
        let window = windows.get_single().expect("Expected only one window! Wth");
        let window_height = window.height() / 2.0;
        let window_width = window.width() / 2.0;

        let mut rng = rand::thread_rng();
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Mushroom"),
            Mushroom,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
}

// Spawn new mushrooms if there are less than 10
pub fn spawn_random_ore(
    windows: Query<&Window>,
    mut commands: Commands,
    ores: Query<Entity, With<Ore>>,
) {
    if ores.iter().len() < 10 {
        let window = windows.get_single().expect("Expected only one window! Wth");
        let window_height = window.height() / 2.0;
        let window_width = window.width() / 2.0;

        let mut rng = rand::thread_rng();
        let y = rng.gen_range(-window_height..window_height);
        let x = rng.gen_range(-window_width..window_width);
        commands.spawn((
            Name::new("Ore"),
            Ore,
            Transform::from_translation(Vec3::new(x, y, 0.0)),
        ));
    }
}

// Helper method that allows us to "delay" an action by a set amount
// Accepts a callback that has `is_completed: bool` as a parameter
pub fn action_with_progress<F>(
    progresses: &mut Local<HashMap<Entity, Timer>>,
    entity: Entity,
    time: &Res<Time>,
    delay_seconds: f32,
    on_progress: F,
) where
    F: FnOnce(bool),
{
    let progress = progresses.get_mut(&entity);

    match progress {
        Some(progress) => {
            if progress.tick(time.delta()).just_finished() {
                // TODO Wonder if we can do this in a nicer way?
                on_progress(true);
                progresses.remove(&entity);
            } else {
                on_progress(false);
            }
        }
        None => {
            progresses.insert(entity, Timer::from_seconds(delay_seconds, TimerMode::Once));
        }
    }
}

// World's shittiest graphics incoming, beware and don't copy
pub fn draw_gizmos(
    mut gizmos: Gizmos,
    q_miner: Query<&Transform, With<Miner>>,
    q_house: Query<&Transform, With<House>>,
    q_smelter: Query<&Transform, With<Smelter>>,
    q_merchant: Query<&Transform, With<Merchant>>,
    q_mushrooms: Query<&Transform, With<Mushroom>>,
    q_ore: Query<&Transform, With<Ore>>,
) {
    gizmos
        .grid_2d(
            Vec2::ZERO,
            UVec2::new(16, 9),
            Vec2::new(80., 80.),
            // Dark gray
            Srgba::new(0.1, 0.1, 0.1, 0.5),
        )
        .outer_edges();

    for miner_transform in q_miner.iter() {
        gizmos.circle_2d(miner_transform.translation.truncate(), 16., NAVY);
    }

    gizmos.rect_2d(
        q_house.get_single().unwrap().translation.truncate(),
        Vec2::new(40.0, 80.0),
        AQUAMARINE,
    );

    gizmos.rect_2d(
        q_smelter.get_single().unwrap().translation.truncate(),
        Vec2::new(30.0, 30.0),
        YELLOW_GREEN,
    );

    gizmos.circle_2d(
        q_merchant.get_single().unwrap().translation.truncate(),
        16.,
        GOLD,
    );

    for mushroom_transform in q_mushrooms.iter() {
        gizmos.circle_2d(mushroom_transform.translation.truncate(), 4., GREEN_YELLOW);
    }

    for ore_transform in q_ore.iter() {
        gizmos.circle_2d(ore_transform.translation.truncate(), 4., ROSY_BROWN);
    }
}