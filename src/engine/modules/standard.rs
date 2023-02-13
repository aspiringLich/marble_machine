use crate::{engine::modules::BodyType, *};
use bevy_egui::egui::Ui;

use crate::{
    engine::{marble::Marble, marble_io::FireMarble},
    query::{QueryQueryIter, QueryQuerySimple},
};

use super::{
    header::{Module, ModuleCallbackTimer, ModuleResources},
    SpawnInstructions,
};

#[derive(Copy, Clone)]
pub struct Basic;

impl Default for Basic {
    fn default() -> Self {
        Basic
    }
}

impl Module for Basic {
    fn spawn_instructions(&self) -> SpawnInstructions {
        SpawnInstructions::from_body(BodyType::Small)
            .with_input_rotations([-180.0].into_iter())
            .with_output_rotations([0.0].into_iter())
    }

    fn update(&mut self, res: &mut ModuleResources, module: Entity) {
        res.update_input_indicators(module);
        let input_state = res.q_input_state.entity(module);

        if input_state[0].is_some() {
            res.commands
                .entity(module)
                .insert(ModuleCallbackTimer::new(10));
        }
    }

    fn callback_update(&mut self, res: &mut ModuleResources, module: Entity) {
        let mut input_state = res.q_input_state.entity_mut(module);

        // if theres a marble in there (there should be)
        if let Some(marble) = input_state[0] {
            let outputs = res
                .q_children
                .entity(module)
                .iter()
                .with_collect(&res.w_output);

            // fire it outta the input and mark that the input is empty
            res.fire_marble
                .send(FireMarble::new(marble, outputs[0], 1.0));
            input_state[0] = None;
        } else {
            self.log_warn("expected marble in input state", module);
        }
        res.update_input_indicators(module);
    }

    fn get_name(&self) -> &'static str {
        "Basic Module"
    }

    fn get_identifier(&self) -> &'static str {
        "basic.module"
    }
}
