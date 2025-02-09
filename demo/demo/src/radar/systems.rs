use bevy::color;
use super::*;
use bevy::prelude::*;

pub fn on_radar_removed(trigger: Trigger<OnRemove, TrackedByRadar>, mut query: Query<&mut TrackedByRadar>) {
    for mut radar in query.iter_mut() {
        radar.inside_radar_radius.remove(&trigger.entity());
    }
}

pub fn update(tree: Res<RadarNNTree>, mut query: Query<(Entity, &Transform, &mut TrackedByRadar)>, mut commands: Commands) {
    for (source, transform, mut radar) in query.iter_mut() {
        let mut escaped = radar.inside_radar_radius.clone();

        radar.inside_radar_radius.clear();

        for (pos, entity) in tree.within_distance(transform.translation.xy(), RADIO_TRANSMISSION_RADIUS) {
            let Some(entity) = entity else {
                continue
            };

            radar.inside_radar_radius.insert(entity, pos);

            if !escaped.contains_key(&entity) {
                if let Some(mut entity) = commands.get_entity(entity) {
                    entity.trigger(EnterRadioTransmissionRadius(source));
                }
            }
            else {
                escaped.remove(&entity);
            }
        }

        for (entity, _) in escaped.iter() {
            if let Some(mut entity) = commands.get_entity(*entity) {
                entity.trigger(ExitRadioTransmissionRadius(source));
            }
        }
    }
}

pub fn draw_debug_circles(mut gizmos: Gizmos<RadarGizmos>, query: Query<&Transform, With<TrackedByRadar>>) {
    for transform in query.iter() {
        let identity = Isometry2d {
            translation: transform.translation.xy(),
            rotation: Rot2::IDENTITY,
        };

        gizmos.circle_2d(identity, RADIO_TRANSMISSION_RADIUS, color::palettes::css::NAVY);
    }
}

pub fn update_debug_config(mut config_store: ResMut<GizmoConfigStore>, keyboard: Res<ButtonInput<KeyCode>>) {
    let (config, _) = config_store.config_mut::<RadarGizmos>();

    if keyboard.just_pressed(KeyCode::KeyR) {
        config.enabled ^= true;
    }
}
