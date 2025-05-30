use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui;
use bevy_inspector_egui::bevy_egui::egui::CollapsingHeader;
use bevy_inspector_egui::bevy_egui::EguiContexts;

use crate::ai_driver::AiDriverDestination;
use crate::inspector::show_item_list;
use crate::location::Money;
use crate::picking::Picked;
use crate::storage::Storage;

use super::{Driver, Fuel};

pub fn ui_show_picked_driver(
    mut contexts: EguiContexts,
    query: Query<(&Driver, &Name, &Money, &Storage, &Fuel, Option<&AiDriverDestination>), With<Picked>>,
) {
    let Ok((_, name, money, storage, fuel, destination)) = query.get_single() else {
        return;
    };

    egui::Window::new("Picked driver")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(name.as_str());

            CollapsingHeader::new("Info")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.label(name.as_str());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Money: ");
                        ui.label(money.0.to_string());
                    });
                });

            if let Some(AiDriverDestination(destination)) = destination {
                CollapsingHeader::new("Ai Destination")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(destination.x.to_string());
                            ui.label(destination.y.to_string());
                        });
                    });
            }

            ui.horizontal(|ui| {
                ui.label("Fuel: ");
                ui.label(format!("{:.2}", fuel.0));
            });

            show_item_list("Storage", &storage.items, ui);
        });
}
