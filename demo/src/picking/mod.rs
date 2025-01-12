use std::fmt::Debug;
use bevy::prelude::*;


#[derive(Component, Reflect)]
pub struct Picked;


pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Picked>();
    }
}

pub fn recolor_on<E: Debug + Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.entity()) else {
            return;
        };
        sprite.color = color;
    }
}

pub fn pick_on<E: Debug + Clone + Reflect>() -> impl Fn(Trigger<E>, Commands, Query<Entity, With<Picked>>) {
    move |ev, mut commands, query| {
        for entity in query.iter() {
            commands.entity(entity).remove::<Picked>();
        }

        commands.entity(ev.entity()).insert(Picked);
    }
}
