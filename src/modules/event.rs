use bevy::{ prelude::*, ecs::system::SystemParam };

use crate::{
    engine::{ marble::Marble, module_state::ModuleState, marble_io::FireMarbleEvent },
    modules::ModuleCallbackTimer,
};

use super::QuerySimple;

pub struct ModuleEvent {
    affecting: Entity,
    update: ModuleUpdate,
}

pub struct ModuleEventSender<'w, 's> {
    event_writer: EventWriter<'w, 's, ModuleEvent>,
    entity: Option<Entity>,
}

impl<'w, 's> ModuleEventSender<'w, 's> {
    pub fn new(event_writer: EventWriter<'w, 's, ModuleEvent>) -> Self {
        Self { event_writer, entity: None }
    }

    pub fn entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn send(&mut self, update: ModuleUpdate) {
        self.event_writer.send(ModuleEvent {
            affecting: self.entity.expect("dont forget to initialize the entityyy"),
            update,
        });
    }
}

/// Things to change about this module
pub enum ModuleUpdate {
    FireMarble(Marble),
    ChangeIndicatorColor(f32),
    UpdateIndicatorColors,
    Callback(f32),
}

#[derive(SystemParam)]
pub struct Queries<'w, 's> {
    module_state: QuerySimple<'w, 's, ModuleState>,
    sprite: QuerySimple<'w, 's, Sprite>,
}

pub fn do_module_events(
    mut commands: Commands,
    mut events: EventReader<ModuleEvent>,
    mut query: Queries,
    mut marble_event: EventWriter<FireMarbleEvent>
) {
    for event in events.iter() {
        let entity = event.affecting;
        use ModuleUpdate::*;
        let state = query.module_state.get(entity).unwrap();

        match event.update {
            FireMarble(marble) => {
                marble_event.send(
                    crate::engine::marble_io::FireMarbleEvent::new(marble, state.outputs[0], 1.0)
                );
            }
            ChangeIndicatorColor(_) => todo!(),
            UpdateIndicatorColors => update_indicator_colors(state, &mut query.sprite),
            Callback(ticks) => {
                commands.entity(entity).insert(ModuleCallbackTimer::new(ticks));
            }
        }
    }
}

fn update_indicator_colors(state: &ModuleState, q_sprite: &mut Query<&mut Sprite>) {
    for (i, input_state) in state.input_state.iter().enumerate() {
        let mut sprite = q_sprite.get_mut(state.indicators[i]).unwrap();

        let hsla = sprite.color.as_hsla_f32();
        let hue = [117.0, 0.0][input_state.is_some() as usize];
        let new_color = Color::hsla(hue, hsla[1], hsla[2], hsla[3]);
        sprite.color = new_color;
    }
}