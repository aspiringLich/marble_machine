use crate::{
    module::{Module, ModuleType},
    *,
};
use atlas::AtlasDictionary;
use bevy::ecs::system::EntityCommands;
use components::SpawnComponents;
// use bevy_rapier2d::{prelude::*, rapier::prelude::ColliderMaterial};

/// methods for spawning random things to make my code more reasonable
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

    /// spawn a sprite that inher   its stuff from its atlas also with a specified anchor
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
            .insert((mt, marker::Module))
            .id();

        macro spawn_body_circular($atlasdict:expr, $name:expr) {
            commands
                .spawn_atlas_sprite($atlasdict, MODULE_COLOR, Transform::from_xyz(0.0, 0.0, 0.5))
                .insert((
                    Name::new($name),
                    Collider::ball($atlasdict.width() * 0.5),
                    RigidBody::Fixed,
                    Restitution::coefficient(0.8),
                    marker::ModuleBody,
                ))
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
            *selected = SelectedModule(Some(parent));
        }
    }
}
