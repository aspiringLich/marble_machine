pub mod save_load;
pub mod level;

use crate::*;

pub fn app(app: &mut App) {
    app.add_event::<save_load::SaveWorld>()
        .add_event::<save_load::LoadWorld>()
        .add_system(save_load::save_world)
        .add_system_to_stage(
            CoreStage::PreUpdate,
            save_load::load_world.before("spawn::spawn_modules")
        )
        .init_non_send_resource::<level::Lua>()
        .add_startup_system(level::load_levels);
}