use crate::picking::Picked;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};
use bevy_inspector_egui::egui::Ui;
use rainmetal_goap::prelude::*;

pub trait DebugPlannerState {
    fn show_egui(&self, names: &Query<&Name>, ui: &mut Ui);
}

pub fn ui_show_ai_plans<S: PlannerState + DebugPlannerState>(
    mut contexts: EguiContexts,
    planners: Query<(&Name, &Planner<S>, &S), With<Picked>>,
    names: Query<&Name>,
) {
    let Ok((name, planner, state)) = planners.get_single() else {
        return
    };

    egui::Window::new("GOAP Planner")
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(name.as_str());

            CollapsingHeader::new("Planner")
                .default_open(true)
                .show(ui, |ui| {
                    let goal = match &planner.goal {
                        Some(goal) => goal.key.clone(),
                        None => "None".to_string(),
                    };

                    let current_action = match planner.action.clone() {
                        Some(action) => action.key,
                        None => "None".to_string(),
                    };

                    CollapsingHeader::new("Goal")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label(goal);
                        });

                    CollapsingHeader::new("Action")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label(current_action);
                        });


                    CollapsingHeader::new("Plan")
                        .default_open(false)
                        .show(ui, |ui| {
                            for action in planner.plan.iter() {
                                ui.label(action);
                            }
                        });

                    CollapsingHeader::new("All Goals")
                        .default_open(false)
                        .show(ui, |ui| {
                            for goal in planner.all_goals.iter() {
                                display_goal(goal, state, ui);
                            }
                        });

                    CollapsingHeader::new("All Actions")
                        .default_open(false)
                        .show(ui, |ui| {
                            for action in planner.all_actions.iter() {
                                display_action(action, &names, state, ui);
                            }
                        });
                });

            CollapsingHeader::new("State")
                .default_open(false)
                .show(ui, |ui| {
                    state.show_egui(&names, ui);
                });
        });
}

fn display_goal<S: PlannerState>(goal: &Goal<S>, state: &S, ui: &mut Ui) {
    let priority = (goal.priority)(state);
    let distance = (goal.distance)(state, DistanceCalculator::new());
    let requirements = (goal.requirements)(state);

    CollapsingHeader::new(&goal.key)
        .default_open(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(&goal.key);
            });

            ui.horizontal(|ui| {
                ui.label("Priority");
                ui.label(&priority.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Distance");
                ui.label(&distance.value.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Reached");
                ui.label(&requirements.to_string());
            });
        });
}

fn display_action<S: PlannerState + DebugPlannerState>(action: &Action<S>, names: &Query<&Name>, state: &S, ui: &mut Ui) {
    let preconditions = (action.preconditions)(state);
    let cost = (action.cost)(state);
    let effect = (action.effect)(state.clone());

    CollapsingHeader::new(&action.key)
        .default_open(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(&action.key);
            });

            ui.horizontal(|ui| {
                ui.label("Preconditions");
                ui.label(&preconditions.to_string());
            });

            ui.horizontal(|ui| {
                ui.label("Cost");
                ui.label(&cost.to_string());
            });

            CollapsingHeader::new("Effect")
                .default_open(false)
                .show(ui, |ui| {
                    effect.show_egui(names, ui);
                });
        });
}