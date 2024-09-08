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
                    UiLink::<MainUi>::path("ButtonA"),
                    UiLayout::window().size(Ab(55.0)).pos(Ab(0.0)).pack::<Base>(),
                    Icon::RAINMETAL.animate(Animate::always_on(1.0).speed_up(IconEvent::Hover).speed_up_by(IconEvent::Click, 6.0))
                ));

                ui.spawn((
                    UiLink::<MainUi>::path("ButtonB"),
                    UiLayout::window().size(Ab(55.0)).pos(Ab(60.0)).pack::<Base>(),
                    Icon::RAINMETAL.animate(Animate::always_on(1.0).slow_down(IconEvent::Hover).speed_up(IconEvent::Click))
                ));

                ui.spawn((
                    UiLink::<MainUi>::path("ButtonC"),
                    UiLayout::window().size(Ab(55.0)).pos(Ab(120.0)).pack::<Base>(),
                    Icon::RAINMETAL.animate(Animate::always_on(1.0).stop(IconEvent::Hover).speed_up(IconEvent::Click))
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