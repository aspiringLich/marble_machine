use crate::{misc::CastShapeTransform, query::QueryQuerySimple, *};

#[derive(Deref, DerefMut, Resource, Default)]
pub struct IntersectingModules(Vec<Entity>);

pub fn update_intersecting_modules(
    In(input): In<Option<Entity>>,
    ctx: Res<RapierContext>,
    q_children: Query<&Children>,
    q_parent: Query<&Parent>,
    q_static_physics_obj: Query<(&RigidBody, Entity, &Collider), Without<Sensor>>,
    q_transform: Query<&Transform>,
    q_global_transform: Query<&GlobalTransform>,
    q_name: Query<&Name>,
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

    let colliders = q_children
        .iter_descendants(dragged)
        .filter_map(is_fixed_collider)
        .collect::<Vec<_>>();
    let entities = colliders.iter().map(|(e, _)| *e).collect::<Vec<_>>();

    // for every collider found
    for (test, collider) in colliders {
        let transform = q_global_transform.entity(test).compute_transform();

        let intersect = ctx.cast_shape_transform(
            transform,
            collider.clone(),
            // for some reason this does not work without a velocity
            Vect::splat(1.0 / 128.0),
            1.0,
            QueryFilter::only_fixed()
                .exclude_sensors()
                .predicate(&|e| !entities.contains(&e)),
        );

        if let Some((e, _)) = intersect {
            dbg!(q_name.get(e));
        }
    }
}

pub fn draw_intersection_warnings() {}
