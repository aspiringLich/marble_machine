use bevy::prelude::*;

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
