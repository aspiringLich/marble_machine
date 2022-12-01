use crate::{atlas::AtlasDictionary, spawn::CommandsSpawn, *};
use marble::Marble;

/// an event that tells the program to fire a marble from this marble output.
#[derive(Copy, Clone)]

pub struct FireMarble {
    pub marble: Marble,
    pub from: Entity,
    pub power: f32,
}

/// if any `SpawnMarbles` events have fired, fire a marble at the specified entity with the
/// right power and such and such.
pub fn fire_marbles(
    mut commands: Commands,
    mut spawn_events: EventReader<FireMarble>,
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
