use crate::driver::Driver;
use crate::location::Location;
use crate::picking::Picked;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui::{self},
    EguiContexts,
};
use bevy_inspector_egui::egui::{CollapsingHeader, Color32, RichText};

pub fn ui_show_items(
    mut contexts: EguiContexts,
    mut commands: Commands,
    drivers: Query<(Entity, &Name, Option<&Picked>), With<Driver>>,
    locations: Query<(Entity, &Name, Option<&Picked>), With<Location>>,
    all_picked: Query<Entity, With<Picked>>
) {
    egui::Window::new("All Objects")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            CollapsingHeader::new("Locations")
                .default_open(false)
                .show(ui, |mut ui| {
                    for (entity, name, is_picked) in locations.iter().sort_by::<&Name>(|a, b| a.cmp(b)) {
                        show_picking_button(&entity, &name.to_string(), &mut ui, &is_picked.is_some(), &all_picked, &mut commands);
                    }
                });

            CollapsingHeader::new("Drivers")
                .default_open(false)
                .show(ui, |mut ui| {
                    for (entity, name, is_picked) in drivers.iter().sort_by::<&Name>(|a, b| a.cmp(b)) {
                        show_picking_button(&entity, &name.to_string(), &mut ui, &is_picked.is_some(), &all_picked, &mut commands);
                    }
                });
        });
}

fn show_picking_button(entity: &Entity, name: &str, ui: &mut egui::Ui, is_picked: &bool, all_picked: &Query<Entity, With<Picked>>,  commands: &mut Commands) {
    let mut text = RichText::new(name);

    if *is_picked {
        text = text.color(Color32::from_rgb(255, 168, 0));
    }

    let checked = ui.button(text).clicked();

    if checked {
        for entity in all_picked.iter() {
            commands.entity(entity).remove::<Picked>();
        }

        commands.entity(*entity).insert(Picked);
    }
}