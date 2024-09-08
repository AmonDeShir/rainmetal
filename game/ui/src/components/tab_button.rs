use bevy::a11y::accesskit::Size;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_lunex::prelude::*;

#[derive(Default)]
pub enum TabButtonType {
    Start,
    #[default]
    Normal,
    End
}

#[derive(Component, Default)]
pub struct TabButton {
    pub text: String,
    pub tab_type: TabButtonType,
}

#[derive(Component)]
struct TabButtonUI;

fn build_component(mut commands: Commands, assets: Res<AssetServer>, query: Query<(Entity, &TabButton), Added<TabButton>>) {
    for (entity, button) in query.iter() {
        commands.entity(entity).insert((
            UiTreeBundle::<TabButtonUI>::from(UiTree::new2d("TabButton/Image")),
        )).with_children(|ui| {
            let texture;
            let size;
            let border;

            match button.tab_type {
                TabButtonType::Start => {
                    size = (299.19, 100.0);
                    border = BorderRect::from([15.0, 147.0, 1.0, 1.0]);
                    texture = assets.load("ui/buttons/card_start.png");
                },
                TabButtonType::Normal => {
                    size = (334.68, 100.0);
                    border = BorderRect::from([147.0, 147.0, 1.0, 1.0]);
                    texture = assets.load("ui/buttons/card.png");
                },
                TabButtonType::End => {
                    size = (281.85, 100.0);
                    border = BorderRect::from([147.0, 15.0, 1.0, 1.0]);
                    texture = assets.load("ui/buttons/card_end.png");
                }
            }

            ui.spawn((
                UiLink::<TabButtonUI>::path("TabButton"),
                UiLayout::window_full().pack::<Base>(),

                UiImage2dBundle {
                    texture,
                    sprite: Sprite::default(),
                    ..default()
                },
            ));

            ui.spawn((
                UiLink::<TabButtonUI>::path("TabButton/Image/Text"),
                UiLayout::window().pos(Rl((50., 50.))).anchor(Anchor::Center).pack::<Base>(),
                UiText2dBundle {
                    text: Text::from_section(
                        &button.text,
                        TextStyle::default(),
                    ),
                    ..default()
                },
            ));
        });
    }
}

pub struct TabButtonPlugin;
impl Plugin for TabButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiGenericPlugin::<TabButtonUI>::new())
            .add_systems(Update, build_component.before(UiSystems::Compute));
    }
}