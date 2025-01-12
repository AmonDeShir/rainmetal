use bevy_inspector_egui::bevy_egui::{egui::{self, CollapsingHeader}, EguiContexts};
use bevy::prelude::*;
use super::{ItemList, ItemListHandle};

#[derive(Resource)]
pub struct InspectorState {
    pub default_show: bool
}


pub fn ui_init(mut commands: Commands) {
    commands.insert_resource(InspectorState {  default_show: true });
}


pub fn ui_show_items(mut contexts: EguiContexts, mut state: ResMut<InspectorState>, items_handler: Res<ItemListHandle>, assets: Res<Assets<ItemList>>) {
    egui::Window::new("All Items").default_open(false).show(contexts.ctx_mut(), |ui| {
        let Some(car) = assets.get(&items_handler.0) else {
            ui.label("Loading items.ron...");

            return;
        };
        
        let checkbox_changed = ui.checkbox( &mut state.default_show, "Show").changed();


        for (name, item) in &car.items {

            CollapsingHeader::new(name)
                .default_open(state.default_show)
                .open(if !checkbox_changed { None } else { Some(state.default_show)})
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.label(&item.name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Price: ");
                        ui.label(item.price.to_string());
                    });
                });
        }
    });
}