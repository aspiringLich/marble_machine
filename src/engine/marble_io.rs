use crate::{
    modules::UpdateModule,
    query::{ QueryQueryIter, QueryQuerySimple },
    *,
    graphics::grid::GridInfo,
};
use atlas::{ basic, AtlasDictionary };
use marble::Marble;
use rand::Rng;
use spawn::CommandsSpawn;

use super::{lifetime::Lifetime, module_state::ModuleState};

/// an event that tells the program to fire a marble from this marble output.
#[derive(Copy, Clone)]
pub struct FireMarbleEvent {
    pub marble: Marble,
    pub from: Entity,
    pub power: f32,
}

impl FireMarbleEvent {
    pub fn new(marble: Marble, from: Entity, power: f32) -> Self {
        FireMarbleEvent {
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
    mut spawn_events: EventReader<FireMarbleEvent>,
    q_global_transform: Query<&GlobalTransform>,
    q_children: Query<&Children>,
    w_sprite: Query<Entity, With<TextureAtlasSprite>>,
    grid_info: Res<GridInfo>
) {
    for event in spawn_events.iter() {
        let mut transform = q_global_transform
            .entity(q_children.entity(event.from).iter().with(&w_sprite).next().unwrap())
            .compute_transform();
        transform.translation.z = 0.0;
        let pos = transform.translation;
        if !grid_info.in_bounds(pos.truncate()) {
            continue;
        }

        // dbg!(rotation.mul_vec3(Vec3::X).truncate() * 120.0);
        commands
            .spawn_atlas_sprite(
                basic::marble_small,
                Color::GREEN,
                Transform::from_translation(pos + -pos.z + ZOrder::Marble)
            )
            .insert((
                Collider::ball(basic::marble_small.width() * 0.5),
                RigidBody::Dynamic,
                Velocity {
                    linvel: transform.rotation.mul_vec3(Vec3::X).truncate() * VELOCITY_FACTOR,
                    angvel: rand::thread_rng().gen_range(-10.0..10.0),
                },
                ColliderMassProperties::Mass(1.0),
                Restitution::coefficient(0.9),
                Lifetime(1200),
            ))
            .insert(event.marble)
            .name("bit.marble");
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_inputs(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut q_state: Query<&mut ModuleState>,
    q_parent: Query<&Parent>,
    q_marble: Query<&Marble>,
    q_input: Query<&marker::Input>,
    has_marble: Query<With<Marble>>,
    mut update_event: EventWriter<UpdateModule>
) {
    for event in collision_events.iter() {
        use CollisionEvent::*;

        let (e1, e2) = match event {
            Started(e1, e2, _) => (*e1, *e2),
            _ => {
                continue;
            }
        };

        let mut handle_event = |e1, e2| {
            // if e1 is an input and e2 is a marble
            if let Ok(&marker::Input(index)) = q_input.get(e1) && has_marble.has(e2) {
                let marble_e = e2;
                let marble = *q_marble.entity(e2);

                let parent = q_parent.entity(q_parent.entity(e1).get()).get();
                let input_state = &mut q_state.entity_mut(parent).input_state;

                // if the input is not occupied, despawn the marble and update input_state
                if input_state[index].is_none() {
                    input_state[index] = Some(marble);
                    commands.entity(marble_e).despawn();
                    update_event.send(UpdateModule(parent));
                }
            }
        };

        handle_event(e1, e2);
        handle_event(e2, e1);
    }
}