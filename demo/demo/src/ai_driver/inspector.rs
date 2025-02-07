use std::collections::BTreeMap;
use crate::picking::Picked;
use bevy::prelude::*;
use bevy::reflect::Enum;
use bevy_dogoap::prelude::{ActionComponent, Compare, Datum, DatumComponent, Planner};
use bevy_inspector_egui::bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};

pub fn ui_show_ai_plans(
    mut contexts: EguiContexts,
    planners: Query<(Entity, &Name, &Planner), With<Picked>>,
    actions: Query<(Entity, &dyn ActionComponent)>,
    datums: Query<(Entity, &dyn DatumComponent)>,
) {
    let Ok((entity, name, planner)) = planners.get_single() else {
        return
    };

    egui::Window::new("GOAP Planner")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(name.as_str());

            CollapsingHeader::new("Planner")
                .default_open(true)
                .show(ui, |ui| {
                    let current_goal = match planner.current_goal.clone() {
                        Some(goal) => goal.requirements,
                        None => BTreeMap::new(),
                    };

                    let current_action = match planner.current_action.clone() {
                        Some(action) => action.key,
                        None => "None".to_string(),
                    };

                    CollapsingHeader::new("Goal")
                        .default_open(true)
                        .show(ui, |ui| {
                            display_goal(&current_goal, ui);
                        });

                    CollapsingHeader::new("Action")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label(current_action);
                        });


                    CollapsingHeader::new("Plan")
                        .default_open(true)
                        .show(ui, |ui| {
                            for action in planner.current_plan.iter() {
                                ui.label(action);
                            }
                        });

                    CollapsingHeader::new("All Goals")
                        .default_open(true)
                        .show(ui, |ui| {
                            for goal in planner.goals.iter() {
                                display_goal(&goal.requirements, ui);
                            }
                        });

                    CollapsingHeader::new("All Actions")
                        .default_open(true)
                        .show(ui, |ui| {
                            for (action, _) in planner.actions_map.iter() {
                                ui.label(action);
                            }
                        });
                });


            CollapsingHeader::new("Actions (Components)")
                .default_open(true)
                .show(ui, |ui| {
                    for (_entity, actions) in actions.get(entity).iter() {
                        for action in actions.iter() {
                            ui.label(action.action_type_name());
                        }
                    }
                });

            CollapsingHeader::new("State")
                .default_open(true)
                .show(ui, |ui| {
                    for (_entity, data) in datums.get(entity).iter() {
                        for datum in data.iter() {
                            ui.horizontal(|ui| {
                                ui.label(datum.field_key().to_string());
                                ui.label(datum_to_string(datum.field_value()));
                            });
                        }
                    }
                });
        });
}

fn datum_to_string(datum: Datum) -> String {
    match datum {
        Datum::Bool(v) => v.to_string(),
        Datum::F64(v) => format!("{:.2}", v).to_string(),
        Datum::I64(v) => format!("{}", v).to_string(),
        Datum::Enum(v) => format!("{}", v).to_string(),
    }
}

fn display_goal(goal: &BTreeMap<String, Compare>, ui: &mut egui::Ui) {
    for (name, compare) in goal.iter() {
        ui.horizontal(|ui| {
            ui.label(name);
            ui.label(compare.variant_name().to_string());
            ui.label(datum_to_string(compare.value()));
        });
    }
}