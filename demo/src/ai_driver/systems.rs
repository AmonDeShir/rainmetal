use crate::{driver::Driver, map::Map, picking::Picked};

use super::*;
use bevy::{prelude::*, window::PrimaryWindow};
use components::{AiDriver, AiDriverDestination};

const SPEED: f32 = 3.0;
const ROTATION_SPEED: f32 = 1.0;

pub fn travel_to_destination(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &AiDriverDestination), With<AiDriver>>,
) {
    for (entity, mut transform, destination) in query.iter_mut() {
        let destination = Vec3::new(destination.0.x, destination.0.y, transform.translation.z);

        transform.translation = Vec3::lerp(
            transform.translation,
            destination,
            time.delta_secs() * SPEED,
        );

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
