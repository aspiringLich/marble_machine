use crate::{
    graphics::atlas::{basic, AtlasDictionary},
    *,
};

// macro color($r:expr, $g:expr, $b:expr) {
//     Color::Rgba {
//         red: $r as f32 / 255.0,
//         green: $g as f32 / 255.0,
//         blue: $b as f32 / 255.0,
//         alpha: 1.0,
//     }
// }

// pub static MODULE_COLOR: Color = color!(101, 237, 192);

#[derive(Default)]
pub enum BodyType {
    #[default]
    Small,
    Large,
}

pub fn offset_of<T: AtlasDictionary>(input: T) -> f32 {
    input.width() * 0.5 + 1.0
}

impl BodyType {
    pub fn offset(&self) -> f32 {
        use BodyType::*;
        match self {
            Small => offset_of(basic::body_small),
            Large => todo!(),
        }
    }

    pub fn color(&self) -> Color {
        Color::rgb_u32(self.color_internal())
    }

    pub fn color32(&self) -> bevy_egui::egui::Color32 {
        bevy_egui::egui::Color32::rgb_u32(self.color_internal())
    }

    fn color_internal(&self) -> u32 {
        use BodyType::*;

        let cyanish = 0x65edc0;

        match self {
            Small => cyanish,
            Large => cyanish,
        }
    }
}
