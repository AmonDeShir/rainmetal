use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::components::Rotate;

pub enum IconEvent {
    Hover,
    Click,
}

#[derive(Copy, Clone)]
pub enum IconAnimation {
    Off,
    On,
    Speed(f32),
    None,
}

impl IconAnimation {
    pub fn is_none(self) -> bool {
        match self  {
            IconAnimation::None => true,
            _ => false,
        }
    }
}

pub struct RotationSpeed;

impl RotationSpeed {
    pub const STOP: f32 = 0.0;
    pub const SLOW: f32 = 0.5;
    pub const NORMAL: f32 = 1.0;
    pub const FAST: f32 = 3.0;
}

#[derive(Component)]
pub struct Icon {
    pub url: &'static str,
    pub animate: Animate,
}

pub struct Animate {
    pub speed: f32,
    pub always: IconAnimation,
    pub hover_in: IconAnimation,
    pub hover_out: IconAnimation,
    pub click_in: IconAnimation,
    pub click_out: IconAnimation,
}

impl Animate {
    pub const NONE: Animate = Animate {
        speed: 0.0,
        always: IconAnimation::None,
        hover_in: IconAnimation::None,
        hover_out: IconAnimation::None,
        click_in: IconAnimation::None,
        click_out: IconAnimation::None,
    };

    pub fn always(state: IconAnimation, speed: f32) -> Self {
        Self { always: state, speed, ..Self::NONE }
    }

    pub fn always_on(speed: f32) -> Self {
        Self { always: IconAnimation::On, speed, ..Self::NONE }
    }

    pub fn speed_up(self, event: IconEvent) -> Self {
        self.speed_up_by(event, 3.0)
    }

    pub fn speed_up_by(self, event: IconEvent, value: f32) -> Self {
        match event {
            IconEvent::Hover => Self {
                hover_in: IconAnimation::Speed(self.speed * value),
                hover_out: if self.always.is_none() { IconAnimation::Off } else { self.always },
                ..self
            },
            IconEvent::Click => Self {
                click_in: IconAnimation::Speed(self.speed * value),
                click_out: if self.hover_in.is_none() { self.always } else { self.hover_in },
                ..self
            },
        }
    }

    pub fn slow_down(self, event: IconEvent) -> Self {
        self.slow_down_by(event, 3.0)
    }

    pub fn slow_down_by(self, event: IconEvent, value: f32) -> Self {
        match event {
            IconEvent::Hover => Self {
                hover_in: IconAnimation::Speed(self.speed / value),
                hover_out: if self.always.is_none() { IconAnimation::Off } else { self.always },
                ..self
            },
            IconEvent::Click => Self {
                click_in: IconAnimation::Speed(self.speed / value),
                click_out: if self.hover_in.is_none() { self.always } else { self.hover_in },
                ..self
            },
        }
    }

    pub fn stop(self, event: IconEvent) -> Self {
        match event {
            IconEvent::Hover => Self {
                hover_in: IconAnimation::Off,
                hover_out: if self.always.is_none() { IconAnimation::Off } else { self.always },
                ..self
            },
            IconEvent::Click => Self {
                click_in: IconAnimation::Off,
                click_out: if self.hover_in.is_none() { self.always } else { self.hover_in },
                ..self
            },
        }
    }

    pub fn start(self, event: IconEvent) -> Self {
        match event {
            IconEvent::Hover => Self {
                hover_in: IconAnimation::On,
                hover_out: if self.always.is_none() { IconAnimation::Off } else { self.always },
                ..self
            },
            IconEvent::Click => Self {
                click_in: IconAnimation::On,
                click_out: if self.hover_in.is_none() { self.always } else { self.hover_in },
                ..self
            },
        }
    }
}

impl Icon {
    pub const RAINMETAL: Self = Self {
        url: "ui/details/rainmetal.png",
        animate: Animate::NONE,
    };

    pub const RAINMETAL_DARK: Self = Self {
        url: "ui/details/rainmetal_dark.png",
        animate: Animate::NONE,
    };

    pub const RAINMETAL_OFF: Self = Self {
        url: "ui/details/rainmetal_off.png",
        animate: Animate::NONE,
    };

    pub fn animate(self, value: Animate) -> Self {
        Self { animate: value, url: self.url }
    }
}

#[derive(Component)]
struct IconUI;

fn handle_animation(animation: IconAnimation, button: &mut EntityCommands, speed: f32) {
    match animation {
        IconAnimation::On => { button.insert(Rotate(speed)); }
        IconAnimation::Off => { button.remove::<Rotate>(); }
        IconAnimation::Speed(value) => {
            button.remove::<Rotate>();
            button.insert(Rotate(value));
        }
        _ => {}
    };
}

fn build_component(mut commands: Commands, assets: Res<AssetServer>, query: Query<(Entity, &Icon), Added<Icon>>) {
    for (entity, icon) in query.iter() {
        commands.entity(entity).insert((
            UiTreeBundle::<IconUI>::from(UiTree::new2d("Icon")),
        )).with_children(|ui| {
            let mut button = ui.spawn((
                UiLink::<IconUI>::path("Icon"),
                UiLayout::solid()
                    .size((100.0, 100.0))
                    .pack::<Base>(),

                UiImage2dBundle {
                    texture: assets.load(icon.url),
                    ..default()
                }
            ));

            if !icon.animate.always.is_none() {
                handle_animation(icon.animate.always.clone(), &mut button, icon.animate.speed.clone());
            }

            if !icon.animate.hover_in.is_none() {
                let speed = icon.animate.speed.clone();
                let animation = icon.animate.hover_in.clone();

                button.insert(On::<Pointer<Over>>::commands_mut(move |input, cmd| {
                    handle_animation(animation, &mut cmd.entity(input.target), speed);
                }));
            }

            if !icon.animate.hover_out.is_none() {
                let speed = icon.animate.speed.clone();
                let animation = icon.animate.hover_out.clone();

                button.insert(On::<Pointer<Out>>::commands_mut(move |input, cmd| {
                    handle_animation(animation, &mut cmd.entity(input.target), speed);
                }));
            }

            if !icon.animate.click_in.is_none() {
                let speed = icon.animate.speed.clone();
                let animation = icon.animate.click_in.clone();

                button.insert(On::<Pointer<Down>>::commands_mut(move |input, cmd| {
                    handle_animation(animation, &mut cmd.entity(input.target), speed);
                }));
            }

            if !icon.animate.click_out.is_none() {
                let speed = icon.animate.speed.clone();
                let animation = icon.animate.click_out.clone();

                button.insert(On::<Pointer<Up>>::commands_mut(move |input, cmd| {
                    handle_animation(animation, &mut cmd.entity(input.target), speed);
                }));
            }
        });
    }
}

pub struct CustomButtonPlugin;
impl Plugin for CustomButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiGenericPlugin::<IconUI>::new())
            .add_systems(Update, build_component.before(UiSystems::Compute));
    }
}