use crate::*;

pub mod components;
pub mod lifetime;
pub mod marble;
pub mod marble_io;
pub mod module;
pub mod spawn;

pub fn app(app: &mut App) {
    app.add_system(lifetime::update_lifetime);
}
