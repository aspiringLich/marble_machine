use crate::{atlas::AtlasDictionary, spawn::CommandsSpawn, *};

#[derive(Copy, Clone, Debug, Component)]
pub enum Marble {
    Bit { value: bool },
    Basic { value: u8 },
}

/// an event that says to spawn a marble here
#[derive(Copy, Clone)]

pub struct SpawnMarble {
    pub marble: Marble,
    pub from: Entity,
    pub power: f32,
}

pub fn spawn_marbles(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnMarble>,
    q_transform: Query<&mut Transform>,
) {
    for event in spawn_events.iter() {
        let transform = *q_transform.get(event.from).unwrap();
        let pos = transform.translation;
        commands
            .spawn_atlas_sprite(
                basic::marble_small,
                Color::GREEN,
                Transform::from_translation(pos - pos.z),
            )
            .insert((
                Collider::ball(basic::marble_small.width() * 0.5),
                RigidBody::Dynamic,
                Velocity {
                    linvel: transform.rotation.mul_vec3(Vec3::X).truncate() * 80.0,
                    angvel: 0.0,
                },
                ColliderMassProperties::Mass(1.0),
                Restitution::coefficient(0.9),
            ))
            .insert(event.marble)
            .insert(Name::new("bit.marble"));
    }
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
