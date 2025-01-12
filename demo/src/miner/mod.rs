use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_dogoap::prelude::*;
use crate::components::NeedsText;
use crate::miner::planner::create_planner;
use action_systems::*;
use systems::*;
use states::*;

pub mod actions;
pub mod states;
mod systems;
mod planner;
mod action_systems;

#[derive(Component)]
pub struct Miner;

impl Miner {
   pub fn setup(commands: &mut Commands, pos: Vec3) {
       let text_style = TextFont {
           font_size: 18.0,
           ..default()
       };

       let  (planner, components) = create_planner();

       commands
           .spawn((
               Name::new("Miner"),
               Miner,
               planner,
               components,
               Transform::from_translation(pos),
               GlobalTransform::from_translation(pos),
           ))
           .with_children(|subcommands| {
               subcommands.spawn((
                   Transform::from_translation(Vec3::new(10.0, -10.0, 10.0)),
                   Text2d("".into()),
                   text_style,
                   bevy::sprite::Anchor::TopLeft,
                   NeedsText,
               ));
           });
   }
}

pub struct MinerPlugin;

impl Plugin for MinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            handle_go_to_outside_action,
            handle_go_to_house_action,
            handle_go_to_mushroom_action,
            handle_go_to_ore_action,
            handle_go_to_smelter_action,
            handle_go_to_merchant_action,
            handle_sleep_action,
            handle_eat_action,
            handle_mine_ore_action,
            handle_smelt_ore_action,
            handle_sell_metal_action,
        ));

        app.add_systems(
            FixedUpdate,
            over_time_needs_change.run_if(on_timer(Duration::from_millis(100))),
        );

        app.add_systems(
            FixedUpdate,
            print_current_local_state.run_if(on_timer(Duration::from_millis(50))),
        );

        register_components!(
            app,
            vec![Hunger, Energy, AtLocation, HasOre, HasMetal, GoldAmount]
        );
    }
}