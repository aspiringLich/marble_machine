use crate::*;

pub mod atlas;
pub mod grid;

pub fn app(app: &mut App) {
    app.insert_resource(ClearColor(Color::hsl(216.0, 0.23, 0.50)))
        .add_startup_system(grid::spawn_background);
}
