use crate::*;

// copied from pancam and modified
pub fn pan_camera(
    windows: Res<Windows>,
    mut query: Query<(&PanCam, &mut Transform, &OrthographicProjection)>,
    // mut last_pos: Local<Option<Vec2>>,
    keys: Res<Input<KeyCode>>
) {
    let Some(window) = windows.get_primary() else {
        error!("no window you dingus");
        return;
    };
    let window_size = Vec2::new(window.width(), window.height());

    // // Use position instead of MouseMotion, otherwise we don't get acceleration movement
    // let current_pos = match window.cursor_position() {
    //     Some(current_pos) => current_pos,
    //     None => return,
    // };
    // let delta_device_pixels = current_pos - last_pos.unwrap_or(current_pos);

    for (cam, mut transform, projection) in &mut query {
        let proj_size =
            Vec2::new(projection.right - projection.left, projection.top - projection.bottom) *
            projection.scale;

        // The proposed new camera position
        let mut proposed_cam_transform = if
            cam.enabled &&
            keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
        {
            let world_units_per_device_pixel = proj_size / window_size;
            let mut delta_world = Vec2::ZERO;

            let n = 12.0;
            if keys.pressed(KeyCode::W) {
                delta_world.y -= n;
            }
            if keys.pressed(KeyCode::A) {
                delta_world.x += n;
            }
            if keys.pressed(KeyCode::S) {
                delta_world.y += n;
            }
            if keys.pressed(KeyCode::D) {
                delta_world.x -= n;
            }
            transform.translation - (delta_world * world_units_per_device_pixel).extend(0.0)
        } else {
            continue;
        };

        // Check whether the proposed camera movement would be within the provided boundaries, override it if we
        // need to do so to stay within bounds.
        if let Some(min_x_boundary) = cam.min_x {
            let min_safe_cam_x = min_x_boundary + proj_size.x / 2.0;
            proposed_cam_transform.x = proposed_cam_transform.x.max(min_safe_cam_x);
        }
        if let Some(max_x_boundary) = cam.max_x {
            let max_safe_cam_x = max_x_boundary - proj_size.x / 2.0;
            proposed_cam_transform.x = proposed_cam_transform.x.min(max_safe_cam_x);
        }
        if let Some(min_y_boundary) = cam.min_y {
            let min_safe_cam_y = min_y_boundary + proj_size.y / 2.0;
            proposed_cam_transform.y = proposed_cam_transform.y.max(min_safe_cam_y);
        }
        if let Some(max_y_boundary) = cam.max_y {
            let max_safe_cam_y = max_y_boundary - proj_size.y / 2.0;
            proposed_cam_transform.y = proposed_cam_transform.y.min(max_safe_cam_y);
        }

        transform.translation = proposed_cam_transform;
    }
    // *last_pos = Some(current_pos);
}