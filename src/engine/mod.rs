use crate::*;

pub mod components;
pub mod lifetime;
pub mod marble;
pub mod marble_io;
pub mod spawn;
pub mod module_state;

pub fn app(app: &mut App) {
    app.add_system(lifetime::update_lifetime)
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(spawn::spawn_modules.label("spawn::spawn_modules"))
                .with_system(marble_io::fire_marbles)
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(marble::despawn_marbles)
                .with_system(marble_io::update_inputs)
        );
}