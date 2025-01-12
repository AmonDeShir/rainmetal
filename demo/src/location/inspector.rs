use bevy_inspector_egui::{bevy_egui::{egui::{self, CollapsingHeader}, EguiContexts}, egui::Ui};
use bevy::{prelude::*, utils::HashMap};

use super::*;
use crate::{needs::Needs, picking::Picked};

pub fn ui_show_picked_location(mut contexts: EguiContexts, query: Query<(&Location, &Name, &LocalEconomy, &Storage, &Needs), With<Picked>>) {
    egui::Window::new("Picked location").default_open(false).show(contexts.ctx_mut(), |ui| {
        let Ok((location, name, economy, storage, needs)) = query.get_single() else {
            ui.label("Click a location to select it.");

            return;
        };

        ui.label(name.as_str());
        
        CollapsingHeader::new("Info")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.label(name.as_str());
                });

                ui.horizontal(|ui| {
                    ui.label("Population: ");
                    ui.label(location.population.to_string());
                });
            });


        show_item_list("Storage", &storage.items, ui);
        show_item_list("Needs", &needs.items, ui);


        CollapsingHeader::new("Local Economy")
            .default_open(true)
            .show(ui, |ui| {
                show_item_list("Sell Prices", &economy.sell_price, ui);
                show_item_list("Buy Prices", &economy.buy_price, ui);
            });
    });
}


fn show_item_list(title: &str, items: &HashMap<String, i32>, ui: &mut Ui) {
    CollapsingHeader::new(title)
        .default_open(true)
        .show(ui, |ui| {
            for (name, quantity) in items.iter() {
                ui.horizontal(|ui| {
                    ui.label(name);
                    ui.label(quantity.to_string());
                });
            }
        });
}