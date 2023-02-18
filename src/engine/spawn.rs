use crate::{
    modules::{ SpawnInstructions, ModuleComponent, Module },
    *,
    engine::module_state::ModuleState,
    game::save_load::ModuleInfo,
};
use atlas::AtlasDictionary;
use bevy::ecs::system::EntityCommands;
use components::SpawnComponents;

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

pub struct SpawnModule {
    info: ModuleInfo,
    // whether this module is going to be dragged around
    place: bool,
}

impl SpawnModule {
    pub fn from_type(module: ModuleType) -> Self {
        SpawnModule {
            info: ModuleInfo::new(module),
            place: false,
        }
    }

    pub fn new(info: ModuleInfo) -> Self {
        SpawnModule {
            info,
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
        let SpawnModule { info: ModuleInfo { module, instructions, module_type, offset }, place } =
            event;

        let mut sprite = if *place {
            SpriteBundle {
                visibility: Visibility::INVISIBLE,
                ..default()
            }
        } else {
            SpriteBundle::default()
        };
        sprite.transform.translation = *offset;
        let parent = commands
            .spawn(sprite)
            .name(module_type.get_identifier())
            .insert((ModuleComponent { ty: *module_type, module: module.clone() }, marker::Module))
            .id();
        let mut children: Vec<Entity> = vec![];

        let mut state = ModuleState::default();

        // spawn a small circular body and return the id
        macro spawn_body(
            $body_type:expr,
            $name:literal
            $($tail:tt)*
        ) {
            let body = commands
                    .spawn_atlas_sprite($body_type.sprite(), $body_type.color(), Transform::from_xyz(0.0, 0.0, ZOrder::BodyComponent.f32()))
                    .insert((
                        $body_type.collider(),
                        RigidBody::Fixed,
                        Restitution::coefficient(0.8),
                        *$body_type,
                        $($tail)*
                    ))
                    .name($name)
                    .id();
            children.push(body);
            state.body = body;
        }

        // run through all the instructions laid out in the module
        let SpawnInstructions {
            body,
            inputs: input_transforms,
            outputs: output_transforms,
        } = instructions;

        // spawn the body
        match body {
            body @ BodyType::Small => {
                spawn_body!(body, "body_small.component");
            }
            _ => todo!(),
        }

        // inputs
        let (inputs, indicators): (Vec<_>, Vec<_>) = input_transforms
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let (a, b) = commands.spawn_input(x, i);
                (a.id(), b)
            })
            .unzip();
        children.extend(&inputs);

        // outputs
        let outputs = output_transforms
            .iter()
            .enumerate()
            .map(|(i, x)| commands.spawn_output(x, i).id())
            .collect::<Vec<_>>();
        children.extend(&outputs);

        state.inputs = inputs;
        state.outputs = outputs;
        state.indicators = indicators;
        state.input_state = vec![None; state.inputs.len()];

        commands.entity(parent).push_children(&children).insert(state);

        if *place {
            *selected = SelectedModules::place_entity(parent);
        } else {
            *selected = SelectedModules::from_entity(parent);
        }
    }
}