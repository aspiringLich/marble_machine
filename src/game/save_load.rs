use crate::{*, modules::ModuleType};

pub struct SaveWorld(String);

pub fn save_world(q_modules: Query<Entity, With<marker::Module>>, save_events: EventReader<SaveWorld>) {
    for module in q_modules.iter() {
        
    }
}