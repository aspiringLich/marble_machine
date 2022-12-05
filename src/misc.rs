use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use std::f32::consts::TAU;

pub mod marker {
    use bevy::prelude::Component;

    /// marks modules
    #[derive(Component)]
    pub struct Module;

    /// marks module bodies
    #[derive(Component)]
    pub struct ModuleBody;

    /// marks marble inputs
    #[derive(Component)]
    pub struct Input;

    /// marks marble outputs
    #[derive(Component)]
    pub struct Output;

    /// marks the main camera
    #[derive(Component)]
    pub struct Camera;
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

pub macro vec2($x:expr, $y:expr) {
    Vec2::new($x as f32, $y as f32)
}

pub struct Wrapper<T>(T);

impl<T> std::ops::Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Wrapper<T> {
    pub fn new(t: T) -> Self {
        Wrapper(t)
    }
}
