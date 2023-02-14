use bevy::prelude::Entity;
use bevy_egui::egui::Ui;

use crate::engine::{marble::Marble, marble_io::FireMarble};

use super::header::ModuleResources;

pub fn default_debug_ui(ui: &mut Ui, res: &mut ModuleResources, module: Entity) {
    let outputs: Vec<_> = res.outputs(module).collect();
    if ui.button("Fire Marble!").clicked() {
        res.fire_marble.send(FireMarble {
            marble: Marble::Bit { value: true },
            from: outputs[0],
            power: 1.0,
        })
    }
}
