pub mod body;
use bevy_inspector_egui::egui::Ui;
pub use body::*;

pub mod instructions;
pub use instructions::*;

pub mod defs;
pub use defs::*;

mod event;

use std::time::Duration;
use bevy::{prelude::*, core::FrameCount};
use derive_more::{ Deref, DerefMut };

use crate::{ engine::{ marble_io, module_state::ModuleState, marble::Marble }, Label };

pub use self::event::{ ModuleEventSender, ModuleEvent };

pub fn app(app: &mut App) {
    app.add_event::<ModuleEvent>().add_system_set_to_stage(
        Label::StageMain,
        SystemSet::new()
            .with_system(update_modules.label("modules::update_modules"))
            .with_system(update_module_callbacks.label("modules::update_modules"))
            .with_system(event::do_module_events.after("modules::update_modules"))
    );
}

pub struct ModuleInfo {
    instructions: SpawnInstructions,
    name: &'static str,
    identifier: &'static str,
}

pub trait Module {
    fn info(&self) -> ModuleInfo;
    /// function that runs to update this module
    fn update(&mut self, events: &mut ModuleEventSender, state: &mut ModuleState);
    fn callback_update(&mut self, events: &mut ModuleEventSender, state: &mut ModuleState);

    fn debug_ui(&mut self, ui: &mut Ui, events: &mut ModuleEventSender, state: &ModuleState) {
        if ui.button("Fire Marble!").clicked() {
            events.send(event::ModuleUpdate::FireMarble(Marble::Bit { value: true }));
        }
    }
}

type QuerySimple<'w, 's, T> = Query<'w, 's, &'static mut T>;
// type QueryWith<'w, 's, T, W> = Query<'w, 's, &'static mut T, bevy::prelude::With<W>>;
// type QueryEntity<'w, 's, W> = Query<'w, 's, bevy::prelude::Entity, bevy::prelude::With<W>>;

/// "i want to do something after x second(s) pls help"
#[derive(Deref, DerefMut, Component)]
pub struct ModuleCallbackTimer(Timer);

impl ModuleCallbackTimer {
    pub fn new(ticks: f32) -> Self {
        ModuleCallbackTimer(Timer::new(Duration::from_secs_f32(ticks), TimerMode::Once))
    }
}

pub fn update_module_callbacks(
    mut commands: Commands,
    mut timers: Query<(&mut ModuleType, Entity, &mut ModuleCallbackTimer, &mut ModuleState)>,
    events: EventWriter<ModuleEvent>,
    time: Res<Time>,
) {
    let mut events = ModuleEventSender::new(events);
    for (mut module, entity, mut timer, mut state) in timers.iter_mut() {
        events.entity(entity);
        timer.tick(time.delta());
        
        if timer.finished() {
            module.callback_update(&mut events, &mut state);
            commands.entity(entity).remove::<ModuleCallbackTimer>();
        }
    }
}

/// tells this entity that they need to be updated (!!! (!!!)) (probably a module)
#[derive(Deref, DerefMut)]
pub struct UpdateModule(pub Entity);

/// run the update functions for the modules!!
pub fn update_modules(
    mut modules: Query<(&mut ModuleType, Entity, &mut ModuleState), Changed<ModuleState>>,
    events: EventWriter<ModuleEvent>
) {
    let mut events = ModuleEventSender::new(events);
    for (mut module, entity, mut state) in modules.iter_mut() {
        events.entity(entity);
        module.update(&mut events, &mut state);
    }
}