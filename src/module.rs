use bevy::ecs::system::SystemParam;
use bevy_egui::*;

use crate::atlas::AtlasDictionary;
use crate::marble::Marble;
use crate::marble::SpawnMarble;
use crate::misc::ChildrenMatches;
use crate::spawn::SpawnInstruction;
use crate::ui::UiElements;
use crate::*;

#[derive(Copy, Clone, Component)]
pub enum ModuleType {
    Basic(Basic),
}

impl Default for ModuleType {
    fn default() -> Self {
        Self::Basic(default())
    }
}

impl ModuleType {
    pub fn get_inner<'a>(&'a mut self) -> &'a mut impl Module {
        match self {
            Self::Basic(x) => x,
        }
    }
}

/// information the modules get to mess around with
#[derive(SystemParam)]
pub struct ModuleResources<'w, 's> {
    pub commands: Commands<'w, 's>,
    // queries
    pub get_name: Query<'w, 's, &'static mut Name>,
    pub get_module_type: Query<'w, 's, &'static mut ModuleType>,
    pub get_transform: Query<'w, 's, &'static mut Transform>,
    pub get_children: Query<'w, 's, &'static Children>,
    pub get_input: Query<'w, 's, &'static marker::Input>,
    pub get_output: Query<'w, 's, &'static marker::Output>,
    // events
    pub spawn_marble: EventWriter<'w, 's, SpawnMarble>,
    // resources
}

pub trait Module {
    /// return instructions on spawning this module
    fn spawn_instructions(&self) -> Vec<SpawnInstruction>;
    /// function to regulate the gui and whatever
    fn gui(&mut self, res: &mut ModuleResources, ui: &mut egui::Ui, entity: Entity);
    /// the name of the module
    const NAME: &'static str;
    fn get_name(&self) -> &'static str {
        Self::NAME
    }
}

/// basically, imagine offsetting some object by `offset` in the x-axis, then rotating it around the origin `rotation` radians.
///
/// this is what this function does.
pub fn transform_from_offset_rotate(offset: f32, rotation: f32, z: f32) -> Transform {
    let rotation = Quat::from_rotation_z(rotation);
    let translation = rotation.mul_vec3(Vec3::X * offset) + Vec3::Z * z;
    Transform {
        rotation,
        translation,
        scale: Vec3::ONE,
    }
}

/// returns a transform that equates to a valid i/o position around a `body_small`.
pub fn body_small_transform(rotation: f32) -> Transform {
    transform_from_offset_rotate(basic::body_small.width() * 0.5 + 1.0, rotation, 0.25)
}

#[derive(Copy, Clone)]
pub struct Basic {
    pub input_rot: f32,
    pub output_rot: f32,
}

impl Default for Basic {
    fn default() -> Self {
        Basic {
            input_rot: 180.0,
            output_rot: 90.0,
        }
    }
}

impl Module for Basic {
    fn spawn_instructions(&self) -> Vec<SpawnInstruction> {
        use SpawnInstruction::*;

        vec![BodySmall(vec![self.input_rot], vec![self.output_rot])]
    }

    fn gui(&mut self, res: &mut ModuleResources, ui: &mut egui::Ui, entity: Entity) {
        ui.angle_slider("Input", &mut self.input_rot);
        ui.angle_slider("Output", &mut self.output_rot);

        let children = res.get_children.get(entity).unwrap();

        let input = children.get_matching(&res.get_input).next().unwrap();
        let mut input_transform = res.get_transform.get_mut(input).unwrap();
        *input_transform = body_small_transform(self.input_rot * misc::DEG_TO_RAD);

        let output = children.get_matching(&res.get_output).next().unwrap();
        let mut output_transform = res.get_transform.get_mut(output).unwrap();
        *output_transform = body_small_transform(self.output_rot * misc::DEG_TO_RAD);

        if ui.button("Fire Marble!").clicked() {
            res.spawn_marble.send(SpawnMarble {
                marble: Marble::Bit { value: true },
                from: output,
                power: 1.0,
            })
        }
    }

    const NAME: &'static str = "Basic Module";
}
