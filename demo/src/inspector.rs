use bevy::utils::HashMap;
use bevy_inspector_egui::egui::{CollapsingHeader, Ui};

pub fn show_item_list<T: ToString>(title: &str, items: &HashMap<String, T>, ui: &mut Ui) {
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
