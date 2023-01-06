use crate::{
    marble_io::InputState,
    module::{Module, ModuleType},
    *,
};
use atlas::{basic, AtlasDictionary};

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

#[derive(Default)]
pub enum BodyType {
    #[default]
    Small,
    Large,
}

pub fn offset_of<T: AtlasDictionary>(input: T) -> f32 {
    input.width() * 0.5 + 1.0
}

impl BodyType {
    pub fn offset(&self) -> f32 {
        match self {
            BodyType::Small => offset_of(basic::body_small),
            BodyType::Large => todo!(),
        }
    }
}

#[derive(Default)]
pub struct SpawnInstructions {
    pub body: BodyType,
    pub input_transforms: Vec<Transform>,
    pub output_transforms: Vec<Transform>,
}

unsafe impl Sync for SpawnInstructions {}
unsafe impl Send for SpawnInstructions {}

impl SpawnInstructions {
    pub fn from_body(body: BodyType) -> Self {
        Self { body, ..default() }
    }

    pub fn with_input_rotations<T: Iterator<Item = f32>>(mut self, input_transforms: T) -> Self {
        self.input_transforms = input_transforms
            .map(|r| {
                Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, r.to_radians()))
            })
            .collect();
        self
    }

    pub fn with_output_rotations<T: Iterator<Item = f32>>(mut self, output_transforms: T) -> Self {
        self.output_transforms = output_transforms
            .map(|r| {
                Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, r.to_radians()))
            })
            .collect();
        self
    }
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
        let mut children: Vec<Entity> = vec![];

        // spawn a small circular body and return the id
        macro spawn_body_circular($atlasdict:expr, $name:literal $($tail:tt)*) {
            children.push(
                commands
                    .spawn_atlas_sprite($atlasdict, MODULE_COLOR, Transform::from_xyz(0.0, 0.0, ZOrder::BodyComponent.f32()))
                    .insert((
                        Name::new($name),
                        Collider::ball($atlasdict.width() * 0.5),
                        RigidBody::Fixed,
                        Restitution::coefficient(0.8),
                        marker::ModuleBody
                        $($tail)*
                    ))
                    .id()
            )
        }

        // run through all the instructions laid out in the module
        let SpawnInstructions {
            body,
            input_transforms,
            output_transforms,
        } = module.get_inner().spawn_instructions();

        // spawn the body
        match body {
            BodyType::Small => {
                spawn_body_circular!(basic::body_small, "body_small.component");
            }
            _ => todo!(),
        }
        // spawn the input state
        commands
            .entity(parent)
            .insert(InputState::new(input_transforms.len()));

        // inputs
        children.extend(
            input_transforms
                .iter()
                .enumerate()
                .map(|(i, &x)| commands.spawn_input(x, body.offset(), i).id()),
        );
        // outputs
        children.extend(
            output_transforms
                .iter()
                .enumerate()
                .map(|(i, &x)| commands.spawn_output(x, body.offset(), i).id()),
        );
        commands.entity(parent).push_children(&children);

        if *place {
            *selected = SelectedModules::place_entity(parent);
        } else {
            *selected = SelectedModules::from_entity(parent);
        }
    }
}
