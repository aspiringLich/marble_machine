use crate::*;

#[derive(Clone, Copy, Debug)]
pub enum MarbleType {
    Bit,
    Num,
}

#[derive(Copy, Clone, Debug, Component)]
pub struct Marble {
    ty: MarbleType,
    val: i32,
}

impl Marble {
    pub fn new(ty: MarbleType, val: i32) -> Self {
        Self {
            ty,
            val,
        }
    }
    
    pub fn get_type(&self) -> MarbleType {
        self.ty
    }
    
    pub fn get_val(&self) -> i32 {
        self.val
    }
    
    pub fn bit(val: bool) -> Self {
        Self {
            ty: MarbleType::Bit,
            val: val as i32,
        }
    }
}

/// despawn marbles if they go too low (and should be despawned cuz theyre out of bounds)
pub fn despawn_marbles(
    mut commands: Commands,
    q_transform: Query<&Transform>,
    q_marbles: Query<Entity, With<Marble>>
) {
    for marble in q_marbles.iter() {
        let transform = q_transform.get(marble).unwrap();
        if transform.translation.y < -1000.0 {
            commands.entity(marble).despawn_recursive();
        }
    }
}