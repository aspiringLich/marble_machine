
pub mod save_load;

use crate::*;

pub fn app(app: &mut App) {
    app.add_event::<save_load::SaveWorld>().add_system(save_load::save_world);
}