use std::f32::consts::PI;

use crate::{
    marble::Marble,
    module::{Module, ModuleType},
    *,
};
use atlas::AtlasDictionary;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// ??? how the fuck did this compile
pub trait CommandsSpawn<'a, 'b>
where
    Self: Sized,
{
    fn get(&mut self) -> &mut Commands<'a, 'b>;

    /// spawn a sprite that inherits stuff from its atlas
    fn spawn_atlas_sprite<T: AtlasDictionary>(
        &mut self,
        item: T,
        color: Color,
        transform: Transform,
    ) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = item.info();

        cmd.spawn(SpriteSheetBundle {
            texture_atlas,
            transform: transform,
            sprite: TextureAtlasSprite {
                index,
                color,
                anchor: Anchor::Center,
                ..default()
            },
            ..default()
        })
    }

    /// spawn a sprite that inherits stuff from its atlas also with a specified anchor
    fn spawn_atlas_sprite_a<T: AtlasDictionary>(
        &mut self,
        item: T,
        color: Color,
        transform: Transform,
        anchor: Anchor,
    ) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = item.info();

        cmd.spawn(SpriteSheetBundle {
            texture_atlas,
            transform: transform,
            sprite: TextureAtlasSprite {
                index,
                color,
                anchor,
                ..default()
            },
            ..default()
        })
    }

    fn spawn_input(&mut self, transform: Transform) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = basic::marble_input.info();

        cmd.spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: transform,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                ..default()
            },
            Name::new(format!("in.component")),
        ))
    }

    fn spawn_output(&mut self, transform: Transform) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = basic::marble_output.info();

        cmd.spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: transform,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                ..default()
            },
            Name::new(format!("out.component")),
        ))
    }
}

impl<'a, 'b> CommandsSpawn<'a, 'b> for Commands<'a, 'b> {
    fn get(&mut self) -> &mut Commands<'a, 'b> {
        self
    }
}

#[derive(Copy, Clone)]

pub struct SpawnMarble {
    marble: Marble,
    from: Entity,
    velocity: Vec2,
}

#[derive(Copy, Clone)]
pub struct SpawnModule {
    module: ModuleType,
}

impl SpawnModule {
    pub fn new(module: ModuleType) -> Self {
        SpawnModule { module }
    }
}

macro color($r:expr, $g:expr, $b:expr) {
    Color::Rgba {
        red: $r as f32 / 255.0,
        green: $g as f32 / 255.0,
        blue: $b as f32 / 255.0,
        alpha: 1.0,
    }
}

/// basically, imagine offsetting some object by `offset` in the x-axis, then rotating it around the origin `rotation` radians.
///
/// this is what this function does.
pub fn transform_from_offset_rotate(offset: f32, rotation: f32, z: f32) -> Transform {
    let rotation = Quat::from_rotation_z(rotation);
    let translation = rotation.mul_vec3(Vec3::X * offset) + Vec3::Z * z;
    Transform {
        rotation,
        translation,
        scale: Vec3::ONE,
    }
}

/// returns a transform that equates to a valid i/o position around a `body_small`.
pub fn body_small_transform(rotation: f32) -> Transform {
    transform_from_offset_rotate(basic::body_small.width() * 0.5 - 3.0, rotation, 0.25)
}

pub static MODULE_COLOR: Color = color!(101, 237, 192);

pub enum SpawnInstruction {
    BodySmall(Vec<f32>, Vec<f32>),
    BodyLarge(Vec<f32>, Vec<f32>),
    Decal((Handle<TextureAtlas>, usize)),
}

/// spawn a module based on [`SpawnModule`] events fired
pub fn spawn_modules(mut commands: Commands, mut events: EventReader<SpawnModule>) {
    for event in events.iter() {
        let module = event.module.get();

        let parent = commands.spawn(SpriteBundle { ..default() }).id();

        macro spawn_body($atlasdict:expr, $name:expr) {
            commands
                .spawn_atlas_sprite($atlasdict, MODULE_COLOR, Transform::from_xyz(0.0, 0.0, 0.5))
                .insert(Name::new($name))
                .id()
        }

        for instruction in module.spawn_instructions() {
            use SpawnInstruction::*;
            let mut children = vec![];

            let append = &mut match instruction {
                // spawn a small body with said inputs and outputs
                BodySmall(i, o) => {
                    children.extend(i.iter().map(|&rotation| {
                        commands.spawn_input(body_small_transform(rotation)).id()
                    }));
                    children.extend(o.iter().map(|&rotation| {
                        commands.spawn_output(body_small_transform(rotation)).id()
                    }));
                    vec![spawn_body!(basic::body_small, "body_small.component")]
                }
                BodyLarge(i, o) => todo!(),
                Decal((handle, index)) => todo!(),
            };
            children.append(append);
            commands.entity(parent).push_children(children.as_slice());
        }
    }
}
