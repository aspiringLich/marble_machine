use crate::{atlas::AtlasDictionary, misc::vec2, spawn::CommandsSpawn, *};
use bevy::ecs::system::EntityCommands;

/// Spawn components that make up marble modules
pub trait SpawnComponents<'a, 'b>
where
    Self: Sized,
{
    fn get(&mut self) -> &mut Commands<'a, 'b>;

    /// Spawn the normal input component
    fn spawn_input(&mut self, transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_input.info();

        let mut children = vec![];
        children.push(
            commands
                .spawn((
                    Collider::polyline(
                        vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                        Some(vec![[0, 1], [2, 3]]),
                    ),
                    TransformBundle::default(),
                ))
                .insert(Name::new("out.collider"))
                .id(),
        );
        children.push(commands.spawn_indicator([-1.5, 0.0, 0.625].into()).id());

        let mut out = commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::Center,
                    ..default()
                },
                transform,
                ..default()
            },
            Collider::ball(2.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            marker::Input(n),
            Name::new("in.component"),
        ));
        out.push_children(&children);
        out
    }

    /// spawn the normal output component
    fn spawn_output(&mut self, transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_output.info();

        let mut children = vec![];
        children.push(
            commands
                .spawn((
                    Collider::polyline(
                        vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                        Some(vec![[0, 1], [2, 3]]),
                    ),
                    TransformBundle::default(),
                ))
                .insert(Name::new("out.collider"))
                .id(),
        );

        let mut out = commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::Center,
                    ..default()
                },
                transform,
                ..default()
            },
            marker::Output(n),
            Name::new("out.component"),
        ));

        out.push_children(&children);
        out
    }

    fn spawn_output_ind(&mut self, transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
        let child = self.get().spawn_indicator([-1.5, 0.0, 0.625].into()).id();
        let mut out = self.spawn_output(transform, n);

        out.add_child(child);
        out
    }

    fn spawn_indicator(&mut self, pos: Vec3) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        // dbg!(basic::indicator.width());

        let child = commands
            .spawn_atlas_sprite(
                basic::indicator,
                Color::GRAY,
                Transform::from_translation([0.0, 0.0, -0.0625].into()),
            )
            .insert(Name::new("indicator.sprite"))
            .id();

        let mut out = commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: pos + Vec3::Z * 0.125,
                    scale: Vec3::ONE,
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
