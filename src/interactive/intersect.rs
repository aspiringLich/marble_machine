use crate::{misc::CastShapeTransform, *};

#[derive(Deref, DerefMut, Resource, Default)]
pub struct IntersectingModules(Vec<Entity>);

pub fn update_intersecting_modules(
    In(input): In<Option<Entity>>,
    ctx: Res<RapierContext>,
    q_children: Query<&Children>,
    q_static_physics_obj: Query<(&RigidBody, Entity, &Transform, &Collider), Without<Sensor>>,
) {
    let Some(dragged) = input else { return };

    let is_fixed_collider = |e| {
        // if its a dynamic rigidbody that isn't a sensor
        if let Ok((rigidbody, entity, transform, collider)) = q_static_physics_obj.get(e)
        && *rigidbody == RigidBody::Fixed {
            return Some((entity, transform, collider));
        } else {
            None
        }
    };

    for (_, transform, collider) in q_children
        .iter_descendants(dragged)
        .filter_map(is_fixed_collider)
    {
        let intersect = ctx.cast_shape_transform(
            *transform,
            collider.clone(),
            Vect::ZERO,
            f32::INFINITY,
            QueryFilter::only_fixed().exclude_sensors(),
        );

        if let Some((e, _)) = intersect {}
    }
}

pub fn draw_intersection_warnings() {}
