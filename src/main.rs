#![feature(let_chains)]
#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]

extern crate bevy_pancam;
extern crate strum;

pub use bevy::{prelude::*, sprite::Anchor};
use bevy_inspector_egui::prelude::*;

use anyhow::{anyhow, bail, ensure, Error, Ok, Result};
use bevy_pancam::PanCam;
use module::ModuleType;
use once_cell::sync::Lazy;
use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

mod atlas;
use atlas::basic;

mod spawn;
use spawn::CommandsSpawn;

mod marble;

mod module;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(bevy_pancam::PanCamPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // events
        .add_event::<spawn::SpawnMarble>()
        .add_event::<spawn::SpawnModule>()
        // startup systems
        .add_startup_system_set(
            SystemSet::new()
                .with_system(atlas::init_texture_atlas)
                .label("init"),
        )
        .add_startup_system(setup.after("init"))
        // systems
        .add_system(spawn::spawn_modules)
        .run();
}

fn setup(mut commands: Commands, mut spawn_module: EventWriter<spawn::SpawnModule>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.1;
    commands.spawn(camera).insert(PanCam::default());

    // commands.spawn_atlas_sprite(basic::body, Color::RED, default(), Anchor::Center);
    spawn_module.send(spawn::SpawnModule::new(ModuleType::Basic));
}
