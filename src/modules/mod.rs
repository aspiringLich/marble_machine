pub mod body;
pub use body::*;

use std::time::Duration;

pub mod instructions;
pub use instructions::*;

pub mod defs;
pub use defs::*;

use bevy::{ prelude::*, ecs::system::SystemParam };
use bevy_egui::egui::Ui;
use trait_enum::DerefMut;
use derive_more::{ Deref, DerefMut };

use crate::{
    engine::{marble_io::{ self, FireMarble }, marble::Marble},
    misc::marker,
    query::{ QueryOutput, QueryQuerySimple, QueryQueryIter },
};

pub struct ModuleInfo {
    instructions: SpawnInstructions,
    name: &'static str,
    identifier: &'static str,
}

pub trait Module {
    fn info(&self) -> ModuleInfo;
    /// function that runs to update this module
    fn update(&mut self, res: &mut ModuleResources, module: Entity);
    fn callback_update(&mut self, res: &mut ModuleResources, module: Entity);

    fn debug_ui(&mut self, ui: &mut Ui, res: &mut ModuleResources, module: Entity) {
        let outputs: Vec<_> = res.outputs(module).collect();
        if ui.button("Fire Marble!").clicked() {
            res.fire_marble.send(FireMarble {
                marble: Marble::Bit { value: true },
                from: outputs[0],
                power: 1.0,
            })
        }
    }
}

type QuerySimple<'w, 's, T> = Query<'w, 's, &'static mut T>;
// type QueryWith<'w, 's, T, W> = Query<'w, 's, &'static mut T, bevy::prelude::With<W>>;
type QueryEntity<'w, 's, W> = Query<'w, 's, bevy::prelude::Entity, bevy::prelude::With<W>>;

// information the modules get to mess around with
#[derive(SystemParam)]
pub struct ModuleResources<'w, 's> {
    pub commands: Commands<'w, 's>,
    // simple queries
    pub q_name: QuerySimple<'w, 's, Name>,
    pub q_module_type: QuerySimple<'w, 's, ModuleType>,
    pub q_input_state: QuerySimple<'w, 's, marble_io::InputState>,
    pub q_transform: QuerySimple<'w, 's, Transform>,
    pub q_children: QuerySimple<'w, 's, Children>,
    pub q_sprite: QuerySimple<'w, 's, Sprite>,
    // entity queries
    pub w_input: QueryEntity<'w, 's, marker::Input>,
    pub w_output: QueryEntity<'w, 's, marker::Output>,
    pub w_indicator: QueryEntity<'w, 's, marker::Indicator>,
    // events
    pub fire_marble: EventWriter<'w, 's, FireMarble>,
    // resources
    pub keyboard: Res<'w, bevy::prelude::Input<KeyCode>>,
}

impl<'w, 's> ModuleResources<'w, 's> {
    #[must_use]
    pub fn inputs(
        &'w self,
        module: Entity
    ) -> QueryOutput<impl Iterator<Item = bevy::prelude::Entity> + 'w> {
        self.q_children.entity(module).iter().with(&self.w_input)
    }

    #[must_use]
    pub fn outputs(
        &'w self,
        module: Entity
    ) -> QueryOutput<impl Iterator<Item = bevy::prelude::Entity> + 'w> {
        self.q_children.entity(module).iter().with(&self.w_output)
    }

    /// update the indicator lights for the inputs to show if theyre full or not
    pub fn update_input_indicators(&mut self, module: Entity) {
        let input_states = self.q_input_state.entity(module).iter();
        let inputs = self.inputs(module);

        for (input, input_state) in inputs.zip(input_states) {
            let indicator_sprite = &mut self.q_children
                .entity(input)
                .iter()
                .with(&self.w_indicator)
                .query_collect_mut(&self.q_sprite)[0];

            let color = &mut indicator_sprite.color;
            let hsla = color.as_hsla_f32();
            let hue = [117.0, 0.0][input_state.is_some() as usize];
            let new_color = Color::hsla(hue, hsla[1], hsla[2], hsla[3]);
            *color = new_color;
        }
    }
}


/// "i want to do something after x second(s) pls help"
#[derive(Deref, DerefMut, Component)]
pub struct ModuleCallbackTimer(Timer);

impl ModuleCallbackTimer {
    pub fn new(ticks: u32) -> Self {
        ModuleCallbackTimer(Timer::from_seconds(ticks as f32, TimerMode::Once))
    }
}

pub fn update_module_callbacks(
    mut set: ParamSet<(ModuleResources, Query<(&mut ModuleType, Entity, &mut ModuleCallbackTimer)>)>
) {
    // no mutability conflict as they conflict because the query gets ModuleType mutably
    // and im passing in ModuleResources into the ModuleType we get, so
    // shhhhhhh
    let res_module = &set.p0() as *const ModuleResources as *mut ModuleResources;
    let res_module = unsafe { &mut *res_module };
    for (mut module, entity, mut timer) in set.p1().iter_mut() {
        // tick once
        timer.tick(Duration::from_secs(1));

        if timer.finished() {
            module.callback_update(res_module, entity);
            res_module.commands.entity(entity).remove::<ModuleCallbackTimer>();
        }
    }
}

/// tells this entity that they need to be updated (!!! (!!!)) (probably a module)
#[derive(Deref, DerefMut)]
pub struct UpdateModule(pub Entity);

/// run the update functions for the modules!!
#[allow(clippy::type_complexity)]
pub fn update_modules(
    mut set: ParamSet<
        (
            ModuleResources,
            Query<(&mut ModuleType, Entity), Changed<marble_io::InputState>>,
            // Query<(&mut ModuleType, Entity)>,
        )
    >
) {
    // no mutability conflict as they conflict because the query gets ModuleType mutably
    // and im passing in ModuleResources into the ModuleType we get, so
    // shhhhhhh
    let res_module = &set.p0() as *const ModuleResources as *mut ModuleResources;
    for (mut module, entity) in set.p1().iter_mut() {
        module.deref_mut().update(unsafe { &mut *res_module }, entity);
    }
}