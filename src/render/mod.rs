use crate::*;

pub mod atlas;
pub mod grid;

pub fn app(app: &mut App) {
    app.insert_resource(ClearColor(Color::rgb_u32(0x505e71)))
        .add_startup_system(grid::spawn_background);
}
