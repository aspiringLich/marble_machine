use crate::*;
use bevy_egui::{egui, EguiContext};

use egui::{Color32, Style, Vec2};

pub mod atlas_image;

// the following contain a ui function for ui stuff
pub mod spawning;
pub mod ui;

pub fn app(app: &mut App) {
    app.add_system_set_to_stage(
        Label::StageUi,
        SystemSet::new()
            .with_system(ui::inspector_ui)
            .with_system(ui::spawning_ui)
            .with_system(spawning::ui)
            .with_system(ui::debug_ui),
    )
    .add_startup_system(set_style);
}

fn set_style(mut context: ResMut<EguiContext>) {
    let mut style: Style = context.ctx_mut().style().as_ref().clone();

    let Style {
        ref mut spacing,
        ref mut visuals,
        // ref mut debug,
        ..
    } = style;

    visuals.panel_fill = Color32::rgba_u32(0x000000f0);
    spacing.button_padding = Vec2::ZERO;
    // debug.debug_on_hover = true;
    // debug.show_resize = true;

    visuals.widgets.active.bg_fill = Color32::rgba_u32(0xffffff10);
    visuals.widgets.hovered.bg_fill = Color32::rgba_u32(0xffffff02);
    visuals.widgets.inactive.bg_fill = Color32::rgba_u32(0x00000000);

    context.ctx_mut().set_style(style);
}
