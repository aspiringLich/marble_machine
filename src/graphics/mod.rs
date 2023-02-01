use crate::*;

pub mod atlas;
pub mod grid;

pub fn app(app: &mut App) {
    app.insert_resource(grid::GridInfo::default())
        .insert_resource(ClearColor(Color::hsl(216.0, 0.24, 0.55)))
        .add_system(grid::spawn_background);
}
