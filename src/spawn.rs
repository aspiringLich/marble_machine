use crate::{
    marble_io::InputState,
    module::{body_small_transform, Module, ModuleType},
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
    // whether this module is going to be dragged around
    place: bool,
}

impl SpawnModule {
    pub fn new(module: ModuleType) -> Self {
        SpawnModule {
            module,
            place: false,
        }
    }

    /// this module is going to be dragged around
    pub fn place(mut self) -> Self {
        self.place = true;
        self
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
    /// spawn a body_small with input indicators
    BodySmall(Vec<f32>, Vec<f32>),
    /// spawn a body_small with input and output indicators
    BodySmallIndicators(Vec<f32>, Vec<f32>),
    /// spawn a body_large with input indicators
    BodyLarge(Vec<f32>, Vec<f32>),
    /// spawn a decal in the center of the block
    Decal((Handle<TextureAtlas>, usize)),
}

/// spawn a module based on [`SpawnModule`] events fired
pub fn spawn_modules(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnModule>,
    mut selected: ResMut<SelectedModules>,
) {
    for event in spawn_events.iter() {
        let SpawnModule { mut module, place } = event;

        let parent = commands
            .spawn(SpriteBundle { ..default() })
            .insert(module.get_inner().get_identifier())
            .insert((module, marker::Module))
            .id();

        // spawn a small circular body and return the id
        macro spawn_body_circular($atlasdict:expr, $name:literal $($tail:tt)*) {
            commands
                .spawn_atlas_sprite($atlasdict, MODULE_COLOR, Transform::from_xyz(0.0, 0.0, 0.5))
                .insert((
                    Name::new($name),
                    Collider::ball($atlasdict.width() * 0.5),
                    RigidBody::Fixed,
                    Restitution::coefficient(0.8),
                    marker::ModuleBody
                    $($tail)*
                ))
                .id()
        }

        // run through all the instructions laid out in the module
        for instruction in module.get_inner().spawn_instructions() {
            use SpawnInstruction::*;
            let mut children = vec![];

            let append = &mut match instruction {
                // spawn a small body with said inputs and outputs
                BodySmall(i, o) => {
                    children.extend(
                        i.iter()
                            .enumerate()
                            .map(|(i, x)| commands.spawn_input(body_small_transform(*x), i).id()),
                    );
                    children.extend(
                        o.iter()
                            .enumerate()
                            .map(|(i, x)| commands.spawn_output(body_small_transform(*x), i).id()),
                    );
                    commands.entity(parent).insert(InputState::new(i.len()));
                    vec![spawn_body_circular!(
                        basic::body_small,
                        "body_small.component"
                    )]
                }
                _ => todo!(),
            };
            children.append(append);
            commands.entity(parent).push_children(&children);
        }

        if *place {
            *selected = SelectedModules::place_entity(parent);
        } else {
            *selected = SelectedModules::from_entity(parent);
        }
    }
}
