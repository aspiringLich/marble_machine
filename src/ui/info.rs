use crate::*;
use crate::modules::ModuleType;
use crate::modules::header::SpawnInstructions;
use crate::ui::spawning;
use bevy_egui::*;
use egui::Sense;
use egui::Align2;
use egui::Layout;

use super::spawning::Images;

pub const WIDTH: f32 = 150.0;

#[derive(Deref, DerefMut, Resource, Default)]
pub struct HoveredModule(Option<(&'static SpawnInstructions, &'static str)>);

pub fn ui(
    mut egui_ctx: ResMut<EguiContext>,
    selected: Res<SelectedModules>,
    q_module: Query<&ModuleType>,
    // q_parent: Query<&Parent>,
    // q_body: Query<With<ModuleBody>>,
    images: Res<Images>,
    hovered: Res<HoveredModule>
) {
    let ctx = egui_ctx.ctx_mut();
    
    let mut instruction: SpawnInstructions = SpawnInstructions::default();
    let Some((instructions, name)) = hovered.or_else(||
        selected.selected.map(|e| {
            let module = q_module.get(e).unwrap();
            instruction = module.spawn_instructions();
            (&instruction, module.get_name())
        })
    ) else {
        return;
    };

    let height = 300.0;
    let margin = 10.0;

    // dbg!(&screen);

    egui::Window
        ::new(name)
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::RIGHT_CENTER, [-margin, 0.0])
        .min_width(WIDTH)
        .show(ctx, |ui| {
            use egui::{ Vec2 };

            ui.set_width(WIDTH);
            ui.set_height(height);

            // let rect = Rect::from_x_y_ranges(cursor.x..=cursor.WIDTH, 0.0..=WIDTH);
            let (rect, _) = ui.allocate_at_least(
                Vec2::new(WIDTH, WIDTH),
                Sense::focusable_noninteractive()
            );
            let mut child = ui.child_ui(rect, Layout::default());
            spawning::recreate_module(&mut child, &images, instructions, true);

            ui.label("uyes");
        });
}