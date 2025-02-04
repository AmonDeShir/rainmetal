use super::*;
use crate::ai_driver::POINT_TO_KM;
use crate::ai_trader::components::TradingPlans;
use crate::ai_trader::systems::calculate_travel_cost;
use crate::local_economy::LocalEconomy;
use crate::picking::Picked;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};

pub fn ui_show_trading_plans(
    mut contexts: EguiContexts,
    query: Query<(&TradingPlans, &Name), With<Picked>>,
    cities: Query<(&Name, &LocalEconomy)>,
) {
    let Ok((plans, name)) = query.get_single() else {
        return;
    };

    egui::Window::new("Trading Plans")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(name.as_str());

            CollapsingHeader::new("Plans")
                .default_open(true)
                .show(ui, |ui| {
                    for plan in plans.0.iter() {
                        let Ok((from, from_economy)) = cities.get(plan.from) else {
                            continue
                        };

                        let Ok((to, _)) = cities.get(plan.to) else {
                            continue
                        };

                        let name = format!("{} from {} to {}", &plan.item, from.to_string(), to.to_string());

                        let Some(fuel) = from_economy.sell_price.get("fuel") else {
                            continue
                        };

                        CollapsingHeader::new(name)
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Item");
                                    ui.label(plan.item.clone());
                                });

                                ui.horizontal(|ui| {
                                    ui.label("From");
                                    ui.label(from.to_string());
                                });

                                ui.horizontal(|ui| {
                                    ui.label("To");
                                    ui.label(to.to_string());
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Profit");
                                    ui.label(format!("{:.02}", plan.profit));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Distance");
                                    ui.label(format!("{:.02}km", plan.distance as f64 * POINT_TO_KM));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Profit / Distance");
                                    ui.label(format!("{:.02}", plan.profit / plan.distance));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Travel Cost");
                                    ui.label(calculate_travel_cost(plan.distance as f64, *fuel).to_string());
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Min Count");
                                    ui.label(plan.min_count.to_string());
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Min Capital");
                                    ui.label(plan.min_capital_required.to_string());
                                });
                            });
                    }
                });
        });
}