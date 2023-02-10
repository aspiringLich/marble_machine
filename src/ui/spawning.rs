use crate::{
    engine::modules::{body::BodyType, ModuleType, SpawnInstructions},
    graphics::atlas::{basic, AtlasDictionary},
    *,
};
use bevy_egui::*;
use egui::{Button, Image, Label, Rect, Vec2, *};
use trait_enum::Deref;

use super::atlas_image::AtlasImage;

#[derive(Resource)]
pub struct Images {
    body_small: Image,
    input: Image,
    output: Image,
    indicator: Image,
}

const SCALING: f32 = 3.5;

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
            input: *new_atlas!(basic::marble_input),
            output: *new_atlas!(basic::marble_output),
            indicator: *new_atlas!(basic::indicator),
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
    use crate::engine::modules::*;

    macro item($arg:expr) {
        ModuleItem::Module {
            module: $arg,
            instructions: $arg.deref().spawn_instructions(),
        }
    }

    macro header($arg:literal) {
        ModuleItem::SectionHeader($arg)
    }

    vec![
        header!("Standard Modules"),
        item!(ModuleType::Basic(Basic)),
        item!(ModuleType::Basic(Basic)),
        item!(ModuleType::Basic(Basic)),
    ]
};

const SIZE: Vec2 = Vec2::new(80., 80.);

pub fn ui(
    mut egui_context: ResMut<EguiContext>,
    images: Res<Images>,
    // mut windows: ResMut<Windows>,
    mut spawn_modules: EventWriter<spawn::SpawnModule>,
) {
    // let Some(window) = windows.get_primary_mut() else { error!("take a guess what the error is"); return };
    
    let ctx = egui_context.ctx_mut();

    let size_rect = Rect {
        min: Pos2::ZERO,
        max: SIZE.to_pos2(),
    };
    
    // set cursor

    SidePanel::left("spawning").resizable(true).default_width(SIZE.x * 2.0).show_separator_line(true).show(ctx, |ui| {        
        let spacing = ui.spacing().window_margin.top;

        let width = (ui.available_size().x) / (SIZE.x + spacing);
        let width = width.round();
        ui.set_width(width as f32 * SIZE.x + spacing);
        // dbg!(width);

        let mut iter = MODULES.iter().peekable();

        while let Some(_) = iter.peek() {
            ui.add_space(spacing);
            let mut i = 0;
            let cursor = ui.cursor().min.to_vec2();

            while i < i32::max(width as i32, 1)&& let Some(item) = iter.next() {
                match item {
                    ModuleItem::Module {
                        module,
                        instructions,
                    } => {
                        // dbg!(cursor);
                        // allocate space
                        let translate = cursor + Vec2::X * ((SIZE.x + spacing + 3.0) * i as f32);
                        let allocated = size_rect.translate(translate);
                        // dbg!(allocated.min);
                        
                        // put down the button
                        let button = ui.put(allocated, Button::new(""));
                        if button.clicked() {
                            spawn_modules.send(spawn::SpawnModule::new(*module).place());
                        }
                        
                        // allocate the area to draw the module and throw stuff there
                        ui.allocate_rect(allocated, Sense::focusable_noninteractive());
                        let mut new_ui = ui.child_ui(allocated, Layout::default());
                        recreate_module(&mut new_ui, &images, instructions, 1.0);

                        i += 1;
                    }
                    ModuleItem::SectionHeader(str) => {
                        ui.add(Label::new(*str).wrap(false));
                        break;
                    }
                }
            }
        }

        ui.set_width(ui.min_size().x);
    });
}

fn recreate_module(ui: &mut Ui, images: &Images, instructions: &SpawnInstructions, scale: f32) {
    let ui_min = ui.max_rect().min.to_vec2();
    let make_rect = |center: Vec2, size: Vec2| {
        let min: Vec2 = center - size / 2.0 + ui_min;
        let max: Vec2 = center + size / 2.0 + ui_min;
        Rect::from_min_max(min.to_pos2(), max.to_pos2())
    };
    
    let angle = -std::f32::consts::PI / 4.0;

    macro put($center:expr, $image:expr) {
        ui.put(make_rect($center, $image.size() * scale), $image.rotate(angle, Vec2::splat(0.5)));
    }
    macro put_tf($transform:expr, $image:expr) {
        let mut transform = $transform;
        // rotate 45 deg
        transform.rotate_around(Vec3::new(0.,0.,$transform.translation.z), Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle));

        let center = transform.translation.truncate() * SCALING;
        let center = Vec2 {
            x: center.x,
            y: center.y
        } + SIZE / 2.0;

        ui.put(make_rect(center, $image.size() * scale), $image.rotate(transform.rotation.to_euler(EulerRot::XYZ).2, Vec2::splat(0.5)));
    }

    let center = SIZE / 2.0;
    
    let extend_pos = |pos: &mut Vec3, mut units: f32| {
        units *= SCALING / 2.0;
        let len = pos.length();
        *pos = pos.normalize() * (len + units);
    };

    // spawn inputs
    for &transform in instructions.input_transforms.iter() {
        put_tf!(transform, images.input);
    }
    // spawn outputs
    for &transform in instructions.output_transforms.iter() {
        put_tf!(transform, images.output);
    }
    
    // spawn body
    let atlas_image = match instructions.body {
        BodyType::Small => &images.body_small,
        BodyType::Large => todo!(),
    };
    put!(center, *atlas_image);
    
    // spawn indicators
    for &transform in instructions.input_transforms.iter() {
        let mut tf = transform;
        extend_pos(&mut tf.translation, -1.0);
        put_tf!(tf, images.indicator);
    }
}
