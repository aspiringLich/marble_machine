use std::{fs::File, io::Write};

use bevy::tasks::IoTaskPool;
use serde::{ Serialize, Deserialize };

use crate::{
    *,
    modules::{ ModuleType, SpawnInstructions, BodyType, ModuleComponent, Module },
    engine::module_state::ModuleState,
};
use ron::ser::PrettyConfig;

pub struct SaveWorld(pub String);

#[derive(Serialize, Deserialize, Debug)]
struct ModuleInfo {
    instructions: SpawnInstructions,
    module: Box<dyn Module>,
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

    let mut instructions: Vec<SpawnInstructions> = vec![];
    
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
        instructions.push(instruction);
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            #[allow(unused_assignments)]
            let mut serialized = ron::ser::to_string(&instructions).unwrap();
            #[cfg(debug_assertions)]
            serialized = ron::ser::to_string_pretty(&instructions, PrettyConfig::default()).unwrap();
            
            // Write the scene RON data to file
            let ret = File::create(format!("data/saves/{path}.ron"))
                .and_then(|mut file| file.write(serialized.as_bytes()));
            if let Err(e) = ret {
                error!("Failed to save scene: {e}")
            }
        })
        .detach();
}

pub struct LoadWorld(pub String);

pub fn load_world() {
    
}