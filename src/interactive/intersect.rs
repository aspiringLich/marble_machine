use bevy::utils::HashSet;
use itertools::Itertools;

use crate::{misc::CastShapeTransform, query::QueryQuerySimple, *};

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
    q_global_transform: Query<&GlobalTransform>,
    rapier_ctx: Res<RapierContext>,
) {
    if requested_move.is_changed() {
        
        let colliders = q_children
            .iter_descendants(requested_move.requesting)
            .filter_map(|e| q_collider.get(e).ok())
            .collect::<Vec<_>>();
        let ignore = colliders.iter().map(|(e, _)| *e).collect::<Vec<_>>();

        let predicate = |e| !ignore.contains(&e);
        let filter = QueryFilter::new().exclude_sensors().predicate(&predicate);
        
        let requesting = q_transform.entity(requested_move.requesting).clone();
        let to = requested_move.to;
        let diff = Transform {
            translation: requesting.translation - to.translation,
            rotation: requesting.rotation - to.rotation,
            scale: default(),
        };

        for (e, c) in colliders.iter() {
            let mut transformed = q_global_transform.entity(*e).compute_transform();
            transformed.translation += diff.translation;
            // transformed.rotate_around(transformed.translation - requesting.translation, -diff.rotation);
            dbg!(transformed);
            // transformed.scale = scale * diff.scale;
            
            // if we detect a collision
            if let Some((_, _)) = rapier_ctx.cast_shape_transform(
                transformed,
                (*c).clone(),
                Vec2::splat(1.0 / 512.0),
                1.0,
                filter,
            ) {
                // oh noes a collision! panic!
                return;
                // jk no panic
            }
        }
        
        if let Ok(mut transform) = q_transform.get_component_mut(requested_move.requesting) {
            // were good
            *transform = requested_move.to;
        } else {
            error!("Could not find transform component on requested_move.requesting, also this shouldnt have hapenned")
        }
    }
}
