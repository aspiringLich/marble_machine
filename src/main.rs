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
#![feature(try_trait_v2)]

extern crate derive_more;
extern crate rand;
extern crate strum;

/// the interactive components, selection, etc.
mod interactive;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use modules::UpdateModule;
use interactive::*;

use bevy::utils::{ HashMap, HashSet };

/// anything to do with graphics
mod graphics;
use graphics::*;

/// spawning in stuff, simpler logic stuff, basically stuff interfacing directly with the game engine
mod engine;
use engine::*;

/// stuff relating to the construction and definition of modules
mod modules;

/// game stuff
mod game;
use game::*;

/// ui stuff
mod ui;

use bevy::{ diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::Anchor };
// use bevy_editor_pls::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_pancam::PanCam;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use ctor::ctor;
use misc::ColorHex;
use res::*;

mod fps;
mod misc;
mod query;
mod res;

use misc::marker;
use misc::CommandsName;
use ui::ui::SelectedModules;

// #[derive(StageLabel)]
// pub enum Label {
//     StartupStageInit,
//     StartupStageStart,
//     StageStart,
//     StageSpawn,
//     StageUi,
//     StageMain,
//     StageInteract,
//     StagePostInteract,
// }

fn main() {
    modules::init_modules();

    let mut app = App::new();

    // bevy plugins
    app.add_plugins(
        DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            window: WindowDescriptor {
                title: "Marble Machine".to_string(),
                ..default()
            },
            ..default()
        }) // .disable::<bevy::log::LogPlugin>()
    )
        // resources
        .init_resource::<SelectedModules>()
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(DebugLinesPlugin::default())
        // .add_plugin(bevy_editor_pls::EditorPlugin)
        // .add_plugin(WorldInspectorPlugin {})
        // .add_plugin(RapierDebugRenderPlugin::default())
        // events
        .add_event::<marble_io::FireMarbleEvent>()
        .add_event::<UpdateModule>()
        .add_event::<spawn::SpawnModule>()
        // startup stages
        .add_startup_system_to_stage(StartupStage::Startup, setup);

    interactive::app(&mut app);
    graphics::app(&mut app);
    engine::app(&mut app);
    ui::app(&mut app);
    modules::app(&mut app);
    game::app(&mut app);

    // bevy_mod_debugdump::print_schedule(&mut app);
    
    app.run();
}

mod err {
    use std::error::Error;
    use std::panic::Location;

    use bevy::prelude::{ In, error };

    #[derive(Debug)]
    pub struct LocatedError {
        inner: anyhow::Error,
        location: &'static Location<'static>,
    }

    impl std::fmt::Display for LocatedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}: {}", self.inner, self.location)
        }
    }

    impl<T: Error + std::marker::Sync + std::marker::Send + 'static> From<T> for LocatedError {
        #[track_caller]
        fn from(err: T) -> Self {
            LocatedError {
                inner: anyhow::Error::new(err),
                location: std::panic::Location::caller(),
            }
        }
    }

    pub fn log_errors(error: In<Result<(), LocatedError>>) {
        if let Some(err) = error.0.err() {
            error!("{:#?}", err);
        }
    }
}
use err::{ log_errors, LocatedError };

fn setup(mut commands: Commands, grid_info: Res<grid::GridInfo>, window: Res<Windows>) {
    let window = window.get_primary().unwrap();
    let screen_size = Vec2::new(window.width(), window.height());
    let grid::GridInfo { half_size: size, ext, .. } = *grid_info;

    let factor = 12.0;

    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    // hdr: true,
                    ..default()
                },
                projection: OrthographicProjection {
                    scale: 0.2,
                    ..default()
                },
                ..default()
            },
            // BloomSettings::default(),
        ))
        .insert((
            PanCam {
                grab_buttons: vec![MouseButton::Middle],
                // max_scale: Some(0.3),
                max_x: Some(size * ext + screen_size.x / factor),
                min_x: Some(-size * ext - screen_size.x / factor),
                max_y: Some(size * ext + screen_size.y / factor),
                min_y: Some(-size * ext - screen_size.y / factor),
                ..default()
            },
            marker::Camera,
        ));
}