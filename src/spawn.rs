use crate::*;
use atlas::AtlasDictionary;
use bevy::{ecs::system::EntityCommands, prelude::*};

/// ??? how the fuck did this compile
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
        anchor: Anchor,
    ) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = item.info();

        cmd.spawn(SpriteSheetBundle {
            texture_atlas,
            transform: transform,
            sprite: TextureAtlasSprite {
                index,
                color,
                anchor,
                ..default()
            },
            ..default()
        })
    }

    fn spawn_input(&mut self, transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = basic::marble_input.info();

        cmd.spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: transform,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                ..default()
            },
            Name::new(format!("in{}.component", n)),
        ))
    }

    fn spawn_output(&mut self, transform: Transform, n: usize) -> EntityCommands<'a, 'b, '_> {
        let cmd = self.get();
        let (texture_atlas, index) = basic::marble_output.info();

        cmd.spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: transform,
                sprite: TextureAtlasSprite {
                    index,
                    color: Color::GRAY,
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                ..default()
            },
            Name::new(format!("out{}.component", n)),
        ))
    }
}

impl<'a, 'b> CommandsSpawn<'a, 'b> for Commands<'a, 'b> {
    fn get(&mut self) -> &mut Commands<'a, 'b> {
        self
    }
}
