use std::cell::UnsafeCell;

use crate::{
    module::{Module, ModuleResources},
    *,
};
use bevy_egui::*;

#[derive(Resource)]
pub struct SelectedModule {
    pub selected: Option<Entity>,
}

impl Default for SelectedModule {
    fn default() -> Self {
        Self { selected: None }
    }
}

pub fn inspector_ui(
    mut egui_context: ResMut<EguiContext>,
    mut res: ModuleResources,
    selected: Res<SelectedModule>,
) {
    let Some(selected) = selected.selected else { return };

    // im not sure how else to make the borrow checker shut up so
    // im pretty sure this isnt actually unsafe buuuuut
    // youre welcome future me if i just shot myself in the foot
    let res = &mut res as *mut ModuleResources;

    let binding = unsafe { &mut *res };
    let mut binding = binding.get_module_type.get_mut(selected).unwrap();
    let module = binding.get_inner();

    egui::Window::new("w").show(egui_context.ctx_mut(), |ui| {
        module.gui(unsafe { &mut *res }, ui, selected);
    });
}

pub trait UiElements {
    fn get(&mut self) -> &mut egui::Ui;

    fn angle_slider(&mut self, res: &mut ModuleResources, label: &str, angle: &mut f32) {
        let ui = self.get();
        ui.horizontal(|ui| {
            ui.label(label);
            let response = ui.add(
                egui::Slider::new(angle, 0.0..=360.0)
                    .text("angle")
                    .step_by(1.0)
                    .suffix("°"),
            );
        });
    }
}

impl UiElements for egui::Ui {
    fn get(&mut self) -> &mut egui::Ui {
        self
    }
}
