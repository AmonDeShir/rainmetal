use bevy::{prelude::*, utils::HashMap};
use ron_asset_manager::prelude::*;
use serde::Deserialize;

#[derive(RonAsset, Deserialize, Debug)]
pub enum MapItems {
    Village(LocationData),
    Town(LocationData)
}

#[derive(Asset, RonAsset, Deserialize, TypePath, Debug)]
pub struct MapData {
    pub name: String,
    #[asset]
    pub image: Shandle<Image>,
    pub items: Vec<((i32, i32), MapItems)>,
}

#[derive(Resource, Default)]
pub struct MapDataHandle(pub Handle<MapData>);


#[derive(Asset, RonAsset, Deserialize, TypePath, Debug)]
pub struct LocationData {
    pub name: String,
    pub storage: HashMap<String, i32>,
    pub needs: Option<HashMap<String, i32>,>,
    pub population: i32,
    pub money: i32,
    #[asset]
    pub image: Shandle<Image>,
}