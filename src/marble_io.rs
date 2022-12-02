use crate::*;
use atlas::AtlasDictionary;
use marble::Marble;
use rand::Rng;
use spawn::CommandsSpawn;

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
        let rotation = transform.rotation;
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
                    linvel: rotation.mul_vec3(Vec3::X).truncate() * 120.0,
                    angvel: rand::thread_rng().gen_range(-10.0..10.0),
                },
                ColliderMassProperties::Mass(1.0),
                Restitution::coefficient(0.9),
            ))
            .insert(event.marble)
            .insert(Name::new("bit.marble"));
    }
}
