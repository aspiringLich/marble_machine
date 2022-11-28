use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use std::f32::consts::TAU;

pub mod marker {
    use bevy::prelude::Component;

    /// marks marble inputs
    #[derive(Component)]
    pub struct Input;

    /// marks marble outputs
    #[derive(Component)]
    pub struct Output;
}

pub trait ChildrenMatches {
    fn get(&self) -> &Children;

    fn get_matching<'a, Q>(
        &'a self,
        q: &'a Query<'a, 'a, Q, ()>,
    ) -> impl Iterator<Item = Entity> + 'a
    where
        Q: WorldQuery,
    {
        self.get()
            .iter()
            .filter_map(move |&e| q.get(e).ok().and(Some(e)))
    }
}

impl ChildrenMatches for Children {
    fn get(&self) -> &Children {
        self
    }
}

pub const DEG_TO_RAD: f32 = TAU / 360.0;
