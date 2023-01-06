use std::{f32::consts, f64};

use crate::{
    module::{param::ModuleResources, Module},
    spawn::SpawnModule,
    *,
};
use bevy::prelude::Image;
use bevy::prelude::Vec2;
use bevy_egui::*;
use egui::*;

/// stores the selected entities
#[derive(Resource, Debug, Default, Hash)]
pub struct SelectedModules {
    /// the entity (TODO: entities) we have selected
    pub selected: Option<Entity>,
    /// whether we are selecting it or placing it
    pub place: bool,
}

impl SelectedModules {
    pub fn from_entity(entity: Entity) -> Self {
        Self {
            selected: Some(entity),
            place: false,
        }
    }

    pub fn place_entity(entity: Entity) -> Self {
        Self {
            selected: Some(entity),
            place: true,
        }
    }

    pub fn clear_selected(&mut self) {
        self.selected = None;
    }
}

pub fn inspector_ui(
    mut egui_context: ResMut<EguiContext>,
    mut res: ModuleResources,
    selected: Res<SelectedModules>,
) {
    if selected.place {
        return;
    };
    let Some(selected) = selected.selected else { return };
    // im not sure how else to make the borrow checker shut up so
    // im pretty sure this isnt actually unsafe buuuuut
    // youre welcome future me if i just shot myself in the foot
    let res = &mut res as *mut ModuleResources;

    let binding = unsafe { &mut *res };
    let mut binding = binding.q_module_type.get_mut(selected).unwrap();
    let module = binding.get_inner();

    egui::Window::new(module.get_name())
        .resizable(true)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            module.interactive(unsafe { &mut *res }, ui, selected)
        });

    // println!("{}", window.unwrap().response.rect.width());
}

type LayoutFn<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

#[non_exhaustive]
pub struct Layout<'a> {
    main: Vec<LayoutFn<'a>>,
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
            .custom_formatter(|n, _| format!("{:>5.1}°", n.rem_euclid(360.0)));
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

#[derive(Resource)]
pub struct SpawningUiImages {
    basic: Handle<Image>,
}

impl FromWorld for SpawningUiImages {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            basic: asset_server.load("module_pictures/basic.png"),
        }
    }
}

/// creates the spawning ui where you can spawn stuff
pub fn spawning_ui(
    mut egui_context: ResMut<EguiContext>,
    images: Res<SpawningUiImages>,
    mut spawn_module: EventWriter<SpawnModule>,
) {
    let basic = egui_context.add_image(images.basic.clone());

    egui::Window::new("Le epic temp Module Spawner thingyyy")
        .resizable(true)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            if ui
                .add(ImageButton::new(basic, [100.0, 100.0]))
                .on_hover_text("The first one")
                .clicked()
            {
                spawn_module
                    .send(SpawnModule::new(ModuleType::Basic(module::Basic::default())).place());
            }
        });
}

/// creates the master debug ui thing
pub fn debug_ui(
    mut egui_context: ResMut<EguiContext>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut q_pancam: Query<&mut PanCam>,
    mut step: Local<bool>,
    mut prev_pancam: Local<Option<PanCam>>,
) {
    let active = &mut rapier_config.physics_pipeline_active;
    if *step {
        *active = false;
        *step = false;
    }

    egui::Window::new("debug ui thing")
        .resizable(true)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Physics Pipeline");
                if ui
                    .button([" ⏵ ", " ⏸ "][*active as usize])
                    .on_hover_text("Start / Stop physics")
                    .clicked()
                {
                    *active = !*active;
                }
                if ui.button(" step ").on_hover_text("Step").clicked() {
                    *active = true;
                    *step = true;
                }
            });

            // transfer the field from b to a and set b.field to default()
            macro transfer {
                ($a:ident, $b:ident, $field:ident) => {
                    $a.$field = $b.$field;
                    $b.$field = default();
                },
                ($a:ident, $b:ident, $field:ident, $($tail:tt)*) => {
                    $a.$field = $b.$field;
                    $b.$field = default();
                    transfer!($a, $b, $($tail)*)
                }
            }

            if prev_pancam.is_some() {
                if ui.button("Re-Lock Pancam").clicked() {
                    let prev = prev_pancam.as_mut().unwrap();
                    let mut pancam = q_pancam.single_mut();

                    transfer!(pancam, prev, min_x, min_y, max_x, max_y, max_scale);
                    *prev_pancam = None;
                }
            } else {
                if ui.button("Unlock Pancam").clicked() {
                    *prev_pancam = Some(PanCam::default());

                    let prev = prev_pancam.as_mut().unwrap();
                    let mut pancam = q_pancam.single_mut();

                    transfer!(prev, pancam, min_x, min_y, max_x, max_y, max_scale);
                }
            }
        });
}
