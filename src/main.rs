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

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::Anchor};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_pancam::PanCam;
use bevy_rapier2d::prelude::*;
use module::ModuleType;
use once_cell::sync::Lazy;

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
        // default plugin
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
        // resources
        .init_resource::<SelectedModules>()
        .init_resource::<select::CursorCoords>()
        .init_resource::<ui::SpawningUiImages>()
        // plugins
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(fps::FpsText)
        .add_plugin(EguiPlugin)
        .add_plugin(bevy_pancam::PanCamPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
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
                .with_system(ui::inspector_ui)
                .with_system_set(select::system_set())
                .with_system(pan_camera)
                .with_system(marble::despawn_marbles)
                .with_system(marble_io::update_inputs)
                .with_system(module::update_modules)
                .with_system(module::update_module_callbacks)
                .with_system(ui::spawning_ui),
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
    let (_, mut transform) = query_camera.single_mut();
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
