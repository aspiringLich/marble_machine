use bevy::prelude::*;
use bevy::utils::Instant;

use crate::atlas::AtlasDictionary;
use crate::*;
use marble::Marble;

use std::f32::consts::*;

macro color($r:expr, $g:expr, $b:expr) {
    Color::Rgba {
        red: $r as f32 / 255.0,
        green: $g as f32 / 255.0,
        blue: $b as f32 / 255.0,
        alpha: 1.0,
    }
}

pub fn transform_from_offset_rotate(offset: f32, rotation: f32, z: f32) -> Transform {
    let rotation = Quat::from_rotation_z(rotation);
    let translation = rotation.mul_vec3(Vec3::X * offset) + Vec3::Z * z;
    Transform {
        rotation,
        translation,
        scale: Vec3::ONE,
    }
}

pub fn body_small_transform(rotation: f32) -> Transform {
    transform_from_offset_rotate(basic::body_small.width() * 0.5 - 3.0, rotation, 0.25)
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

        // body
        commands
            .spawn_atlas_sprite(
                basic::body_small,
                MODULE_COLOR,
                Transform::from_translation(Vec3::Z * 0.5),
                Anchor::Center,
            )
            .insert(Name::new("body.component"))
            .set_parent(parent);
        // input
        commands
            .spawn_input(body_small_transform(0.0), 0)
            .set_parent(parent);
        // output
        commands
            .spawn_output(body_small_transform(PI), 0)
            .set_parent(parent);
    }
}
