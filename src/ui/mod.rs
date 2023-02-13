use crate::*;
use bevy_egui::{egui, EguiContext};

use egui::{Color32, Style, Vec2};

pub mod atlas_image;

// module spawner thingy
pub mod spawning;
// info panel
pub mod info;
pub mod ui;

pub fn app(app: &mut App) {
    app.add_system_set_to_stage(
        Label::StageUi,
        SystemSet::new()
            .with_system(info::ui)
            .with_system(ui::inspector_ui)
            // .with_system(ui::spawning_ui)
            .with_system(spawning::ui)
            .with_system(ui::debug_ui),
    )
    .add_startup_system_to_stage(Label::StartupStageStart, set_style);
}

fn set_style(mut context: ResMut<EguiContext>, mut commands: Commands) {
    use bevy_egui::egui::style::*;
    use bevy_egui::egui::*;
    use epaint::Shadow;

    commands.init_resource::<spawning::Images>();

    let mut style: Style = context.ctx_mut().style().as_ref().clone();

    let Style {
        ref mut spacing,
        ref mut visuals,
        // ref mut debug,
        ..
    } = style;
    
    use egui::FontFamily::*;
    
    let factor = 1.1;
    
    style.text_styles = [
        (TextStyle::Small, FontId::new(9.0 * factor, Proportional)),
        (TextStyle::Body, FontId::new(12.5 * factor, Proportional)),
        (TextStyle::Button, FontId::new(12.5 * factor, Proportional)),
        (TextStyle::Heading, FontId::new(18.0 * factor, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0 * factor, Monospace)),
    ].into();

    *visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(Color32::from_gray(220)),
        window_shadow: Shadow::small_light(),
        window_fill: Color32::rgba_u32(0x000000d0),
        panel_fill: Color32::rgba_u32(0x000000d0),
        window_rounding: Rounding::none(),
        ..default()
    };
    
    spacing.button_padding = Vec2::ZERO;
    style.debug.debug_on_hover = true;
    style.debug.show_resize = true;

    visuals.widgets.active.bg_fill = Color32::rgba_u32(0xffffff10);
    visuals.widgets.hovered.bg_fill = Color32::rgba_u32(0xffffff02);
    visuals.widgets.inactive.bg_fill = Color32::rgba_u32(0x00000000);

    context.ctx_mut().set_style(style);
}
