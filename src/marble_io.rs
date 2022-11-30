use crate::*;

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
