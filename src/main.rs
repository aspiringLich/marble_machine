#![feature(let_chains)]
#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]

extern crate bevy_pancam;
extern crate strum;

pub use bevy::{prelude::*, sprite::Anchor};
use bevy_inspector_egui::prelude::*;

use anyhow::{anyhow, bail, ensure, Error, Ok, Result};
use bevy_pancam::PanCam;
use once_cell::sync::Lazy;
use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

mod atlas;
use atlas::basic;

mod spawn;
use spawn::CommandsSpawn;

mod marble;

mod module;
use module::Module;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(bevy_pancam::PanCamPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // startup systems
        .add_startup_system_set(
            SystemSet::new()
                .with_system(atlas::init_texture_atlas)
                .label("init"),
        )
        .add_startup_system(setup.after("init"))
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

    // commands.spawn_atlas_sprite(basic::body, Color::RED, default(), Anchor::Center);
    module::Basic::build(&mut commands)
}
