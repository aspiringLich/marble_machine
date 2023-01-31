use bevy::utils::HashSet;
use bevy_prototype_debug_lines::DebugLines;
use itertools::Itertools;

use crate::{misc::RapierContextMethods, query::QueryQuerySimple, *};

#[derive(Resource)]
pub struct RequestedMove {
    pub requesting: Entity,
    pub to: Transform,
    pub ignore: Vec<Entity>,
}

impl Default for RequestedMove {
    fn default() -> Self {
        Self {
            requesting: unsafe { std::mem::zeroed() },
            to: Transform::default(),
            ignore: default(),
        }
    }
}

pub fn do_requested_move(
    requested_move: Res<RequestedMove>,
    mut q_transform: Query<&mut Transform>,
    q_children: Query<&Children>,
    q_collider: Query<(Entity, &Collider)>,
    has_rigidbody: Query<With<RigidBody>>,
    q_global_transform: Query<&GlobalTransform>,
    rapier_ctx: Res<RapierContext>,
    mut lines: ResMut<DebugLines>,
) {
    if requested_move.is_changed() {
        let mut colliders = q_children
            .iter_descendants(requested_move.requesting)
            .filter_map(|e| q_collider.get(e).ok())
            .collect::<Vec<_>>();
        let ignore = colliders.iter().map(|(e, _)| *e).collect::<Vec<_>>();
        colliders.retain(|(e, _)| has_rigidbody.get(*e).is_ok());

        let predicate = |e| !ignore.contains(&e);
        let filter = QueryFilter::new().exclude_sensors().predicate(&predicate);

        let requesting = q_transform.entity(requested_move.requesting).clone();
        let to = requested_move.to;
        let diff = Transform {
            translation: requesting.translation - to.translation,
            rotation: requesting.rotation - to.rotation,
            scale: default(),
        };

        for (e, c) in &colliders {
            let mut transformed = q_global_transform.entity(*e).compute_transform();
            transformed.translation -= diff.translation;
            
            lines.line(requesting.translation, transformed.translation, 0.0);
            // transformed.rotate_around(transformed.translation - requesting.translation, -diff.rotation);
            // dbg!(transformed);
            // transformed.scale = scale * diff.scale;

            // if we detect a collision
            if let Some(e) = rapier_ctx.intersection_with_shape_transform(
                transformed,
                c,
                filter,
            ) {
                // oh noes a collision! panic!
                info!("{:#?} colllision with {:#?}", c, e);
                return;
                // jk no panic
            }
        }
        info!("good to go");
        // dbg!(colliders);

        if let Ok(mut transform) = q_transform.get_component_mut(requested_move.requesting) {
            // were good
            *transform = requested_move.to;
        } else {
            error!("Could not find transform component on requested_move.requesting, also this shouldnt have hapenned")
        }
    }
}
