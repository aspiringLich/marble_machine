use bevy::prelude::*;

use crate::res::ZOrder;

use super::BodyType;

#[derive(Default)]
pub struct SpawnInstructions {
    pub body: BodyType,
    pub input_transforms: Vec<Transform>,
    pub output_transforms: Vec<Transform>,
}

impl SpawnInstructions {
    pub fn from_body(body: BodyType) -> Self {
        Self { body, ..default() }
    }

    pub fn with_input_rotations<T: IntoIterator<Item = f32>>(
        mut self,
        input_transforms: T,
    ) -> Self {
        self.input_transforms = input_transforms
            .into_iter()
            .map(|r| {
                let rot = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, r.to_radians());
                let mut transform = Transform::from_xyz(
                    self.body.offset() + 1.0,
                    0.0,
                    ZOrder::InputComponent.f32(),
                );
                transform.rotate_around(Vec3::ZERO, rot);
                transform
            })
            .collect();
        self
    }

    pub fn with_output_rotations<T: IntoIterator<Item = f32>>(
        mut self,
        output_transforms: T,
    ) -> Self {
        self.output_transforms = output_transforms
            .into_iter()
            .map(|r| {
                let rot = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, r.to_radians());
                let mut transform = Transform::from_xyz(
                    self.body.offset() + 0.5,
                    0.0,
                    ZOrder::OutputComponent.f32(),
                );
                transform.rotate_around(Vec3::ZERO, rot);
                transform
            })
            .collect();
        self
    }
}
