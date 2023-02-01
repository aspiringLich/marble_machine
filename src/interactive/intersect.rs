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

    pub fn to_transform(&self) -> Transform {
        use MoveType::*;
        match self.move_type {
            TranslateTo(to) => Transform::from_translation(to),
            RotateTo(to) => Transform::from_rotation(to),
        }
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

        // move this thingy
        let transform = |factor: f32, transform: Transform| match requested_move.move_type {
            TranslateTo(_) => {
                Transform::from_translation(transform.translation - diff.translation * factor)
            }
            RotateTo(_) => {
                let mut cpy = transform;
                cpy.rotate_around(requesting.translation, diff.rotation * factor);
                cpy
            }
        };

        // this tests every collider to see if any of then satisfy the
        let test = |factor: f32| {
            colliders
                .iter()
                .map(|(e, c)| {
                    rapier_ctx.intersection_with_shape_transform(
                        transform(factor, q_global_transform.entity(*e).compute_transform()),
                        c,
                        filter,
                    )
                })
                .any(|x| x.is_some())
        };

        // let mut factor = 0.5;
        if test(1.0) {
            const N: f32 = 64.0;
            for i in 0..N as i32 {
                let factor = 1.0 - (i as f32 / N);
                if !test(factor) &&
                let Ok(tf) = q_transform.get(requested_move.requesting){
                    let mut out = unsafe { q_transform.get_unchecked(requested_move.requesting) }.unwrap();
                    
                    *out = transform(factor - 1.0 / N, *tf);
                    if requested_move.snap_flag {
                        let pos = &mut out.translation;
                        pos.x = (pos.x - 0.5).round() + 0.5;
                        pos.y = (pos.y - 0.5).round() + 0.5;
                    }
                    
                    return;
                }
            }
            return;
        }

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
