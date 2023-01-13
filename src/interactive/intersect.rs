use bevy::utils::HashSet;

use crate::{misc::CastShapeTransform, query::QueryQuerySimple, *};

#[derive(Deref, DerefMut, Resource, Default)]
pub struct IntersectingModules(HashSet<Entity>);

pub fn update_intersecting_modules(
    In(input): In<Option<Entity>>,
    ctx: Res<RapierContext>,
    q_parent: Query<&Parent>,
    q_children: Query<&Children>,
    q_static_physics_obj: Query<(&RigidBody, Entity, &Collider), Without<Sensor>>,
    q_global_transform: Query<&GlobalTransform>,
    q_name: Query<&Name>,
    mut intersecting_modules: ResMut<IntersectingModules>,
) {
    let Some(dragged) = input else { return };

    let is_fixed_collider = |e| {
        // if its a dynamic rigidbody that isn't a sensor
        if let Ok((rigidbody, entity, collider)) = q_static_physics_obj.get(e)
        && *rigidbody == RigidBody::Fixed {
            return Some((entity, collider));
        } else {
            None
        }
    };

    debug_assert!(q_parent.get(dragged).is_err());

    let colliders = q_children
        .iter_descendants(dragged)
        .filter_map(is_fixed_collider)
        .collect::<Vec<_>>();
    let entities = colliders.iter().map(|(e, _)| *e).collect::<Vec<_>>();

    macro test_collider($test:expr, $collider:expr, $($tail:tt)*) {{
        let transform = q_global_transform.entity($test).compute_transform();
        ctx.cast_shape_transform(
            transform,
            $collider,
            // for some reason this does not work without a velocity
            Vect::splat(1.0 / 128.0),
            1.0,
            QueryFilter::only_fixed()
                .exclude_sensors()
                .predicate(&$($tail)*),
        )
    }}

    // remove the ones that arent colliding with anything
    intersecting_modules.drain_filter(|&e| {
        let colliders = q_children
            .iter_descendants(e)
            .filter_map(is_fixed_collider)
            .collect::<Vec<_>>();
        let entities = colliders.iter().map(|(e, _)| *e).collect::<Vec<_>>();

        // get the collider
        colliders.iter().all(|(e, collider)| {
            // query the collider
            if let Some((e, _)) = test_collider!(*e, (*collider).clone(), |other| {
                other != *e && !entities.contains(e)
            }) {
                // if the top-level intersecting object is not equal to the original
                let parent = q_parent.iter_ancestors(e).last().unwrap_or(e);
                parent != e
            } else {
                false
            }
        })
    });

    // see if we have to add anything new
    for (test, collider) in colliders {
        // all the colliders weve already seen
        let mut visited: HashSet<_> = default();

        // keep trying intersections until we run out
        loop {
            let intersect = test_collider!(test, collider.clone(), |e| {
                let parent = q_parent.iter_ancestors(e).last().unwrap_or(e);
                !visited.contains(&parent) && !entities.contains(&parent)
            });

            // if they intersect:
            if let Some((e, _)) = intersect {
                // get the parent and the original and mark em off as intersecting
                let parent = q_parent.iter_ancestors(e).last().unwrap_or(e);
                intersecting_modules.insert(parent);
                intersecting_modules.insert(dragged);
                visited.insert(parent);
                visited.insert(dragged);
                // dbg!(&visited);
            } else {
                break;
            }
        }
    }

    dbg!(&**intersecting_modules);
}

pub fn draw_intersection_warnings() {}
