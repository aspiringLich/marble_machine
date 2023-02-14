use crate::*;

pub mod components;
pub mod lifetime;
pub mod marble;
pub mod marble_io;
pub mod spawn;

pub fn app(app: &mut App) {
    app.add_system_to_stage(Label::StageStart, select::get_cursor_pos)
        .add_system(lifetime::update_lifetime)
        .add_system_set_to_stage(
            Label::StageSpawn,
            SystemSet::new()
                .with_system(spawn::spawn_modules.label("spawn::spawn_modules"))
                .with_system(marble_io::fire_marbles)
        )
        .add_system_set_to_stage(
            Label::StageMain,
            SystemSet::new()
                .with_system(marble::despawn_marbles)
                .with_system(marble_io::update_inputs)
                .with_system(modules::update_modules)
                .with_system(modules::update_module_callbacks)
        );
}