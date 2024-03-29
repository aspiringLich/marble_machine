use crate::{query::QueryQuerySimple, *};

/// a struct that stores the number of ticks left until it gets borked
#[derive(Deref, DerefMut, Component)]
pub struct Lifetime(pub usize);

/// update the lifetimes of entities that have it, and despawn them when the counter reaches 0
#[allow(clippy::type_complexity)]
pub fn update_lifetime(
    mut commands: Commands,
    mut lifetime: ParamSet<(Query<Entity, Added<Lifetime>>, Query<&mut Lifetime>)>,
    mut entities: Local<Vec<Entity>>,
) {
    for entity in lifetime.p0().iter() {
        entities.push(entity);
        // dbg!("e");
    }

    let mut q_lifetime = lifetime.p1();
    // filter out entities that may not exist anymore
    entities.drain_filter(|&mut e| commands.get_entity(e).is_none());
    // do the lifetime thing
    entities
        .drain_filter(|&mut e| {
            let mut lifetime = q_lifetime.entity_mut(e);
            **lifetime -= 1;
            **lifetime == 0
        })
        .for_each(|e| commands.entity(e).despawn());
}
