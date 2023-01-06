use crate::{atlas::AtlasDictionary, misc::vec2, *};
use bevy::ecs::system::EntityCommands;

/// Spawn components that make up marble modules
pub trait SpawnComponents<'a, 'b>
where
    Self: Sized,
{
    fn get(&mut self) -> &mut Commands<'a, 'b>;

    /// Spawn the normal input component
    fn spawn_input(
        &mut self,
        mut transform: Transform,
        offset: f32,
        n: usize,
    ) -> EntityCommands<'a, 'b, '_> {
        transform.translation.z = 0.375;
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_input.info();
        let offset_tf = Transform::from_translation(Vec3::X * offset + ZOrder::InputComponent);

        let children = vec![
            commands
                .spawn((
                    Collider::polyline(
                        vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                        Some(vec![[0, 1], [2, 3]]),
                    ),
                    TransformBundle::from_transform(offset_tf),
                ))
                .insert(Name::new("in.collider"))
                .id(),
            commands
                .spawn_indicator(Vec3::new(-1.5, 0.0, 0.625) + offset_tf.translation)
                .id(),
            commands
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas,
                        sprite: TextureAtlasSprite {
                            index,
                            anchor: Anchor::Center,
                            ..default()
                        },
                        transform: offset_tf,
                        ..default()
                    },
                    Collider::ball(2.0),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    Name::new("in.sprite"),
                    marker::Input(n),
                ))
                .id(),
        ];

        let mut out = commands.spawn((
            SpriteBundle {
                transform,
                ..default()
            },
            Name::new("in.component"),
            marker::Input(n),
        ));
        out.push_children(&children);
        out
    }

    /// spawn the normal output component
    fn spawn_output(
        &mut self,
        mut transform: Transform,
        offset: f32,
        n: usize,
    ) -> EntityCommands<'a, 'b, '_> {
        transform.translation.z = 0.25;
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_output.info();
        let offset_tf = Transform::from_translation(Vec3::X * offset + ZOrder::OutputComponent);

        let children = vec![
            commands
                .spawn((
                    Collider::polyline(
                        vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                        Some(vec![[0, 1], [2, 3]]),
                    ),
                    TransformBundle::from_transform(offset_tf),
                ))
                .insert(Name::new("out.collider"))
                .id(),
            commands
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas,
                        sprite: TextureAtlasSprite {
                            index,
                            anchor: Anchor::Center,
                            ..default()
                        },
                        transform: offset_tf,
                        ..default()
                    },
                    Name::new("out.sprite"),
                ))
                .id(),
        ];

        let mut out = commands.spawn((
            SpriteBundle {
                transform,
                ..default()
            },
            Name::new("out.component"),
            marker::Output(n),
        ));

        out.push_children(&children);
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
            .insert(Name::new("indicator.sprite"))
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
            Name::new("indicator.component"),
        ));
        out.add_child(child);
        out
    }
}

impl<'a, 'b> SpawnComponents<'a, 'b> for Commands<'a, 'b> {
    fn get(&mut self) -> &mut Commands<'a, 'b> {
        self
    }
}
