use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::routes::MainMenu;

pub fn init_ui(mut commands: Commands) {
    commands.spawn((
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        }
    ));

    commands.spawn( CursorBundle::default() );

    commands.spawn((
        SourceFromCamera,
        UiTreeBundle::<MainUi>::from(UiTree::new2d("Hello UI!")),
    )).with_children(|ui| {
        ui.spawn((
            UiLink::<MainUi>::path("Root"),
            UiLayout::window_full().pack::<Base>(),
        ));
    });

    commands.spawn(MainMenu);
}
