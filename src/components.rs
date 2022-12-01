use crate::{atlas::AtlasDictionary, *};
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

    /// spawn the normal output component
    fn spawn_output(&mut self) -> EntityCommands<'a, 'b, '_> {
        let commands = self.get();
        let (texture_atlas, index) = basic::marble_output.info();

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
            // Collider::ball(basic::marble_output.width() * 0.5),
            // Sensor,
            // ActiveEvents::COLLISION_EVENTS,
            marker::Output,
            Name::new("out.component"),
        ))
    }
}

impl<'a, 'b> SpawnComponents<'a, 'b> for Commands<'a, 'b> {
    fn get(&mut self) -> &mut Commands<'a, 'b> {
        self
    }
}
