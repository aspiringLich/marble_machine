use std::{f32::consts, f64};

use crate::{
    module::{Module, ModuleResources},
    *,
};
use bevy_egui::*;
use egui::*;

#[derive(Resource, Debug)]
pub struct SelectedModules(pub Option<Entity>);

impl Default for SelectedModules {
    fn default() -> Self {
        Self(None)
    }
}

impl std::ops::Deref for SelectedModules {
    type Target = Option<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init_egui_context(mut egui_context: ResMut<EguiContext>) {
    // let ctx = egui_context.ctx_mut();
    // ctx.set_visuals(egui::Visuals::light());
    // ctx.set_pixels_per_point(20.0);
}

pub fn inspector_ui(
    mut egui_context: ResMut<EguiContext>,
    mut res: ModuleResources,
    selected: Res<SelectedModules>,
) {
    let Some(selected) = **selected else { return };
    // dbg!(egui_context.ctx_mut().pixels_per_point());

    // im not sure how else to make the borrow checker shut up so
    // im pretty sure this isnt actually unsafe buuuuut
    // youre welcome future me if i just shot myself in the foot
    let res = &mut res as *mut ModuleResources;

    let binding = unsafe { &mut *res };
    let mut binding = binding.get_module_type.get_mut(selected).unwrap();
    let module = binding.get_inner();

    // let window =
    egui::Window::new(module.get_name())
        .resizable(true)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.spacing_mut().slider_width = 300.0;
            module.gui(unsafe { &mut *res }, ui, selected);
        });

    // println!("{}", window.unwrap().response.rect.width());
}

pub trait UiElements {
    fn get(&mut self) -> &mut Ui;

    /// a slider to modify a float from 0-360 deg
    fn angle_slider(&mut self, label: &str, angle: &mut f32) {
        let ui = self.get();

        macro create_button($text:expr, $hover_text:expr, $($tail:tt)*) {
            if ui.small_button($text).on_hover_text($hover_text).clicked() {
                $($tail)*
            }
        }
        let step_amt = consts::TAU / 72.0;
        let mut deg_angle = angle.to_degrees();

        ui.label(label);
        // ui.add(
        //     Slider::new(angle, 0.0..=consts::TAU)
        //         .step_by(step_amt as f64)
        //         .show_value(false),
        // );

        create_button!("<", "Rotate left 2.5°", *angle -= step_amt / 2.0);
        let drag = DragValue::new(&mut deg_angle)
            .speed(1.0)
            .custom_formatter(|n, _| format!("{:>5.1}°", n));
        if ui.add(drag).changed() {
            *angle = deg_angle.to_radians()
        }
        create_button!(
            "~",
            "Round to the nearest 45°",
            *angle = (*angle * 8.0 / consts::TAU).round() * consts::TAU / 8.0
        );
        create_button!(">", "Rotate right 2.5°", *angle += step_amt / 2.0);
        ui.end_row();

        *angle = f32::rem_euclid(*angle, consts::TAU);
    }
}

impl UiElements for Ui {
    fn get(&mut self) -> &mut Ui {
        self
    }
}
