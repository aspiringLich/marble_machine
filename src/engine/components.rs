use crate::{misc::vec2, *};
use atlas::{basic, AtlasDictionary};
use bevy::ecs::system::EntityCommands;

/// Spawn components that make up marble modules
pub trait SpawnComponents<'a, 'b>
where
    Self: Sized,
{
    fn get(&mut self) -> &mut Commands<'a, 'b>;

    /// Spawn the normal input component
    fn spawn_input(&mut self, transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
        self.spawn_input_inner::<true>(transform, n)
    }

    /// Spawn the input component but nonfunctional (no collider)
    fn spawn_input_nonfunctional(
        &mut self,
        transform: Transform,
        n: usize,
    ) -> EntityCommands<'a, 'b, '_> {
        self.spawn_input_inner::<false>(transform, n)
    }

    /// Spawn the normal input component but you can choose whether its functional or nonfunctional i guess
    fn spawn_input_inner<const B: bool>(
        &mut self,
        transform: Transform,
        n: usize,
    ) -> EntityCommands<'a, 'b, '_> {
        // transform.translation.z = 0.375;
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_input.info();

        let len = transform.translation.length();
        let indicator = commands.spawn_indicator(Vec3::X * (len + 2.0)).id();

        if B {}

        let mut input: Entity = unsafe { std::mem::zeroed() };
        let out = commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_rotation(transform.rotation)
                        .with_translation(Vec3::Z * (ZOrder::InputComponent.f32())),
                    ..default()
                },
                marker::Input(n),
            ))
            .name("in.component")
            .add_child(indicator)
            .add_children(|children| {
                // the sprite itself of the input component
                input = children
                    .spawn((SpriteSheetBundle {
                        texture_atlas,
                        sprite: TextureAtlasSprite {
                            index,
                            anchor: Anchor::CenterLeft,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::X * len),
                        ..default()
                    },))
                    .name("in.sprite")
                    .id();
                // insert the collider if we want it to be functional
                if B {
                    children
                        .spawn((
                            Collider::polyline(
                                vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                                Some(vec![[0, 1], [2, 3]]),
                            ),
                            TransformBundle::from_transform(Transform::from_translation(
                                Vec3::X * len,
                            )),
                            RigidBody::Fixed,
                        ))
                        .name("in.collider");
                }
                children.parent_entity()
            });
        if B {
            commands.entity(input).insert((
                Collider::ball(2.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                marker::Input(n),
            ));
        }
        commands.entity(out)
    }

    /// spawn the normal output component
    fn spawn_output(&mut self, mut transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
        transform.translation.z = 0.25;
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_output.info();

        let len = transform.translation.length();

        let children = vec![
            commands
                .spawn((
                    Collider::polyline(
                        vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                        Some(vec![[0, 1], [2, 3]]),
                    ),
                    TransformBundle::from_transform(Transform::from_translation(Vec3::X * len)),
                    RigidBody::Fixed,
                ))
                .name("out.collider")
                .id(),
            commands
                .spawn((SpriteSheetBundle {
                    texture_atlas,
                    sprite: TextureAtlasSprite {
                        index,
                        anchor: Anchor::CenterLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::X * len),
                    ..default()
                },))
                .name("out.sprite")
                .id(),
        ];

        let mut out = commands.spawn((
            SpriteBundle {
                transform: Transform::from_rotation(transform.rotation)
                    .with_translation(Vec3::Z * (ZOrder::OutputComponent.f32())),
                ..default()
            },
            marker::Output(n),
        ));

        out.push_children(&children).name("out.component");
        out
    }

    // fn spawn_output_ind(&mut self, transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
    //     let child = self.get().spawn_indicator([-1.5, 0.0, 0.625].into()).id();
    //     let mut out = self.spawn_output(transform, n);

    //     out.add_child(child);
    //     out
    // }

    fn spawn_indicator(&mut self, pos: Vec3) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        // dbg!(basic::indicator.width());

        let child = commands
            .spawn(SpriteBundle {
                transform: Transform::from_scale([3.0, 3.0, 1.0].into())
                    .with_translation(Vec3::new(0.0, 0.0, -0.0625)),
                sprite: Sprite {
                    color: Color::DARK_GRAY,
                    ..default()
                },
                ..default()
            })
            .name("indicator.sprite")
            .id();

        let mut out = commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: pos + ZOrder::IndicatorComponent - Vec3::Z * pos.z,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::Hsla {
                        hue: 0.0,
                        saturation: 0.8,
                        lightness: 0.4,
                        alpha: 1.0,
                    },
                    ..default()
                },
                ..default()
            },
            marker::Indicator,
        ));
        out.add_child(child).name("indicator.component");
        out
    }
}

impl<'a, 'b> SpawnComponents<'a, 'b> for Commands<'a, 'b> {
    fn get(&mut self) -> &mut Commands<'a, 'b> {
        self
    }
}
