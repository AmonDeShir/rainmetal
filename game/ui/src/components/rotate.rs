use bevy::prelude::*;

#[derive(Component)]
pub struct Rotate(pub f32);

fn rotate(mut query: Query<(&Rotate, &mut Transform)>, time: Res<Time>) {
    for (Rotate(value), mut transform) in query.iter_mut() {
        transform.rotate_local_z(value * time.delta_seconds());
    }
}

pub struct RotatePlugin;
impl Plugin for RotatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, rotate);
    }
}