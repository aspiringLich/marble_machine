use bevy::prelude::*;
use bevy::utils::Instant;

use crate::atlas::AtlasDictionary;
use crate::*;
use marble::Marble;

macro color($r:expr, $g:expr, $b:expr) {
    Color::Rgba {
        red: $r as f32 / 255.0,
        green: $g as f32 / 255.0,
        blue: $b as f32 / 255.0,
        alpha: 1.0,
    }
}

pub static MODULE_COLOR: Color = color!(101, 237, 192);

pub trait Module
where
    Self: Sized + Component,
{
    fn build(commands: &mut Commands);
}

#[derive(Component)]
pub struct Basic {
    input: Option<(Marble, Instant)>,
}

impl Module for Basic {
    fn build(commands: &mut Commands) {
        // the parent entity
        let parent = commands
            .spawn(SpriteBundle { ..default() })
            .insert(Basic { input: None })
            .insert(Name::new("basic.module"))
            .id();

        commands
            .spawn_atlas_sprite(
                basic::body_small,
                MODULE_COLOR,
                Transform::from_translation(Vec3::Z * 0.5),
                Anchor::Center,
            )
            .insert(Name::new("body.component"))
            .set_parent(parent);

        let vec = basic::body_small.vec() - Vec3::X * 3.0;

        commands
            .spawn_input(
                Transform {
                    translation: vec,
                    ..default()
                },
                0,
            )
            .set_parent(parent);

        commands
            .spawn_output(
                Transform {
                    translation: vec * Vec3::NEG_X,
                    rotation: Quat::from_rotation_z(std::f32::consts::PI),
                    ..default()
                },
                0,
            )
            .set_parent(parent);
    }
}
