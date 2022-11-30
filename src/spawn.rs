// use std::f32::consts::*;

use crate::{
    module::{Module, ModuleType},
    *,
};
use atlas::AtlasDictionary;
use bevy::ecs::system::EntityCommands;
// use bevy_rapier2d::{prelude::*, rapier::prelude::ColliderMaterial};

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
            transform,
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
            transform,
            sprite: TextureAtlasSprite {
                index,
                color,
                anchor,
                ..default()
            },
            ..default()
        })
    }

    fn spawn_input(&mut self) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_input.info();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::Center,
                    ..default()
                },
                ..default()
            },
            Collider::ball(basic::marble_input.width() * 0.5),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            marker::Input,
            Name::new("in.component"),
        ))
    }

    fn spawn_output(&mut self) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = basic::marble_output.info();

        cmd.spawn((
            SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::Center,
                    ..default()
                },
                ..default()
            },
            Collider::ball(basic::marble_output.width() * 0.5),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            marker::Output,
            Name::new("out.component"),
        ))
    }
}

impl<'a, 'b> CommandsSpawn<'a, 'b> for Commands<'a, 'b> {
    fn get(&mut self) -> &mut Commands<'a, 'b> {
        self
    }
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

pub static MODULE_COLOR: Color = color!(101, 237, 192);

pub enum SpawnInstruction {
    BodySmall(Vec<f32>, Vec<f32>),
    BodyLarge(Vec<f32>, Vec<f32>),
    Decal((Handle<TextureAtlas>, usize)),
}

/// spawn a module based on [`SpawnModule`] events fired
pub fn spawn_modules(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnModule>,
    mut selected: ResMut<SelectedModule>,
) {
    for event in spawn_events.iter() {
        let mut mt = event.module;

        let parent = commands
            .spawn(SpriteBundle { ..default() })
            .insert(Name::new({
                use ModuleType::*;

                #[allow(unreachable_patterns)]
                match mt {
                    Basic { .. } => "basic.module",
                    _ => unimplemented!(),
                }
            }))
            .insert(mt)
            .id();

        macro spawn_body_circular($atlasdict:expr, $name:expr) {
            commands
                .spawn_atlas_sprite($atlasdict, MODULE_COLOR, Transform::from_xyz(0.0, 0.0, 0.5))
                .insert(Name::new($name))
                .insert(Collider::ball($atlasdict.width() * 0.5))
                .insert((RigidBody::Fixed, Restitution::coefficient(0.8)))
                .id()
        }

        for instruction in mt.get_inner().spawn_instructions() {
            use SpawnInstruction::*;
            let mut children = vec![];

            let append = &mut match instruction {
                // spawn a small body with said inputs and outputs
                BodySmall(i, o) => {
                    children.extend(i.iter().map(|_| commands.spawn_input().id()));
                    children.extend(o.iter().map(|_| commands.spawn_output().id()));
                    vec![spawn_body_circular!(
                        basic::body_small,
                        "body_small.component"
                    )]
                }
                BodyLarge(i, o) => todo!(),
                Decal((handle, index)) => todo!(),
            };
            children.append(append);
            commands.entity(parent).push_children(children.as_slice());
            *selected = SelectedModule {
                selected: Some(parent),
            };
        }
    }
}
