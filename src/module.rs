use bevy::ecs::component::TableStorage;
use bevy::prelude::*;
use bevy::utils::Instant;

use crate::atlas::AtlasDictionary;
use crate::spawn::SpawnInstruction::{self, *};
use crate::*;
use marble::Marble;

use std::f32::consts::*;

#[derive(Component)]
pub struct InputState {
    inputs: Vec<Option<(Marble, Instant)>>,
}

#[derive(Copy, Clone)]
pub enum ModuleType {
    Basic,
}

impl ModuleType {
    pub fn get(self) -> impl Module<Storage = TableStorage> {
        match self {
            Self::Basic => Basic,
        }
    }
}

pub trait Module
where
    Self: Component<Storage = TableStorage> + Clone,
{
    /// return instructions on spawning this module
    fn spawn_instructions(&self) -> &Vec<SpawnInstruction>;
}

#[derive(Component, Clone)]
pub struct Basic;

static BASIC_SPAWN_INSTRUCTIONS: Lazy<Vec<SpawnInstruction>> = Lazy::new(|| {
    use SpawnInstruction::*;
    vec![BodySmall(vec![0.0], vec![PI])]
});

impl Module for Basic {
    fn spawn_instructions(&self) -> &Vec<SpawnInstruction> {
        use SpawnInstruction::*;
        &*BASIC_SPAWN_INSTRUCTIONS
    }
}
