use bevy::{ecs::system::EntityCommands, prelude::*};

pub mod marker {
    use bevy::prelude::Component;
    use derive_more::{Deref, DerefMut};

    /// marks modules
    #[derive(Component)]
    pub struct Module;

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

pub trait ColorHex {
    fn rgba_u32(hex: u32) -> Self;
    fn rgb_u32(hex: u32) -> Self;
}

impl ColorHex for Color {
    fn rgba_u32(hex: u32) -> Self {
        Self::rgba_u8(
            (hex >> 24 & 0xff) as u8,
            (hex >> 16 & 0xff) as u8,
            (hex >> 8 & 0xff) as u8,
            (hex & 0xff) as u8,
        )
    }

    fn rgb_u32(hex: u32) -> Self {
        Self::rgb_u8(
            (hex >> 16 & 0xff) as u8,
            (hex >> 8 & 0xff) as u8,
            (hex & 0xff) as u8,
        )
    }
}

impl ColorHex for bevy_egui::egui::Color32 {
    fn rgba_u32(hex: u32) -> Self {
        Self::from_rgba_unmultiplied(
            (hex >> 24 & 0xff) as u8,
            (hex >> 16 & 0xff) as u8,
            (hex >> 8 & 0xff) as u8,
            (hex & 0xff) as u8,
        )
    }

    fn rgb_u32(hex: u32) -> Self {
        Self::from_rgb(
            (hex >> 16 & 0xff) as u8,
            (hex >> 8 & 0xff) as u8,
            (hex & 0xff) as u8,
        )
    }
}

/// a trait to allow us to run the name method on Commands
pub trait CommandsName {
    fn name(&mut self, name: impl Into<std::borrow::Cow<'static, str>>) -> &mut Self;
}

impl<'w, 's, 'a> CommandsName for EntityCommands<'w, 's, 'a> {
    /// set this sprites name
    fn name(&mut self, name: impl Into<std::borrow::Cow<'static, str>>) -> &mut Self {
        let id = self.id();
        self.insert(Name::new(format!("{} ({:#?})", name.into(), id)))
    }
}

impl<'a> CommandsName for bevy::ecs::world::EntityMut<'a> {
    /// set this sprites name
    fn name(&mut self, name: impl Into<std::borrow::Cow<'static, str>>) -> &mut Self {
        let id = self.id();
        self.insert(Name::new(format!("{} ({:#?})", name.into(), id)))
    }
}

pub macro builder_fn{
    ($name:ident, $type:ty, { $($tail:tt)* }) => {
        pub fn $name(mut self, $name: $type) -> Self {
            self.$($tail)*;
            self
        }
    },
    ($name:ident, $type:ty, $($path:tt)*) => {
        pub fn $name(mut self, $name: $type) -> Self {
            self.$($path)* = $name;
            self
        }
    },
    ($name:ident, $type:ty) => {
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = $name;
            self
        }
    },
}

use bevy_rapier2d::prelude::*;

pub trait RapierContextMethods {
    fn cast_shape_transform(
        &self,
        transform: Transform,
        collider: &Collider,
        velocity: Vect,
        max_toi: Real,
        filter: QueryFilter,
    ) -> Option<(Entity, Toi)>;

    fn intersection_with_shape_transform(
        &self,
        transform: Transform,
        shape: &Collider,
        filter: QueryFilter,
    ) -> Option<Entity>;
}

impl RapierContextMethods for RapierContext {
    fn cast_shape_transform(
        &self,
        transform: Transform,
        collider: &Collider,
        velocity: Vect,
        max_toi: Real,
        filter: QueryFilter,
    ) -> Option<(Entity, Toi)> {
        self.cast_shape(
            transform.translation.truncate(),
            transform.rotation.to_euler(EulerRot::XYZ).2,
            velocity,
            collider,
            max_toi,
            filter,
        )
    }

    fn intersection_with_shape_transform(
        &self,
        transform: Transform,
        shape: &Collider,
        filter: QueryFilter,
    ) -> Option<Entity> {
        self.intersection_with_shape(
            transform.translation.truncate(),
            transform.rotation.to_euler(EulerRot::XYZ).2,
            shape,
            filter,
        )
    }
}
