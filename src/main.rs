#![feature(let_chains)]
#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]

extern crate bevy_pancam;
extern crate strum;

use bevy::{prelude::*, sprite::Anchor};
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

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

mod misc;
use misc::marker;

fn main() {
    App::new()
        // resources
        // plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(bevy_pancam::PanCamPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
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
        .add_system(display_events)
        .run();
}

fn setup(mut commands: Commands, mut spawn_module: EventWriter<spawn::SpawnModule>) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.1,
                ..default()
            },
            ..default()
        })
        .insert(PanCam::default());

    // commands.spawn_atlas_sprite(basic::body, Color::RED, default(), Anchor::Center);
    spawn_module.send(spawn::SpawnModule::new(ModuleType::Basic));
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
