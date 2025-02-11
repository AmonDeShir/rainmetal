use super::*;
use crate::driver::Fuel;
use bevy::prelude::*;
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