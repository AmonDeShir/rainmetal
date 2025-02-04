//mod components;
// mod miner;
//mod systems;
mod ai_driver;
mod driver;
mod inspector;
mod local_economy;
mod location;
mod map;
mod needs;
mod picking;
mod storage;
mod town;
mod village;
mod ai_trader;
mod memory;
mod radar;

use ai_driver::AiDriverPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::remote::http::RemoteHttpPlugin;
use bevy::remote::RemotePlugin;
use bevy_dogoap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use driver::DriverPlugin;
use local_economy::LocalEconomyPlugin;
use location::LocationPlugin;
use map::MapPlugin;
use crate::ai_trader::AiTraderPlugin;
use crate::memory::MemoryPlugin;
use crate::radar::RadarPlugin;
//use systems::*;
use crate::storage::StoragePlugin;
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
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::KeyI)),
    );

    app.add_plugins(DogoapPlugin);
    app.add_plugins(LocationPlugin);
    app.add_plugins(StoragePlugin);
    app.add_plugins(MapPlugin);
    app.add_plugins(LocalEconomyPlugin);
    app.add_plugins(DriverPlugin);
    app.add_plugins(AiDriverPlugin);
    app.add_plugins(RadarPlugin);
    app.add_plugins(MemoryPlugin);
    app.add_plugins(AiTraderPlugin);

    app.add_systems(Startup, startup);


    app.run();
}
