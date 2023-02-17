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
                .with_input_rotations([-180.0].into_iter(), 0.0)
                .with_output_rotations([0.0].into_iter(), 0.0),
            name: "Basic Module",
            identifier: "basic.module",
        }
    }

    fn update(&mut self, events: &mut ModuleEventSender, state: &mut ModuleState) {
        events.send(UpdateIndicatorColors);

        if state.input_state[0].is_some() {
            events.send(Callback(0.15));
        }
    }

    fn callback_update(&mut self, events: &mut ModuleEventSender, state: &mut ModuleState) {
        let input_state = &mut state.input_state;

        if let Some(marble) = input_state[0] {
            events.send(FireMarble(marble));
            input_state[0] = None;
            events.send(UpdateIndicatorColors);
        }
    }
}