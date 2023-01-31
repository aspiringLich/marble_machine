use bevy_prototype_debug_lines::DebugLines;
use itertools::Itertools;

use crate::{misc::RapierContextMethods, query::QueryQuerySimple, *};

pub enum MoveType {
    TranslateTo(Vec3),
    RotateTo(Quat),
}

pub struct RequestedMove {
    pub requesting: Entity,
    pub ignore: HashSet<Entity>,
    pub move_type: MoveType,
    pub snap_flag: bool,
}

impl RequestedMove {
    pub fn new(requesting: Entity, move_type: MoveType) -> Self {
        Self {
            requesting,
            ignore: default(),
            move_type,
            snap_flag: false,
        }
    }
    
    pub fn snapping(mut self) -> Self {
        self.snap_flag = true;
        self
    }
    
    pub fn ignore(mut self, ignore: HashSet<Entity>) -> Self {
        self.ignore = ignore;
        self
    }
}

pub fn do_requested_move(
    mut requested_moves: EventReader<RequestedMove>,
    mut q_transform: Query<&mut Transform>,
    q_children: Query<&Children>,
    q_collider: Query<(Entity, &Collider)>,
    has_rigidbody: Query<With<RigidBody>>,
    q_global_transform: Query<&GlobalTransform>,
    rapier_ctx: Res<RapierContext>,
    // mut lines: ResMut<DebugLines>,
) {
    use MoveType::*;
    
    for requested_move in requested_moves.iter() {
        let mut colliders = q_children
            .iter_descendants(requested_move.requesting)
            .filter_map(|e| q_collider.get(e).ok())
            .collect::<Vec<_>>();
        let ignore = colliders.iter().map(|(e, _)| *e).collect::<Vec<_>>();
        colliders.retain(|(e, _)| has_rigidbody.get(*e).is_ok());

        let predicate = |e| !ignore.contains(&e);
        let filter = QueryFilter::new().exclude_sensors().predicate(&predicate);

        let requesting = q_transform.entity(requested_move.requesting).clone();
        let mut diff = requesting;

        match requested_move.move_type {
            TranslateTo(to) => diff.translation -= to,
            RotateTo(to) => diff.rotation = to * requesting.rotation.inverse(),
        }

        for (e, c) in &colliders {
            let mut transformed = q_global_transform.entity(*e).compute_transform();
            
            // move this thingy
            match requested_move.move_type {
                TranslateTo(_) => transformed.translation -= diff.translation,
                RotateTo(_) => transformed.rotate_around(requesting.translation, diff.rotation),
            }
            
            // transformed.rotate_around(requesting.translation, diff.rotation);

            // if we detect a collision
            if let Some(_) = rapier_ctx.intersection_with_shape_transform(transformed, c, filter) {
                // oh noes a collision! panic!
                // info!("{:#?} colllision with {:#?}", c, e);
                return;
                // jk no panic
            }
        }
        // info!("good to go");
        // dbg!(colliders);

        if let Ok(mut transform) = q_transform.get_mut(requested_move.requesting) {
            // were good
            match requested_move.move_type {
                TranslateTo(to) => transform.translation = to,
                RotateTo(to) => transform.rotation = to,
            }
        } else {
            // error!("Could not find transform component on requested_move.requesting, also this shouldnt have hapenned")
        }
    }
}
