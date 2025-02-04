use crate::{driver::Driver, picking::Picked};

use super::*;
use crate::driver::Fuel;
use crate::memory::{Memo, Memory, TravelPlan};
use bevy::{prelude::*, window::PrimaryWindow};
use components::{AiDriver, AiDriverDestination};

pub fn travel_to_destination(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Fuel, &AiDriverDestination), With<AiDriver>>,
) {
    for (entity, mut transform, mut fuel, destination) in query.iter_mut() {
        let destination = Vec3::new(destination.0.x, destination.0.y, transform.translation.z);

        let movement = (destination - transform.translation) * (time.delta_secs() * SPEED);
        let travel_distance = f64::from(movement.length()) * POINT_TO_KM;
        let cost = travel_distance * FUEL_CONSUMPTION_PER_KILOMETER;

        if fuel.0 < cost {
            continue;
        }

        transform.translation += movement;
        fuel.0 -= cost;

        let deg = f32::atan2(
            destination.y - transform.translation.y,
            destination.x - transform.translation.x,
        );

        transform.rotation = Quat::from_rotation_z(deg - 90f32.to_radians());

        if (transform.translation - destination).length() < 0.5 {
            commands.entity(entity).remove::<AiDriverDestination>();
        }
    }
}

pub fn update_self_position_memory(mut query: Query<(&mut Memory, Entity, &Transform, Option<&AiDriverDestination>), With<Driver>>, time: Res<Time>) {
    for (mut memory, entity, transform, destination) in query.iter_mut() {
        memory.npc_positions.insert(
            entity.clone(),
            Memo::new(TravelPlan {
                current_position: transform.translation,
                destination: destination.and_then(|destination| Option::from(Vec3::new(destination.0.x, destination.0.y, 0.0)))
            }, time.elapsed_secs())
        );
    }
}

pub fn force_ai_travel(
    _: Trigger<Pointer<Up>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    query: Query<Entity, (With<AiDriver>, With<Picked>)>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, transform)) = camera.get_single() else {
        return;
    };

    let Some(cursor_position) = cursor_position(windows.get_single(), &camera, transform) else {
        return;
    };

    let Ok(entity) = query.get_single() else {
        return;
    };

    commands
        .entity(entity)
        .insert(AiDriverDestination(cursor_position));
}

fn cursor_position<T>(
    window: Result<&Window, T>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    window
        .ok()
        .and_then(|window| window.cursor_position())
        .and_then(|pos| camera.viewport_to_world(camera_transform, pos).ok())
        .map(|ray| ray.origin.truncate())
}
