use bevy_inspector_egui::bevy_egui::{egui::{self, CollapsingHeader}, EguiContexts};
use bevy::prelude::*;

use super::*;
use crate::picking::Picked;

pub fn ui_show_picked_location(mut contexts: EguiContexts, query: Query<&Location, With<Picked>>) {
    egui::Window::new("Picked location").default_open(false).show(contexts.ctx_mut(), |ui| {
        let Ok(location) = query.get_single() else {
            ui.label("Click a location to select it.");

            return;
        };

        ui.label(location.name.clone());
        
        CollapsingHeader::new("Info")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.label(location.name.clone());
                });

                ui.horizontal(|ui| {
                    ui.label("Population: ");
                    ui.label(location.population.to_string());
                });
            });

        CollapsingHeader::new("Storage")
            .default_open(true)
            .show(ui, |ui| {
                for (name, quantity) in location.storage.items.iter() {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        ui.label(quantity.to_string());
                    });
                }
            });

        CollapsingHeader::new("Market")
            .default_open(true)
            .show(ui, |ui| {
                CollapsingHeader::new("Prices")
                    .default_open(true)
                    .show(ui, |ui| {
                        for (name, price) in location.market.prices.iter() {
                            ui.horizontal(|ui| {
                                ui.label(name);
                                ui.label(price.to_string());
                            });
                        }
                    });    

                CollapsingHeader::new("Storage")
                    .default_open(true)
                    .show(ui, |ui| {
                        for (name, quantity) in location.market.storage.items.iter() {
                            ui.horizontal(|ui| {
                                ui.label(name);
                                ui.label(quantity.to_string());
                            });
                        }
                    });    
            });
    });
}