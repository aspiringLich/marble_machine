use crate::{ modules::SpawnInstructions, marble_io::InputState, * };
use atlas::{ basic, AtlasDictionary };

use bevy::ecs::system::EntityCommands;
use components::SpawnComponents;
use trait_enum::Deref;

use super::modules::{ body::BodyType, ModuleType };
// use bevy_rapier2d::{prelude::*, rapier::prelude::ColliderMaterial};
/// methods for spawning random things to make my code more reasonable
pub trait CommandsSpawn<'a, 'b> where Self: Sized {
    fn get(&mut self) -> &mut Commands<'a, 'b>;

    /// spawn a sprite that inherits stuff from its atlas
    fn spawn_atlas_sprite<T: AtlasDictionary>(
        &mut self,
        item: T,
        color: Color,
        transform: Transform
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
        anchor: Anchor
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

#[derive(Clone)]
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

/// spawn a module based on [`SpawnModule`] events fired
pub fn spawn_modules(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnModule>,
    mut selected: ResMut<SelectedModules>
) {
    for event in spawn_events.iter() {
        let SpawnModule { module, place } = event;

        let sprite = if *place {
            SpriteBundle {
                visibility: Visibility::INVISIBLE,
                ..default()
            }
        } else {
            SpriteBundle::default()
        };
        let parent = commands
            .spawn(sprite)
            .name(module.get_identifier())
            .insert((*module, marker::Module))
            .id();
        let mut children: Vec<Entity> = vec![];

        // spawn a small circular body and return the id
        macro spawn_body_circular(
            $body_type:expr,
            $atlasdict:expr,
            $name:literal
            $($tail:tt)*
        ) {
            children.push(
                commands
                    .spawn_atlas_sprite($atlasdict, $body_type.color(), Transform::from_xyz(0.0, 0.0, ZOrder::BodyComponent.f32()))
                    .insert((
                        Collider::ball($atlasdict.width() * 0.5 - 0.5),
                        RigidBody::Fixed,
                        Restitution::coefficient(0.8),
                        marker::ModuleBody,
                        $($tail)*
                    ))
                    .name($name)
                    .id()
            )
        }

        // run through all the instructions laid out in the module
        let SpawnInstructions { body, input_transforms, output_transforms } = module
            .spawn_instructions();

        // spawn the body
        match body {
            body @ BodyType::Small => {
                spawn_body_circular!(body, basic::body_small, "body_small.component");
            }
            _ => todo!(),
        }
        // spawn the input state
        commands.entity(parent).insert(InputState::new(input_transforms.len()));

        // inputs
        children.extend(
            input_transforms
                .iter()
                .enumerate()
                .map(|(i, &x)| commands.spawn_input(x, i).id())
        );
        // outputs
        children.extend(
            output_transforms
                .iter()
                .enumerate()
                .map(|(i, &x)| commands.spawn_output(x, i).id())
        );
        commands.entity(parent).push_children(&children);

        if *place {
            *selected = SelectedModules::place_entity(parent);
        } else {
            *selected = SelectedModules::from_entity(parent);
        }
    }
}