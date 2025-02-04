use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};

use super::*;
use crate::{inspector::show_item_list, needs::Needs, picking::Picked};

pub fn ui_show_picked_location(
    mut contexts: EguiContexts,
    query: Query<(&Location, &Name, &LocalEconomy, &Storage, &Needs), With<Picked>>,
) {
    let Ok((location, name, economy, storage, needs)) = query.get_single() else {
        return;
    };

    egui::Window::new("Picked location")
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
                        ui.label("Population: ");
                        ui.label(location.population.to_string());
                    });
                });

            show_item_list("Storage", &storage.items, ui);
            show_item_list("Needs", &needs.items, ui);
            show_item_list("Production", &location.production, ui);
            show_item_list("Consumption", &location.consumption, ui);
            show_item_list("Surplus Factor", &location.surplus_factor, ui);

            CollapsingHeader::new("Local Economy")
                .default_open(true)
                .show(ui, |ui| {
                    show_item_list("Sell Prices", &economy.sell_price, ui);
                    show_item_list("Buy Prices", &economy.buy_price, ui);
                });
        });
}
