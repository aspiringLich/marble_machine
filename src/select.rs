use crate::*;
use bevy::render::camera::RenderTarget;
use bevy_rapier2d::prelude::*;

pub fn get_selected(
    mut selected: ResMut<SelectedModule>,
    mut egui_ctx: ResMut<bevy_egui::EguiContext>,
    rapier_context: Res<RapierContext>,
    buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<CursorCoords>,
    q_body: Query<&marker::ModuleBody>,
    q_parent: Query<&Parent>,
) {
    if egui_ctx.ctx_mut().wants_pointer_input() {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        let mut found = false;
        rapier_context.intersections_with_point(
            **mouse_pos,
            QueryFilter::new().predicate(&|entity| q_body.get(entity).is_ok()),
            |entity| {
                // get the parent of the main body and set that as the selected module
                // main body is assumed to be the child of the overall module parent entity
                *selected = SelectedModule(Some(q_parent.get(entity).unwrap().get()));
                // eprintln!("selected {}", q_name.get(entity).unwrap());
                found = true;
                false
            },
        );
        if !found {
            *selected = SelectedModule(None);
        }
    }
}

#[derive(Resource, Debug)]
pub struct CursorCoords(Vec2);

impl Default for CursorCoords {
    fn default() -> Self {
        CursorCoords(Vec2::ZERO)
    }
}

impl std::ops::Deref for CursorCoords {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// get the position of a cursor in absolute world coordinates
pub fn get_cursor_pos(
    window: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<marker::Camera>>,
    mut coords: ResMut<CursorCoords>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        window.get(id).unwrap()
    } else {
        window.get_primary().unwrap()
    };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        coords.0 = world_pos;
    }
}
