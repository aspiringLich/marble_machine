use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use std::f32::consts::TAU;

pub mod marker {
    use bevy::prelude::Component;
    use derive_more::{Deref, DerefMut};

    /// marks modules
    #[derive(Component)]
    pub struct Module;

    /// marks module bodies
    #[derive(Component)]
    pub struct ModuleBody;

    /// marks marble inputs
    #[derive(Component, Deref, DerefMut)]
    pub struct Input(pub usize);

    /// marks marble outputs
    #[derive(Component, Deref, DerefMut)]
    pub struct Output(pub usize);

    /// marks those funny indicator lights
    #[derive(Component)]
    pub struct Indicator;

    /// marks the main camera
    #[derive(Component)]
    pub struct Camera;
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
