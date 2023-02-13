use crate::*;
use crate::engine::modules::ModuleType;
use crate::interactive::hover::HoveredEntities;
use crate::misc::marker::ModuleBody;
use crate::ui::spawning;
use crate::ui::spawning::recreate_module;
use bevy_egui::*;
use egui::Sense;
use egui::Align2;
use egui::Layout;

use super::spawning::Images;

pub const WIDTH: f32 = 150.0;

pub fn ui(
    mut egui_ctx: ResMut<EguiContext>,
    selected: Res<SelectedModules>,
    q_module: Query<&ModuleType>,
    q_parent: Query<&Parent>,
    q_body: Query<With<ModuleBody>>,
    hovered: Res<HoveredEntities>,
    images: Res<Images>
) {
    let ctx = egui_ctx.ctx_mut();

    let Some(selected) = selected.selected.or_else(||
        hovered
            .iter()
            .find(|e| q_body.get(**e).is_ok())
            .map(|e| q_parent.iter_ancestors(*e).next().unwrap())
    ) else {
        return;
    };
    let module = q_module.get(selected).unwrap();

    let height = 300.0;
    let margin = 10.0;

    // dbg!(&screen);

    egui::Window
        ::new(module.get_name())
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::RIGHT_CENTER, [-margin, 0.0])
        .min_width(WIDTH)
        .show(ctx, |ui| {
            use egui::{ Rect, Vec2};

            ui.set_width(WIDTH);
            ui.set_height(height);
            
            let cursor = ui.cursor().min.to_vec2();
            
            // let rect = Rect::from_x_y_ranges(cursor.x..=cursor.WIDTH, 0.0..=WIDTH);
            let (rect, _) = ui.allocate_at_least(Vec2::new(WIDTH, WIDTH), Sense::focusable_noninteractive());
            let mut child = ui.child_ui(rect, Layout::default());
            spawning::recreate_module(&mut child, &images, &module.spawn_instructions(), true);
            
            ui.label("uyes");
        });
}