use crate::*;

#[derive(Copy, Clone, Debug, Component)]
pub enum Marble {
    Bit { value: bool },
    Basic { value: u8 },
}

/// despawn marbles if they go too low (and should be despawned cuz theyre out of bounds)
pub fn despawn_marbles(
    mut commands: Commands,
    q_transform: Query<&Transform>,
    q_marbles: Query<Entity, With<Marble>>,
) {
    for marble in q_marbles.iter() {
        let transform = q_transform.get(marble).unwrap();
        if transform.translation.y < -1000.0 {
            commands.entity(marble).despawn_recursive();
        }
    }
}
