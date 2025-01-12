//mod components;
// mod miner;
//mod systems;
mod inventory;
mod location;
mod driver;
mod town;
mod village;
mod map;
mod picking;

use bevy::input::common_conditions::input_toggle_active;
use bevy::remote::http::RemoteHttpPlugin;
use bevy::remote::RemotePlugin;
use bevy::prelude::*;
use bevy_dogoap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
//use systems::*;
use crate::inventory::InventoryPlugin;
//use crate::miner::MinerPlugin;

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}


fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#example-canvas".into()),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(RemotePlugin::default());
    app.add_plugins(RemoteHttpPlugin::default());
    app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::KeyI))); 

    app.add_plugins(DogoapPlugin);
    app.add_plugins(InventoryPlugin);
    app.add_plugins(MapPlugin);
   // app.add_plugins(MinerPlugin);

    app.add_systems(Startup, startup);
    //app.add_systems(Update, draw_gizmos);
    //app.add_systems(FixedUpdate, spawn_random_mushroom.run_if(on_timer(Duration::from_secs(5))));
    //app.add_systems(FixedUpdate, spawn_random_ore.run_if(on_timer(Duration::from_secs(5))));

    app.run();
}