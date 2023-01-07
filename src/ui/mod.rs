use crate::*;

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
    );
}
