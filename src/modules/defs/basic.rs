use super::*;

#[derive(Copy, Clone)]
pub struct Basic;

impl Default for Basic {
    fn default() -> Self {
        Basic
    }
}

impl Module for Basic {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            instructions: SpawnInstructions::from_body(BodyType::Small)
                .with_input_rotations([-180.0].into_iter())
                .with_output_rotations([0.0].into_iter()),
            name: "Basic Module",
            identifier: "basic.module",
        }
    }

    fn update(&mut self, res: &mut ModuleResources, module: Entity) {
        res.update_input_indicators(module);
        let input_state = res.q_input_state.entity(module);

        if input_state[0].is_some() {
            res.commands.entity(module).insert(ModuleCallbackTimer::new(10));
        }
    }

    fn callback_update(&mut self, res: &mut ModuleResources, module: Entity) {
        let mut input_state = res.q_input_state.entity_mut(module);

        // if theres a marble in there (there should be)
        if let Some(marble) = input_state[0] {
            let outputs = res.q_children.entity(module).iter().with_collect(&res.w_output);

            // fire it outta the input and mark that the input is empty
            res.fire_marble.send(FireMarble::new(marble, outputs[0], 1.0));
            input_state[0] = None;
        } else {
            warn!("expected marble in input state");
        }
        res.update_input_indicators(module);
    }
}