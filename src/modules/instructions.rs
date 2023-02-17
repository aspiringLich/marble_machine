use bevy::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::res::ZOrder;

use super::BodyType;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Instruction {
    pub offset: Vec3,
    pub ext: f32,
    pub rotation: f32,
}

impl Instruction {
    pub fn new(offset: Vec3, ext: f32, rotation: f32) -> Self {
        Self {
            offset,
            ext,
            rotation
        }
    }
    
    pub fn root(&self, z: f32) -> Transform {
        Transform::from_translation(self.offset + z * Vec3::Z)
            .with_rotation(
                Quat::from_rotation_z(self.rotation)
            )
    }

    pub fn child(&self) -> Transform {
        Transform::from_xyz(self.ext, 0.0, 0.0)
    }
    
    pub fn overall(&self) -> Transform {
        let mut tf = Transform::from_translation(Vec3::X * self.ext);
        tf.rotate_around(Vec3::ZERO, Quat::from_rotation_z(self.rotation));
        tf.translation += self.offset;
        tf
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SpawnInstructions {
    pub body: BodyType,
    pub inputs: Vec<Instruction>,
    pub outputs: Vec<Instruction>,
}

impl SpawnInstructions {
    pub fn from_body(body: BodyType) -> Self {
        Self { body, ..default() }
    }

    pub fn with_input_rotations<T: IntoIterator<Item = f32>>(
        mut self,
        rotations: T,
        d_offset: f32
    ) -> Self {
        self.inputs = rotations
            .into_iter()
            .map(|r| {
                Instruction::new(
                    Vec3::ZERO,
                    self.body.offset() + 1.0 + d_offset,
                    r.to_radians()
                )
            })
            .collect();
        self
    }

    pub fn with_output_rotations<T: IntoIterator<Item = f32>>(
        mut self,
        output_transforms: T,
        d_offset: f32
    ) -> Self {
        self.outputs = output_transforms
            .into_iter()
            .map(|r| {
                Instruction::new(
                    Vec3::ZERO,
                    self.body.offset() + 0.5 + d_offset,
                    r.to_radians()
                )
            })
            .collect();
        self
    }
}