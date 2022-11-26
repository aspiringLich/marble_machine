#![feature(let_chains)]
#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]

extern crate bevy_pancam;
extern crate strum;

pub use bevy::prelude::*;
pub use bevy_inspector_egui::prelude::*;

pub use anyhow::{anyhow, bail, ensure, Result};
use bevy_pancam::PanCam;
pub use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};
use yauc::*;
pub use yauc::{error, info, warn};

pub use once_cell::sync::Lazy;

mod atlas;
use atlas::{basic, AtlasDictionary};

mod spawn;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(bevy_pancam::PanCamPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // startup systems
        .add_startup_system(atlas::init_texture_atlas)
        .add_startup_system(setup.after(atlas::init_texture_atlas))
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

    let (texture_atlas, index, scale) = basic::marble.info();
    commands.spawn(SpriteSheetBundle {
        texture_atlas,
        transform: Transform::from_translation(Vec3::ZERO).with_scale(scale),
        sprite: TextureAtlasSprite {
            index,
            color: Color::RED,
            ..default()
        },
        ..default()
    });
}
