use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::components::*;

#[derive(Component)]
pub struct MainMenu;

fn build_route(mut commands: Commands, query: Query<Entity, Added<MainMenu>>) {
    for route_entity in &query {

        commands.entity(route_entity).insert(
            SpatialBundle::default()
        ).with_children(|route| {
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new2d("MainMenu")),
                SourceFromCamera,
            )).with_children(|ui| {
                ui.spawn((
                    UiLink::<MainUi>::path("Tabs"),
                    UiLayout::window().size((Ab(1000.0), Ab(50.0))).pos((Ab(120.0), Ab(120.0))).pack::<Base>(),
                    Tabs {
                        tabs: vec!["Start", "Middle1", "Middle2", "Middle3", "End"],
                        active: 0,
                    }
                ));

                ui.spawn((
                    UiLink::<MainUi>::path("Tabs/Tabs/Start/ButtonC"),
                    UiLayout::window().size(Ab(44.0)).pos((Ab(-22.0), Ab(3.0))).pack::<Base>(),
                    Icon::RAINMETAL.animate(Animate::always_on(0.8).speed_up(IconEvent::Hover).speed_up_by(IconEvent::Click, 6.0))
                ));
            });
        });
    }
}

pub struct MyRoutePlugin;
impl Plugin for MyRoutePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_route.before(UiSystems::Compute));
    }
}