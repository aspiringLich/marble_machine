use crate::*;

pub mod atlas;
pub mod grid;

pub fn app(app: &mut App) {
    app.insert_resource(grid::Grid {
        size: 40,
        active: true,
    })
    .add_system(grid::spawn_background);
}
