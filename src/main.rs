#![feature(let_chains)]
#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]
#![feature(float_next_up_down)]
#![feature(type_alias_impl_trait)]
#![feature(const_trait_impl)]
#![feature(associated_type_defaults)]
#![feature(once_cell)]
#![feature(trivial_bounds)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(panic_backtrace_config)]
#![feature(core_intrinsics)]

extern crate derive_more;
extern crate rand;
extern crate strum;

use bevy::{
    core_pipeline::bloom::BloomSettings, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
    sprite::Anchor,
};
use bevy_editor_pls::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_pancam::PanCam;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use module::ModuleType;
use once_cell::sync::Lazy;

mod components;
mod fps;
mod marble;
mod marble_io;
mod misc;
mod module;
mod spawn;
mod ui;

mod interactive;
use interactive::*;

mod render;
use render::*;

use atlas::basic;
use misc::marker;
use ui::SelectedModules;

fn main() {
    let mut app = App::new();
    // bevy plugins
    app.add_plugins(
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
    .init_resource::<select::HoveredEntities>()
    .init_resource::<ui::SpawningUiImages>()
    .insert_resource(RapierConfiguration {
        physics_pipeline_active: true,
        query_pipeline_active: true,
        ..default()
    })
    // plugins
    .add_plugin(EguiPlugin)
    .add_plugin(ShapePlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(fps::FpsText)
    .add_plugin(bevy_pancam::PanCamPlugin)
    // .add_plugin(EditorPlugin)
    // .add_plugin(WorldInspectorPlugin::new())
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
            .with_system(select::get_hovered_entities.after(spawn::spawn_modules))
            .with_system(marble_io::fire_marbles),
    )
    .add_stage_after(
        "spawn",
        "ui",
        SystemStage::parallel()
            .with_system(ui::inspector_ui)
            .with_system(ui::spawning_ui)
            .with_system(ui::debug_ui),
    )
    .add_stage_after(
        "ui",
        "main",
        SystemStage::parallel()
            .with_system(pan_camera)
            .with_system(marble::despawn_marbles)
            .with_system(marble_io::update_inputs)
            .with_system(module::update_modules)
            .with_system(module::update_module_callbacks),
    );

    interactive::app(&mut app);
    render::app(&mut app);
    app.run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    // hdr: true,
                    ..default()
                },
                projection: OrthographicProjection {
                    scale: 0.15,
                    ..default()
                },
                ..default()
            },
            // BloomSettings::default(),
        ))
        .insert((
            PanCam {
                grab_buttons: vec![MouseButton::Middle],
                ..default()
            },
            marker::Camera,
        ));
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
