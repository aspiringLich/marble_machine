use std::{ fs::File, io::Write };

use bevy::tasks::IoTaskPool;
use serde::{ Serialize, Deserialize };

use crate::{
    *,
    modules::{ ModuleType, SpawnInstructions, ModuleComponent, Module },
    engine::{module_state::ModuleState, spawn::SpawnModule},
};

pub struct SaveWorld(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleInfo {
    pub instructions: SpawnInstructions,
    pub module: Box<dyn Module>,
    pub module_type: ModuleType,
    pub offset: Vec3,
}

impl ModuleInfo {
    pub fn new(module: ModuleType) -> Self {
        Self {
            module: module.get_module(),
            instructions: module.spawn_instructions().clone(),
            module_type: module,
            offset: Vec3::ZERO,
        }
    }
}

pub fn save_world(
    q_modules: Query<Entity, With<marker::Module>>,
    q_module: Query<&ModuleComponent>,
    q_state: Query<&ModuleState>,
    mut save_events: EventReader<SaveWorld>,
    q_transform: Query<&Transform>
) {
    let Some(SaveWorld(path)) = save_events.iter().next() else {
        return;
    };
    let path = path.clone();

    let mut instructions: Vec<ModuleInfo> = vec![];

    for module in q_modules.iter() {
        let component = q_module.get(module).unwrap();
        let state = q_state.get(module).unwrap();
        let mut instruction = component.ty.spawn_instructions().clone();

        for (i, input) in state.inputs.iter().enumerate() {
            let rot = q_transform.get(*input).unwrap().rotation;
            instruction.inputs[i].rotation = rot.to_euler(EulerRot::XYZ).2;
        }
        for (i, output) in state.outputs.iter().enumerate() {
            let rot = q_transform.get(*output).unwrap().rotation;
            instruction.outputs[i].rotation = rot.to_euler(EulerRot::XYZ).2;
        }
        instructions.push(ModuleInfo {
            instructions: instruction,
            module: component.module.clone(),
            module_type: component.ty,
            offset: q_transform.get(module).unwrap().translation,
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // #[allow(unused_assignments)]
            let serialized = ron::ser::to_string(&instructions).unwrap();
            // #[cfg(debug_assertions)]
            // serialized = ron::ser
            //     ::to_string_pretty(&instructions, PrettyConfig::default())
            //     .unwrap();

            // Write the scene RON data to file
            let ret = File::create(format!("data/saves/{path}.ron")).and_then(|mut file|
                file.write(serialized.as_bytes())
            );
            if ret.is_err() {
                error!("Failed to save world to {path}")
            }
        })
        .detach();
} 

pub struct LoadWorld(pub String);

pub fn load_world(
    q_modules: Query<Entity, With<marker::Module>>,
    mut load_events: EventReader<LoadWorld>,
    mut commands: Commands,
    mut spawn_events: EventWriter<SpawnModule>,
) {
    let Some(LoadWorld(path)) = load_events.iter().next() else {
        return;
    };
    let path = path.clone();

    for module in q_modules.iter() {
        commands.entity(module).despawn_recursive();
    }

    let serialized = std::fs::read_to_string(format!("data/saves/{path}.ron")).unwrap();
    let Ok(instructions) = ron::de::from_str::<Vec<ModuleInfo>>(&serialized) else {
        error!("Failed to load world from {path}");
        return;
    };
    
    for module in instructions {
        spawn_events.send(SpawnModule::new(module));
    }
}