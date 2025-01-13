use bevy::prelude::*;

use crate::{location::Location, picking::{pick_on, recolor_on, Picked}, storage::Storage, town::Town, village::Village};
use super::{assets::*, Map, MapPickedIncicator};

pub fn load_map(
    mut ev_asset: EventReader<AssetEvent<MapData>>,
    assets_map: ResMut<Assets<MapData>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<(Entity, &Children), With<Map>>,
) {
    for ev in ev_asset.read() {        
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                let map_data: &MapData = assets_map.get(*id).unwrap();
                
                let (mut map_entity, children ) = match query.get_single() {
                    Ok ((id, children)) => (commands.entity(id), Some(children)),
                    Err(_) => (commands.spawn_empty(), None),
                };
                
                let map_entity = map_entity.insert((
                    Map,
                    Transform::from_xyz(0., 0., 0.),
                    Sprite::from_image(map_data.image.handle.clone()),
                    Name::new(map_data.name.to_string()),
                )).id();

                if let Some(children) = children {
                    for &child in children.iter() {
                        commands.entity(map_entity).remove_children(&[child]);
                        commands.entity(child).despawn();
                    }
                }
                
                let mut children = vec![];

                for (pos, location_data) in map_data.items.iter() {
                    match location_data {
                        MapItems::Village(location) => {
                            let mut entity = commands.spawn(Village);
                            create_location(location, &asset_server, pos, &mut entity);
                            make_entity_interactable(&mut entity);
                            
                            children.push(entity.id());
                        }

                        MapItems::Town(location) => {
                            let mut entity = commands.spawn(Town);
                            create_location(location, &asset_server, pos, &mut entity);
                            make_entity_interactable(&mut entity);

                            children.push(entity.id());
                        }

                    };
                }

                commands.entity(map_entity).add_children(&children);
            }

            _ => {}
        }
    }
}

fn create_location<'a>(location: &LocationData, asset_server: &Res<AssetServer>, (pos_x, pos_y): &(i32, i32), commands: &mut EntityCommands) {   
   commands.insert((
        Sprite::from_image(asset_server.load(location.image.path.to_string())),        
        Transform::from_xyz(*pos_x as f32, *pos_y as f32, 5.),
        Name::new(location.name.to_string()),
        Storage { items: location.storage.clone() },
        Location { 
            population: location.population,
            consumption: location.consumption.clone(),
            production: location.production.clone(),
            surplus_factor: location.surplus_factor.clone(),
        },
    ));
}

fn make_entity_interactable(entity: &mut EntityCommands) {
    entity.observe(recolor_on::<Pointer<Over>>(Color::srgb(1.0, 0.66, 0.0)));
    entity.observe(recolor_on::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)));
    entity.observe(pick_on::<Pointer<Up>>());
}

pub fn setup(server: Res<AssetServer>, mut handler: ResMut<MapDataHandle>) {
    handler.0 = server.load("map.ron");
}


pub fn on_picked_location(trigger: Trigger<OnAdd, Picked>, query: Query<Entity, With<Location>>, mut commands: Commands, assets: Res<AssetServer>) {
    let Ok(entity) = query.get(trigger.entity()) else {
        return;
    };

    let indicator = commands.spawn((
        MapPickedIncicator,
        Sprite::from_image(assets.load("images/selected.png")),
        Transform::from_xyz(0.0, 0.0, 10.0),
    )).id();

    commands.entity(entity).add_child(indicator);
}

pub fn on_unpicked_location(trigger: Trigger<OnRemove, Picked>, query: Query<(Entity, &Parent), With<MapPickedIncicator>>, mut commands: Commands) {
    for (entity, parent) in query.iter() {
        if parent.get() == trigger.entity() {
            let mut child = commands.entity(entity);

            child.remove_parent();
            child.despawn();
        }
    }
}