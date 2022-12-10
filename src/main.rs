#![feature(let_chains)]
#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]
#![feature(float_next_up_down)]
#![feature(type_alias_impl_trait)]
#![feature(const_trait_impl)]
#![feature(associated_type_defaults)]
#![feature(return_position_impl_trait_in_trait)]

extern crate derive_more;
extern crate rand;
extern crate strum;

use auto_unwrap::auto_unwrap;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::Anchor};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_pancam::PanCam;
use bevy_rapier2d::prelude::*;
use module::ModuleType;
// use once_cell::sync::Lazy;
// use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

mod atlas;
mod components;
mod fps;
mod marble;
mod marble_io;
mod misc;
mod module;
mod place;
mod select;
mod spawn;
mod ui;

use atlas::basic;
use misc::marker;
use ui::SelectedModules;

fn main() {
    App::new()
        // resources
        .init_resource::<SelectedModules>()
        .init_resource::<select::CursorCoords>()
        // plugins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Marble Machine".to_string(),
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(fps::FpsText)
        .add_plugin(EguiPlugin)
        .add_plugin(bevy_pancam::PanCamPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        // events
        .add_event::<marble_io::FireMarble>()
        .add_event::<module::UpdateModule>()
        .add_event::<spawn::SpawnModule>()
        // startup systems
        .add_startup_system_set(
            SystemSet::new()
                .with_system(atlas::init_texture_atlas)
                .label("init"),
        )
        .add_startup_system(setup.after("init"))
        // systems
        .add_stage(
            "start",
            SystemStage::parallel().with_system(select::get_cursor_pos),
        )
        .add_stage_after(
            "start",
            "spawn",
            SystemStage::parallel()
                .with_system(spawn::spawn_modules)
                .with_system(marble_io::fire_marbles),
        )
        .add_stage_after(
            "spawn",
            "main",
            SystemStage::parallel()
                .with_system(display_events)
                .with_system(ui::inspector_ui.label("ui"))
                .with_system(select::get_selected.after(ui::inspector_ui))
                .with_system(select::drag_selected.after(select::get_selected))
                .with_system(marble::despawn_marbles)
                .with_system(pan_camera)
                .with_system(marble_io::update_inputs)
                .with_system(module::update_modules),
        )
        .run();
}

fn setup(mut commands: Commands, mut spawn_module: EventWriter<spawn::SpawnModule>) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.15,
                ..default()
            },
            ..default()
        })
        .insert((
            PanCam {
                grab_buttons: vec![MouseButton::Middle],
                ..default()
            },
            marker::Camera,
        ));

    spawn_module.send(spawn::SpawnModule::new(ModuleType::Basic(default())));
}

fn pan_camera(
    keys: Res<Input<KeyCode>>,
    mut query_camera: Query<(&mut OrthographicProjection, &mut Transform), With<marker::Camera>>,
) {
    let (projection, mut transform) = query_camera.single_mut();
    let scrollamt = 1.8;
    let pos = &mut transform.translation;

    if keys.pressed(KeyCode::A) {
        pos.x -= scrollamt
    }
    if keys.pressed(KeyCode::D) {
        pos.x += scrollamt
    }
    if keys.pressed(KeyCode::W) {
        pos.y += scrollamt
    }
    if keys.pressed(KeyCode::S) {
        pos.y -= scrollamt
    }
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
