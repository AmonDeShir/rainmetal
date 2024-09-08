use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_lunex::prelude::*;
use crate::components::{TabButton, TabButtonType};

#[derive(Component)]
pub struct Tabs {
    pub tabs: Vec<&'static str>,
    pub active: u32
}

#[derive(Component)]
struct TabsUI;

fn build_component(mut commands: Commands, query: Query<(Entity, &Tabs), Added<Tabs>>) {
    for (entity, component) in query.iter() {
        commands.entity(entity).insert((
            UiTreeBundle::<TabsUI>::from(UiTree::new2d("Tabs")),
        )).with_children(|ui| {
            ui.spawn((
                UiLink::<TabsUI>::path("Tabs"),
                UiLayout::window_full().pack::<Base>(),
            ));

            if let Some(text) = component.tabs.first() {
                ui.spawn((
                    UiLink::<TabsUI>::path("Tabs/Start"),
                    UiLayout::window().size((Ab(200.0), Ab(50.0))).pos((Ab(0.0), Ab(0.0))).pack::<Base>(),
                    TabButton {
                        tab_type: TabButtonType::Start,
                        text: text.to_string(),
                    }
                ));
            }

            if (component.tabs.len() > 2) {
                for (index, text) in component.tabs[1..component.tabs.len() - 1].iter().enumerate() {
                    ui.spawn((
                        UiLink::<TabsUI>::path("Tabs/".to_owned() + &*index.to_string()),
                        UiLayout::window().size((Ab(200.0), Ab(50.0))).pos((Ab(200.0 * (index as f32 + 1.0)), Ab(0.0))).pack::<Base>(),
                        TabButton {
                            tab_type: TabButtonType::Normal,
                            text: text.to_string(),
                        }
                    ));
                }
            }

            if (component.tabs.len() <= 1) {
                return;
            }

            if let Some(text) = component.tabs.last() {
                ui.spawn((
                    UiLink::<TabsUI>::path("Tabs/End"),
                    UiLayout::window().size((Ab(200.0), Ab(50.0))).pos((Ab(200.0 * (component.tabs.len() as f32 - 1.0)), Ab(0.0))).pack::<Base>(),
                    TabButton {
                        tab_type: TabButtonType::End,
                        text: text.to_string(),
                    }
                ));
            }
        });
    }
}

pub struct TabsPlugin;
impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiGenericPlugin::<TabsUI>::new())
            .add_systems(Update, build_component.before(UiSystems::Compute));
    }
}