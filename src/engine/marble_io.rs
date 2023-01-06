use crate::{
    module::param::{QueryQueryIter, QueryQuerySimple},
    *,
};
use atlas::{ AtlasDictionary, basic };
use marble::Marble;
use rand::Rng;
use spawn::CommandsSpawn;

use super::lifetime::Lifetime;

/// an event that tells the program to fire a marble from this marble output.
#[derive(Copy, Clone)]

pub struct FireMarble {
    pub marble: Marble,
    pub from: Entity,
    pub power: f32,
}

impl FireMarble {
    pub fn new(marble: Marble, from: Entity, power: f32) -> Self {
        FireMarble {
            marble,
            from,
            power,
        }
    }
}

pub const VELOCITY_FACTOR: f32 = 120.0;

/// if any `SpawnMarbles` events have fired, fire a marble at the specified entity with the
/// right power and such and such.
pub fn fire_marbles(
    mut commands: Commands,
    mut spawn_events: EventReader<FireMarble>,
    q_transform: Query<&mut Transform>,
    q_children: Query<&Children>,
    w_sprite: Query<Entity, With<TextureAtlasSprite>>,
    q_parent: Query<&Parent>,
) {
    for event in spawn_events.iter() {
        let parent = q_parent.entity(event.from).get();
        let mut transform = *q_transform
            .entity(
                q_children
                    .entity(event.from)
                    .iter()
                    .with(&w_sprite)
                    .next()
                    .unwrap(),
            );
        transform.translation.z = 0.0;
        transform.rotate_around(Vec3::ZERO, q_transform.entity(event.from).rotation);
        let p_transform = q_transform.entity(parent);

        // get the rotation and position of the parent entity + the output
        let rotation = transform.rotation.mul_quat(p_transform.rotation);
        let pos = transform.translation + p_transform.translation;
        // dbg!(rotation.mul_vec3(Vec3::X).truncate() * 120.0);
        commands
            .spawn_atlas_sprite(
                basic::marble_small,
                Color::GREEN,
                Transform::from_translation(pos + - pos.z + ZOrder::Marble),
            )
            .insert((
                Collider::ball(basic::marble_small.width() * 0.5),
                RigidBody::Dynamic,
                Velocity {
                    linvel: rotation.mul_vec3(Vec3::X).truncate() * VELOCITY_FACTOR,
                    angvel: rand::thread_rng().gen_range(-10.0..10.0),
                },
                ColliderMassProperties::Mass(1.0),
                Restitution::coefficient(0.9),
                Lifetime(600)
            ))
            .insert(event.marble)
            .name("bit.marble");
    }
}

/// a structure that holds the state of a module's inputs
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct InputState {
    inner: Vec<Option<Marble>>,
}

impl InputState {
    /// create a new InputState from a length
    pub fn new(len: usize) -> Self {
        Self {
            inner: vec![None; len],
        }
    }
}

impl std::ops::Index<usize> for InputState {
    type Output = Option<Marble>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl std::ops::IndexMut<usize> for InputState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_inputs(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut q_input_state: Query<&mut InputState>,
    q_parent: Query<&Parent>,
    q_marble: Query<&Marble>,
    q_input: Query<&marker::Input>,
    has_marble: Query<With<Marble>>,
    mut update_event: EventWriter<module::UpdateModule>,
) {
    for event in collision_events.iter() {
        use CollisionEvent::*;

        let (e1, e2) = match event {
            Started(e1, e2, _) => (*e1, *e2),
            _ => continue,
        };

        let mut handle_event = |e1, e2| {
            // if e1 is an input and e2 is a marble
            if let Ok(&marker::Input(index)) = q_input.get(e1) && has_marble.has(e2) {
                let marble_e = e2;
                let marble = *q_marble.entity(e2);
                
                let parent = q_parent.entity(q_parent.entity(e1).get()).get();
                let mut input_state = q_input_state.entity_mut(parent);
                
                // if the input is not occupied, despawn the marble and update input_state
                if input_state[index].is_none() {
                    input_state[index] = Some(marble);
                    commands.entity(marble_e).despawn();
                    update_event.send(module::UpdateModule(parent));
                }
            }
        };

        handle_event(e1, e2);
        handle_event(e2, e1);
    }
}
