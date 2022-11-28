use bevy::ecs::component::TableStorage;
use bevy::ecs::entity::MapEntities;
use bevy::ecs::system::assert_is_system;
use bevy::ecs::system::SystemParam;
use bevy::utils::Instant;
use bevy_egui::*;

use crate::misc::ChildrenMatches;
use crate::spawn::SpawnInstruction;
use crate::spawn::SpawnMarble;
use crate::ui::UiElements;
use crate::*;
use marble::Marble;

use std::f32::consts::*;

#[derive(Component)]
pub struct InputState {
    inputs: Vec<Option<(Marble, Instant)>>,
}

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

#[derive(Copy, Clone)]
pub struct Basic {
    pub input_rot: f32,
    pub output_rot: f32,
}

impl Default for Basic {
    fn default() -> Self {
        Basic {
            input_rot: 0.0,
            output_rot: PI,
        }
    }
}

impl Module for Basic {
    fn spawn_instructions(&self) -> Vec<SpawnInstruction> {
        use SpawnInstruction::*;

        vec![BodySmall(vec![self.input_rot], vec![self.output_rot])]
    }

    fn gui(&mut self, res: &mut ModuleResources, ui: &mut egui::Ui, entity: Entity) {
        let children = res.get_children.get(entity).unwrap();

        let input = children.get_matching(&res.get_input).next().unwrap();
        let input_transform = res.get_transform.get_mut(input).unwrap();

        let output = children.get_matching(&res.get_output).next().unwrap();
        let output_transform = res.get_transform.get_mut(output).unwrap();

        ui.angle_slider(res, "eeee", &mut self.input_rot);
    }

    const NAME: &'static str = "Basic Module";
}
