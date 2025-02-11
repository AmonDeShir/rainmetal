use crate::ai_driver::{CurrentTradingData, BIG_TRADING_PLAN_VALUE, HUGE_TRADING_PLAN_VALUE, MEDIUM_TRADING_PLAN_VALUE, POINT_TO_KM, SMALL_TRADING_PLAN_VALUE};
use crate::ai_trader::components::TradingPlans;
use crate::ai_trader::{TradingPlan, TradingPlanByValue};
use crate::picking::Picked;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};
use bevy_inspector_egui::egui::Ui;

pub fn ui_show_trading_plans(
    mut contexts: EguiContexts,
    query: Query<(&TradingPlans, Option<&TradingPlanByValue>, Option<&CurrentTradingData>, &Name), With<Picked>>,
    cities: Query<&Name>
) {
    let Ok((plans, plans_by_value, current_trading, name)) = query.get_single() else {
        return;
    };

    egui::Window::new("Trading Plans")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(name.as_str());

            CollapsingHeader::new("Plans")
                .default_open(false)
                .show(ui, |mut ui| {
                    for plan in plans.0.iter() {
                        let Ok(from) = cities.get(plan.from) else {
                            return
                        };

                        let Ok(to) = cities.get(plan.to) else {
                            return
                        };

                        let name = format!("{} from {} to {}", &plan.item, from.to_string(), to.to_string());

                        show_trading_plan(&name, &plan, &cities, &mut ui);
                    }
                });

            if let Some(plans) = plans_by_value {
                CollapsingHeader::new("Plans By Value")
                    .default_open(false)
                    .show(ui, |mut ui| {
                        show_optional_trading_plan(&SMALL_TRADING_PLAN_VALUE.to_string(), &plans.small, &cities, &mut ui);
                        show_optional_trading_plan(&MEDIUM_TRADING_PLAN_VALUE.to_string(), &plans.medium, &cities, &mut ui);
                        show_optional_trading_plan(&BIG_TRADING_PLAN_VALUE.to_string(), &plans.big, &cities, &mut ui);
                        show_optional_trading_plan(&HUGE_TRADING_PLAN_VALUE.to_string(), &plans.huge, &cities, &mut ui);
                    });
            }
        });

    if let Some(current_trading) = current_trading {
        egui::Window::new("Current Trading")
            .default_open(false)
            .show(contexts.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Item");
                    ui.label(&current_trading.item);
                });

                ui.horizontal(|ui| {
                    ui.label("Phase");
                    ui.label(format!("{:?}", &current_trading.phase));
                });
            });
    }
}

pub fn show_trading_plan(name: &str, plan: &TradingPlan, names: &Query<&Name>, ui: &mut Ui) {
    let Ok(from) = names.get(plan.from) else {
        return
    };

    let Ok(to) = names.get(plan.to) else {
        return
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
                ui.label("Count");
                ui.label(plan.count.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Capital");
                ui.label(plan.capital_required.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Buy Price");
                ui.label(plan.buy_price.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Sell Price");
                ui.label(plan.sell_price.to_string());
            });
        });
}

pub fn show_optional_trading_plan(name: &str, plan: &Option<TradingPlan>, names: &Query<&Name>, mut ui: &mut Ui) {
    if let Some(plan) = plan {
        show_trading_plan(name, plan, names, &mut ui);
    }
    else {
        CollapsingHeader::new(name)
            .default_open(true)
            .show(ui, |ui| {
                ui.label("None");
            });
    }
}