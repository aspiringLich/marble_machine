use crate::{graphics::atlas::AtlasDictionary, *};

use bevy_egui::*;
use egui::{Image, *};

/// image button but its from an atlas texture
#[derive(Deref, DerefMut)]
pub struct AtlasImage(pub Image);

impl AtlasImage {
    pub fn new<T: AtlasDictionary>(sprite: T, texture_id: TextureId) -> Self {
        Self::new_scaled(sprite, texture_id, 1.0)
    }

    pub fn new_scaled<T: AtlasDictionary>(
        sprite: T,
        texture_id: TextureId,
        scaling_factor: f32,
    ) -> Self {
        let atlas_size = T::atlas_rect();
        let rect = sprite.rect();
        let uv: egui::Rect = egui::Rect::from_min_max(
            Pos2::new(rect.min.x / atlas_size.x, rect.min.y / atlas_size.y),
            Pos2::new(rect.max.x / atlas_size.x, rect.max.y / atlas_size.y),
        );
        let rect = egui::Rect {
            min: Pos2::new(0., 0.),
            max: Pos2::new(rect.width(), rect.height()),
        };
        Self(Image::new(texture_id, rect.size() * scaling_factor).uv(uv))
    }
}

impl Widget for &AtlasImage {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(self)
    }
}

pub trait GuiImages: FromWorld {
    
}