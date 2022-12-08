use std::{f32::consts, f64};

use crate::{
    module::{param::ModuleResources, Module},
    *,
};
use bevy::prelude::Vec2;
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
    let mut binding = binding.q_module_type.get_mut(selected).unwrap();
    let module = binding.get_inner();

    // let window =
    egui::Window::new(module.get_name())
        .resizable(true)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.spacing_mut().slider_width = 300.0;
            module.gui(unsafe { &mut *res }, ui, selected)
        });

    // println!("{}", window.unwrap().response.rect.width());
}

#[non_exhaustive]
pub struct Layout<'a> {
    main: Vec<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

impl<'a> Layout<'a> {
    /// create a new layout with everything blank
    pub fn new() -> Self {
        Self { main: vec![] }
    }

    /// make default rotation sliders
    /// so one slider for every input and output
    /// TODO: rotate everything on q and r
    pub fn default_rotation_sliders<I, J, T>(mut self, i: I, o: I, transform_fn: &'a T) -> Self
    where
        I: IntoIterator<Item = J> + 'a,
        J: AsMut<Transform> + 'a,
        T: Fn(f32) -> Transform + 'static,
    {
        self.main.push(Box::new(|ui: &mut Ui| {
            // add sliders, if theres only one of the type dont add a # to the label
            // so like "Input" or "Input #1" and "Input #2"
            let mut make_sliders = |transforms: I, name: &str| {
                let mut make_slider = |label: String, transform: &mut Transform| {
                    ui.angle_slider_transform(&label, transform, transform_fn)
                };
                let mut transforms: Vec<J> = transforms.into_iter().collect();
                let len = transforms.len();
                let name_fn = |name: &str, i| {
                    if len == 1 {
                        name.to_string()
                    } else {
                        format!("{} #{}", name, i)
                    }
                };
                for (i, transform) in transforms.iter_mut().enumerate() {
                    make_slider(name_fn(name, i), transform.as_mut())
                }
            };
            make_sliders(i, "Input");
            make_sliders(o, "Output");
        }));
        self
    }

    /// build the ui
    pub fn build(self, ui: &mut Ui) {
        for build_fn in self.main {
            egui::Grid::new("main")
                .min_col_width(0.0)
                .striped(true)
                .show(ui, build_fn);
        }
    }
}

pub trait UiElements {
    fn get(&mut self) -> &mut Ui;

    fn angle_slider_transform<T>(
        &mut self,
        label: &str,
        transform: &mut Transform,
        transform_fn: &T,
    ) where
        T: Fn(f32) -> Transform + 'static,
    {
        let mut rot: Vec3 = transform.rotation.to_euler(EulerRot::XYZ).into();
        let mut angle = rot.z;
        self.angle_slider(label, &mut angle);

        if angle != rot.z {
            *transform = transform_fn(angle);
        }
    }

    /// A slider to modify an angle in radians, displays it in a more readable format (degrees)
    /// and has some buttons to modify it further
    ///
    /// ```text
    /// [<] [<~] [100.0°] [~>] [>]
    /// ```
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

        create_button!(
            "<",
            "Rotate counter-clockwise 2.5°",
            *angle += step_amt / 2.0
        );
        let drag = DragValue::new(&mut deg_angle)
            .speed(1.0)
            .custom_formatter(|n, _| format!("{:>5.1}°", n + 180.0));
        create_button!(
            "<~",
            "Rotate counter-clockwise with a 45° step",
            *angle = ((*angle + 0.01) * 8.0 / consts::TAU).ceil() * consts::TAU / 8.0
        );
        if ui.add(drag).changed() {
            *angle = deg_angle.to_radians()
        }
        create_button!(
            "~>",
            "Rotate clockwise with a 45° step",
            *angle = ((*angle - 0.01) * 8.0 / consts::TAU).floor() * consts::TAU / 8.0
        );
        create_button!(">", "Rotate clockwise 2.5°", *angle -= step_amt / 2.0);
        ui.end_row();

        *angle = f32::rem_euclid(*angle, consts::TAU);
    }
}

impl UiElements for Ui {
    fn get(&mut self) -> &mut Ui {
        self
    }
}
