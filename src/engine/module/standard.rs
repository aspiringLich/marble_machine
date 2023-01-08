use bevy::prelude::{warn, Entity};
use bevy_egui::egui::Ui;
use ctor::ctor;

use crate::{
    engine::{
        marble::Marble, marble_io::FireMarble, module::body::BodyType, spawn::SpawnInstructions,
    },
    query::{QueryQueryIter, QueryQuerySimple},
};

use super::header::{Module, ModuleCallbackTimer, ModuleResources};

#[derive(Copy, Clone)]
pub struct Basic;

impl Default for Basic {
    fn default() -> Self {
        Basic
    }
}

#[ctor]
static BASIC_INSTRUCTIONS: SpawnInstructions = {
    SpawnInstructions::from_body(BodyType::Small)
        .with_input_rotations([270.].into_iter())
        .with_output_rotations([90.].into_iter())
};

impl Module for Basic {
    fn spawn_instructions(&self) -> &'static SpawnInstructions {
        &BASIC_INSTRUCTIONS
    }

    fn update(&mut self, res: &mut ModuleResources, module: Entity) {
        // update indicators
        res.update_input_indicators(module);

        let ModuleResources {
            commands,
            q_input_state,
            ..
        } = res;

        let input_state = q_input_state.entity(module);
        // if we have a marble then activate the callback
        if input_state[0].is_some() {
            commands.entity(module).insert(ModuleCallbackTimer::new(10));
        }
    }

    fn callback_update(&mut self, res: &mut ModuleResources, module: Entity) {
        let mut input_state = res.q_input_state.entity_mut(module);

        // if theres a marble in there (there should be)
        if let Some(marble) = input_state[0] {
            let ModuleResources {
                fire_marble,
                w_output,
                q_children,
                ..
            } = res;
            let outputs = q_children.entity(module).iter().with_collect(w_output);

            // fire it outta the input and mark that the input is empty
            fire_marble.send(FireMarble::new(marble, outputs[0], 1.0));
            input_state[0] = None;
        } else {
            warn!(
                "callback_update on {}: expected marble in input state",
                self.get_name()
            )
        }
        res.update_input_indicators(module);
    }

    fn interactive(&mut self, res: &mut ModuleResources, ui: &mut Ui, module: Entity) {
        // let inputs: Vec<_> = res.inputs(module).collect();
        let outputs: Vec<_> = res.outputs(module).collect();

        let ModuleResources {
            fire_marble: spawn_marble,
            // q_transform,
            // keyboard,
            ..
        } = &mut *res;
        // let input_tfs = inputs.query_collect_mut(q_transform);
        // let output_tfs = outputs.query_collect_mut(q_transform);

        // ui::Layout::new()
        //     .default_rotation_sliders(input_tfs, output_tfs, &body_small_transform)
        //     .build(ui);

        // cool epic le hacker debug button
        if ui.button("Fire Marble!").clicked() {
            spawn_marble.send(FireMarble {
                marble: Marble::Bit { value: true },
                from: outputs[0],
                power: 1.0,
            })
        }
    }

    fn get_name(&self) -> &'static str {
        "Basic Module"
    }

    fn get_identifier(&self) -> &'static str {
        "basic.module"
    }
}
