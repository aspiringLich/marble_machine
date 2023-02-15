use bevy::prelude::{Entity, Component};
use super::marble::Marble;



/// a structure attatched to the parent entity of a module 
/// that holds the state of the module
#[derive(Component)]
pub struct ModuleState {
    /// the inputs entities, children being the sprite
    pub inputs: Vec<Entity>,
    /// the indicators
    pub indicators: Vec<Entity>,
    /// the output entities, children being the sprite
    pub outputs: Vec<Entity>,
    /// the decal entities
    pub decals: Vec<Entity>,
    /// the body entity
    pub body: Entity,
    /// the state of all the inputs
    pub input_state: Vec<Option<Marble>>,
}

impl Default for ModuleState {
    fn default() -> Self {
        ModuleState {
            inputs: Vec::new(),
            indicators: Vec::new(),
            outputs: Vec::new(),
            decals: Vec::new(),
            body: unsafe { std::mem::zeroed() },
            input_state: Vec::new(),
        }
    }
}