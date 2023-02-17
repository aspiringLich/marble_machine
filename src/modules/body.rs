use crate::{
    graphics::atlas::{basic, AtlasDictionary},
    *,
};
use serde::{Serialize, Deserialize};

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize, Component)]
pub enum BodyType {
    #[default]
    Small,
    Large,
}

pub fn offset_of<T: AtlasDictionary>(input: T) -> f32 {
    f32::round(input.width() * 0.5 + 1.0) - 0.5
}

enum ColliderType {
    Ball,
    Other(Collider),
}

impl BodyType {
    fn collider_type(self) -> ColliderType {
        use BodyType::*;
        use ColliderType::*;
        match self {
            Small => Ball,
            Large => Ball,
        }
    }
    
    fn color_internal(self) -> u32 {
        use BodyType::*;

        let cyanish = 0x65edc0;

        match self {
            Small => cyanish,
            Large => cyanish,
        }
    }
    
    pub fn sprite(self) -> impl AtlasDictionary {
        use BodyType::*;
        match self {
            Small => basic::body_small,
            Large => basic::body,
        }
    }
    
    pub fn offset(self) -> f32 {
        offset_of(self.sprite())
    }

    pub fn color(self) -> Color {
        Color::rgb_u32(self.color_internal())
    }

    pub fn color32(self) -> bevy_egui::egui::Color32 {
        bevy_egui::egui::Color32::rgb_u32(self.color_internal())
    }
    
    pub fn collider(self) -> Collider {
        use ColliderType::*;
        match self.collider_type() {
            Ball => Collider::ball(self.offset() - 1.0),
            Other(collider) => collider,
        }
    }
    
    
}
