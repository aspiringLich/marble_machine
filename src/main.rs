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
#![feature(iter_array_chunks)]
#![feature(drain_filter)]

extern crate derive_more;
extern crate rand;
extern crate strum;

/// the interactive components, selection, etc.
mod interactive;
use engine::modules::header::UpdateModule;
use interactive::*;

/// anything to do with graphics
mod graphics;
use graphics::*;

/// spawning in stuff, simpler logic stuff, basically stuff interfacing directly with the game engine
mod engine;
use engine::*;

/// game stuff
mod game;
use game::*;

/// take a wild guess
mod ui;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::Anchor};
// use bevy_editor_pls::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_pancam::PanCam;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use ctor::ctor;
use misc::ColorHex;
use once_cell::sync::Lazy;
use res::*;

mod fps;
mod misc;
mod query;
mod res;

use misc::marker;
use misc::CommandsName;
use ui::ui::SelectedModules;

#[derive(StageLabel)]
pub enum Label {
    StartupStageInit,
    StartupStageStart,
    StageStart,
    StageSpawn,
    StageUi,
    StageMain,
}

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
    .init_resource::<ui::ui::SpawningUiImages>()
    .insert_resource(RapierConfiguration {
        physics_pipeline_active: true,
        query_pipeline_active: true,
        timestep_mode: TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 1,
        },
        ..default()
    })
    // plugins
    .add_plugin(ShapePlugin)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(fps::FpsText)
    .add_plugin(bevy_pancam::PanCamPlugin)
    .add_plugin(EguiPlugin)
    // .add_plugin(bevy_editor_pls::EditorPlugin)
    // .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    // events
    .add_event::<marble_io::FireMarble>()
    .add_event::<UpdateModule>()
    .add_event::<spawn::SpawnModule>()
    // startup systems
    .add_startup_stage(
        Label::StartupStageInit,
        SystemStage::parallel().with_system(atlas::init_texture_atlas),
    )
    .add_startup_stage_after(
        Label::StartupStageInit,
        Label::StartupStageStart,
        SystemStage::parallel().with_system(setup),
    )
    // systems
    .add_stage(
        Label::StageStart,
        SystemStage::parallel().with_system(select::get_cursor_pos),
    )
    .add_stage_after(
        Label::StageStart,
        Label::StageSpawn,
        SystemStage::parallel()
            .with_system(spawn::spawn_modules.label("spawn::spawn_modules"))
            .with_system(marble_io::fire_marbles),
    )
    .add_stage_after(Label::StageSpawn, Label::StageUi, SystemStage::parallel())
    .add_stage_after(
        Label::StageUi,
        Label::StageMain,
        SystemStage::parallel()
            .with_system(pan_camera.label(bevy_pancam::PanCamSystemLabel))
            .with_system(marble::despawn_marbles)
            .with_system(marble_io::update_inputs)
            .with_system(modules::update_modules)
            .with_system(modules::update_module_callbacks),
    );

    interactive::app(&mut app);
    graphics::app(&mut app);
    engine::app(&mut app);
    ui::app(&mut app);

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
                    scale: 0.20,
                    ..default()
                },
                ..default()
            },
            // BloomSettings::default(),
        ))
        .insert((
            PanCam {
                grab_buttons: vec![MouseButton::Middle],
                max_scale: Some(0.3),
                max_x: Some(grid::size * grid::ext),
                min_x: Some(-grid::size * grid::ext),
                max_y: Some(grid::size * grid::ext),
                min_y: Some(-grid::size * grid::ext),
                ..default()
            },
            marker::Camera,
        ));
}

// copied from pancam and modified
fn pan_camera(
    windows: Res<Windows>,
    mut query: Query<(&PanCam, &mut Transform, &OrthographicProjection)>,
    // mut last_pos: Local<Option<Vec2>>,
    keys: Res<Input<KeyCode>>,
) {
    let Some(window) = windows.get_primary() else { error!("no window you dingus"); return };
    let window_size = Vec2::new(window.width(), window.height());

    // // Use position instead of MouseMotion, otherwise we don't get acceleration movement
    // let current_pos = match window.cursor_position() {
    //     Some(current_pos) => current_pos,
    //     None => return,
    // };
    // let delta_device_pixels = current_pos - last_pos.unwrap_or(current_pos);

    for (cam, mut transform, projection) in &mut query {
        let proj_size = Vec2::new(
            projection.right - projection.left,
            projection.top - projection.bottom,
        ) * projection.scale;

        // The proposed new camera position
        let mut proposed_cam_transform =
            if cam.enabled && keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
                let world_units_per_device_pixel = proj_size / window_size;
                let mut delta_world = Vec2::ZERO;

                let n = 12.0;
                if keys.pressed(KeyCode::W) {
                    delta_world.y -= n;
                }
                if keys.pressed(KeyCode::A) {
                    delta_world.x += n;
                }
                if keys.pressed(KeyCode::S) {
                    delta_world.y += n;
                }
                if keys.pressed(KeyCode::D) {
                    delta_world.x -= n;
                }
                transform.translation - (delta_world * world_units_per_device_pixel).extend(0.)
            } else {
                continue;
            };

        // Check whether the proposed camera movement would be within the provided boundaries, override it if we
        // need to do so to stay within bounds.
        if let Some(min_x_boundary) = cam.min_x {
            let min_safe_cam_x = min_x_boundary + proj_size.x / 2.;
            proposed_cam_transform.x = proposed_cam_transform.x.max(min_safe_cam_x);
        }
        if let Some(max_x_boundary) = cam.max_x {
            let max_safe_cam_x = max_x_boundary - proj_size.x / 2.;
            proposed_cam_transform.x = proposed_cam_transform.x.min(max_safe_cam_x);
        }
        if let Some(min_y_boundary) = cam.min_y {
            let min_safe_cam_y = min_y_boundary + proj_size.y / 2.;
            proposed_cam_transform.y = proposed_cam_transform.y.max(min_safe_cam_y);
        }
        if let Some(max_y_boundary) = cam.max_y {
            let max_safe_cam_y = max_y_boundary - proj_size.y / 2.;
            proposed_cam_transform.y = proposed_cam_transform.y.min(max_safe_cam_y);
        }

        transform.translation = proposed_cam_transform;
    }
    // *last_pos = Some(current_pos);
}
