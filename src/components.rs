use crate::{atlas::AtlasDictionary, misc::vec2, *};
use bevy::ecs::system::EntityCommands;

/// Spawn components that make up marble modules
pub trait SpawnComponents<'a, 'b>
where
    Self: Sized,
{
    fn get(&mut self) -> &mut Commands<'a, 'b>;

    /// Spawn the normal input component
    fn spawn_input(&mut self) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_input.info();

        let child = commands
            .spawn((
                Collider::polyline(
                    vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                    Some(vec![[0, 1], [2, 3]]),
                ),
                TransformBundle::default(),
            ))
            .insert(Name::new("in.collider"))
            .id();
        let mut out = commands.spawn((
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
            Collider::ball(2.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            marker::Input,
            Name::new("in.component"),
        ));
        out.add_child(child);
        out
    }

    /// spawn the normal output component
    fn spawn_output(&mut self) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_output.info();

        let child = commands
            .spawn((
                Collider::polyline(
                    vec![vec2!(3, 5), vec2!(-3, 3), vec2!(-3, -3), vec2!(3, -5)],
                    Some(vec![[0, 1], [2, 3]]),
                ),
                TransformBundle::default(),
            ))
            .insert(Name::new("out.collider"))
            .id();
        let mut out = commands.spawn((
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
            // Collider::ball(basic::marble_output.width() * 0.5),
            // Sensor,
            // ActiveEvents::COLLISION_EVENTS,
            marker::Output,
            Name::new("out.component"),
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
