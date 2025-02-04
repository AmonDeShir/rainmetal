use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};
use super::*;
use crate::{inspector::show_item_list, picking::Picked};

pub fn ui_show_memory(
    mut contexts: EguiContexts,
    query: Query<(&Memory, &Name), With<Picked>>,
    names: Query<&Name>,
) {
    let Ok((memory, name)) = query.get_single() else {
        return;
    };

    egui::Window::new("Memory")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(name.as_str());


            CollapsingHeader::new("Locations")
                .default_open(true)
                .show(ui, |ui| {
                    for (entity, memo) in memory.city_prices.iter() {
                        let Ok(location) = names.get(*entity) else {
                            continue
                        };

                        CollapsingHeader::new(location.to_string())
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("time");
                                    ui.label(format!("{:.02}", memo.time.to_string()));
                                });

                                show_item_list("Sell Prices", &memo.value.sell_price, ui);
                                show_item_list("Buy Prices", &memo.value.buy_price, ui);
                            });
                    }
                });

            CollapsingHeader::new("Characters")
                .default_open(true)
                .show(ui, |ui| {
                    for (entity, memo) in memory.npc_positions.iter() {
                        let Ok(name) = names.get(*entity) else {
                            continue
                        };

                        CollapsingHeader::new(name.as_str())
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("time");
                                    ui.label(format!("{:.02}", memo.time.to_string()));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("current position");
                                    ui.label(format!("{:.02}, {:.02}, {:.02}",
                                        memo.value.current_position.x,
                                        memo.value.current_position.y,
                                        memo.value.current_position.z
                                    ));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("destination");

                                    ui.label(match memo.value.destination {
                                        Some(pos) =>  format!("{:.02}, {:.02}, {:.02}", pos.x, pos.y, pos.z),
                                        None => "None".to_string()
                                    });
                                });
                            });
                    }
                });
        });
}