use crate::{
    modules::{ body::BodyType, ModuleType, SpawnInstructions },
    graphics::atlas::{ basic, AtlasDictionary },
    *,
};
use bevy_egui::*;
use egui::{ Button, Image, Label, Rect, Vec2, * };
// use once_cell::sync::Lazy;

use super::{ atlas_image::AtlasImage, info::HoveredModule };

pub struct ImageItem {
    pub small: Image,
    pub large: Image,
}

const SCALING: f32 = 3.5;

impl ImageItem {
    const LARGE_SCALING: f32 = super::info::WIDTH / SIZE.x / 1.3;
}

#[derive(Resource)]
pub struct Images {
    pub body_small: ImageItem,
    pub input: ImageItem,
    pub output: ImageItem,
    pub indicator: ImageItem,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let assets;
        let mut ctx;

        unsafe {
            assets = world.get_resource::<Assets<TextureAtlas>>().expect("resource exists");
            ctx = world.get_resource_unchecked_mut::<EguiContext>().expect("resource exists");
        }

        macro new_atlas(
            $sprite:expr;
            $($tail:tt)*
        ) {
        {
            let texture = assets.get(&$sprite.info().0).unwrap().texture.clone();
            ImageItem {
                small: (*AtlasImage::new_scaled($sprite, ctx.add_image(texture.clone()), SCALING)) $($tail)*,
                large: (*AtlasImage::new_scaled($sprite, ctx.add_image(texture), SCALING * ImageItem::LARGE_SCALING)) $($tail)*,
            }
        }
        }

        Self {
            body_small: new_atlas!(basic::body_small; .tint(BodyType::Small.color32())),
            input: new_atlas! {
                basic::marble_input;
            },
            output: new_atlas! {
                basic::marble_output;
            },
            indicator: new_atlas! {
                basic::indicator;
            },
        }
    }
}

#[derive(Debug)]
pub enum ModuleItem {
    Module {
        module: ModuleType,
    },
    SectionHeader(&'static str),
}

#[ctor::ctor]
static MODULES: Vec<ModuleItem> =  {
    use crate::modules::ModuleType::*;
    let item = |module| {
        ModuleItem::Module {
            module,
        }
    };
    let header = |name| { ModuleItem::SectionHeader(name) };

    vec![header("Basic"), item(Basic), item(Basic), item(Basic)]
};

pub const SIZE: Vec2 = Vec2::new(80.0, 80.0);

pub fn ui(
    mut egui_context: ResMut<EguiContext>,
    images: Res<Images>,
    // mut windows: ResMut<Windows>,
    mut spawn_modules: EventWriter<spawn::SpawnModule>,
    mut hovered: ResMut<HoveredModule>
) {
    // let Some(window) = windows.get_primary_mut() else { error!("take a guess what the error is"); return };

    let ctx = egui_context.ctx_mut();

    let size_rect = Rect {
        min: Pos2::ZERO,
        max: SIZE.to_pos2(),
    };

    // set cursor

    SidePanel::left("spawning")
        .resizable(true)
        .default_width(SIZE.x * 2.0)
        .show_separator_line(true)
        .show(ctx, |ui| {
            let spacing = ui.spacing().window_margin.top;

            let width = ui.available_size().x / (SIZE.x + spacing);
            let width = width.round();
            ui.set_width(width * SIZE.x + spacing);
            // dbg!(width);
            
            let mut iter = MODULES.iter().peekable();

            let mut set = None;

            while iter.peek().is_some() {
                ui.add_space(spacing);
                let mut i = 0;
                let cursor = ui.cursor().min.to_vec2();

                while i < i32::max(width as i32, 1) && let Some(item) = iter.next() {
                    // dbg!(item);
                    match item {
                        ModuleItem::Module { module } => {
                            // dbg!(cursor);
                            // allocate space
                            let translate =
                                cursor + Vec2::X * ((SIZE.x + spacing + 5.0) * (i as f32));
                            let allocated = size_rect.translate(translate);
                            // dbg!(allocated.min);

                            // put down the button
                            let button = ui.put(allocated, Button::new(""));

                            if button.hovered() {
                                set = Some(module);
                            }
                            if button.clicked() {
                                spawn_modules.send(spawn::SpawnModule::new(*module).place());
                            }

                            // allocate the area to draw the module and throw stuff there
                            ui.allocate_rect(allocated, Sense::hover()).hovered();

                            let mut new_ui = ui.child_ui(allocated, Layout::default());
                            recreate_module(&mut new_ui, &images, module.spawn_instructions(), false);

                            i += 1;
                        }
                        ModuleItem::SectionHeader(str) => {
                            ui.add(Label::new(*str));
                            break;
                        }
                    }
                }
            }

            **hovered = set.copied();

            ui.set_width(ui.min_size().x);
        });
}

pub fn recreate_module(
    ui: &mut Ui,
    images: &Images,
    instructions: &SpawnInstructions,
    large: bool
) {
    let scaling = if large { ImageItem::LARGE_SCALING } else { 1.0 };

    let ui_min = ui.max_rect().min.to_vec2();
    let ui_center = (ui.max_rect().max - ui.max_rect().min) / 2.0;
    let make_rect = |center: Vec2, size: Vec2| {
        let min: Vec2 = center - size / 2.0 + ui_min;
        let max: Vec2 = center + size / 2.0 + ui_min;
        Rect::from_min_max(min.to_pos2(), max.to_pos2())
    };

    let angle = -std::f32::consts::PI / 4.0;

    macro put($center:expr, $image:expr) {
        let image = if large {
            $image.large     
        } else {
            $image.small
        };
        ui.put(
            make_rect($center, image.size()),
            image.rotate(angle, Vec2::splat(0.5)),
        );
    }
    macro put_tf($instruction:expr, $image:expr) {
        let mut transform = $instruction.overall();
        // rotate 45 deg
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle),
        );

        let center = transform.translation.truncate() * SCALING * scaling;
        let center = Vec2 {
            x: center.x,
            y: center.y,
        } + ui_center;
        
        let image = if large {
            $image.large     
        } else {
            $image.small
        };

        ui.put(
            make_rect(center, image.size()),
            image.rotate(
                transform.rotation.to_euler(EulerRot::XYZ).2,
                Vec2::splat(0.5),
            ),
        );
    }

    let extend_pos = |pos: &mut Vec3, mut units: f32| {
        units *= (SCALING / 2.0) * scaling;
        let len = pos.length();
        *pos = pos.normalize() * (len + units);
    };

    // spawn inputs
    for instruction in instructions.inputs.iter() {
        put_tf!(instruction, images.input);
    }
    // spawn outputs
    for instruction in instructions.outputs.iter() {
        put_tf!(instruction, images.output);
    }

    // spawn body
    let atlas_image = match instructions.body {
        BodyType::Small => &images.body_small,
        BodyType::Large => todo!(),
    };
    put!(ui_center, *atlas_image);

    // spawn indicators
    for instruction in instructions.inputs.iter() {
        let mut i = instruction.clone();
        i.ext -= 2.0;
        put_tf!(i, images.indicator);
    }
}