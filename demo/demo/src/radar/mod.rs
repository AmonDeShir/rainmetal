mod systems;

use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::HashMap;
use bevy_spatial::{AutomaticUpdate, kdtree::KDTree2, TransformMode, SpatialAccess, SpatialStructure};
use crate::radar::systems::{draw_debug_circles, update_debug_config, update, on_radar_removed};

pub const RADIO_TRANSMISSION_RADIUS: f32 = 50.0;
pub const UPDATE_RADAR_TREE: f32 = 0.3;
pub const SCAN_EVERY: f32 = 1.0;

#[derive(Component, Default)]
pub struct TrackedByRadar {
    pub inside_radar_radius: HashMap<Entity, Vec2>
}

pub type RadarNNTree = KDTree2<TrackedByRadar>;

#[derive(Event)]
pub struct EnterRadioTransmissionRadius(pub Entity);

#[derive(Event)]
pub struct ExitRadioTransmissionRadius(pub Entity);

#[derive(Default, Reflect, GizmoConfigGroup)]
struct RadarGizmos {}

pub struct RadarPlugin;

impl Plugin for RadarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutomaticUpdate::<TrackedByRadar>::new()
            .with_spatial_ds(SpatialStructure::KDTree2)
            .with_frequency(Duration::from_secs_f32(UPDATE_RADAR_TREE))
            .with_transform(TransformMode::GlobalTransform)
        );

        app.init_gizmo_group::<RadarGizmos>();

        app.add_systems(Update, update.run_if(on_timer(Duration::from_secs_f32(SCAN_EVERY))));
        app.add_systems(Update, draw_debug_circles);
        app.add_systems(Update, update_debug_config);

        app.add_observer(on_radar_removed);
    }
}