use std::f32::consts;

use crate::{
    engine::spawn::{BodyType, SpawnInstructions},
    graphics::atlas::{basic, AtlasDictionary},
    module::{param::ModuleResources, Module},
    spawn::SpawnModule,
    *,
};
use bevy_egui::*;
use egui::{Button, Image, Rect, Vec2, *};

use super::atlas_image::AtlasImage;

pub struct Images {
    body_small: Image,
}

const SCALING: f32 = 5.0;

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let assets;
        let mut ctx;

        unsafe {
            assets = world
                .get_resource::<Assets<TextureAtlas>>()
                .expect("resource exists");
            ctx = world
                .get_resource_unchecked_mut::<EguiContext>()
                .expect("resource exists");
        }

        macro new_atlas($sprite:expr) {{
            let texture = assets.get(&$sprite.info().0).unwrap().texture.clone();
            AtlasImage::new_scaled($sprite, ctx.add_image(texture), SCALING)
        }}

        Self {
            body_small: new_atlas!(basic::body_small).tint(BodyType::Small.color32()),
        }
    }
}

pub enum ModuleItem {
    Module {
        module: ModuleType,
        instructions: &'static SpawnInstructions,
    },
    SectionHeader(&'static str),
}

#[ctor]
static MODULES: Vec<ModuleItem> = {
    use crate::engine::module::*;

    macro item($arg:expr) {
        ModuleItem::Module {
            module: $arg,
            instructions: $arg.get_inner().spawn_instructions(),
        }
    }
    vec![
        item!(ModuleType::Basic(Basic)),
        item!(ModuleType::Basic(Basic)),
        item!(ModuleType::Basic(Basic)),
    ]
};

const SIZE: Vec2 = Vec2::new(120., 120.);

pub fn ui(mut egui_context: ResMut<EguiContext>, images: Local<Images>) {
    let ctx = egui_context.ctx_mut();

    let size_rect = Rect {
        min: Pos2::ZERO,
        max: SIZE.to_pos2(),
    };

    SidePanel::left("spawning").resizable(true).show(ctx, |ui| {
        let spacing = ui.spacing().window_margin.top;

        let width = (ui.available_size().x) / (SIZE.x + spacing);
        let width = width.round() as i32;
        // dbg!(width);

        let mut iter = MODULES.iter().peekable();

        while let Some(_) = iter.peek() {
            ui.add_space(spacing);
            let mut i = 0;
            let cursor = ui.cursor().min.to_vec2();
            let mut label = None;

            while let Some(item) = iter.next() && i < i32::max(width, 1) {
                match item {
                    ModuleItem::Module {
                        module: _,
                        instructions,
                    } => {
                        // dbg!(cursor);
                        // allocate space
                        let translate = cursor + Vec2::X * ((SIZE.x + spacing * 2.0) * i as f32);
                        let allocated = size_rect.translate(translate);
                        ui.put(allocated, Button::new(""));
                        ui.allocate_rect(allocated, Sense::focusable_noninteractive());
                        let mut new_ui = ui.child_ui(allocated, Layout::default());
                        recreate_module(&mut new_ui, &images, instructions);

                        i += 1;
                    }
                    ModuleItem::SectionHeader(str) => {
                        label = Some(str);
                    }
                }

                if let Some(str) = label {
                    ui.label(*str);
                }
            }
        }

        ui.set_width(ui.min_size().x);
    });
}

fn recreate_module(ui: &mut Ui, images: &Images, instructions: &SpawnInstructions) {
    let make_rect = |center: Vec2, size: Vec2| {
        let min: Vec2 = center - size / 2.0 + ui.max_rect().min.to_vec2();
        let max: Vec2 = center + size / 2.0 + ui.max_rect().min.to_vec2();
        Rect::from_min_max(min.to_pos2(), max.to_pos2())
    };

    let center = SIZE / 2.0;

    // spawn inputs
    for &transform in instructions.input_transforms.iter() {
        // let size =
    }

    // spawn body
    let atlas_image = match instructions.body {
        spawn::BodyType::Small => &images.body_small,
        spawn::BodyType::Large => todo!(),
    };

    let size = atlas_image.size();
    ui.put(make_rect(center, size), *atlas_image);
}
