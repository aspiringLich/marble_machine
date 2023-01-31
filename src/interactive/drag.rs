use std::f32::consts::TAU;

use crate::{query::QueryQuerySimple, *};

use super::{interact::InteractiveRotation, intersect::RequestedMove, select::CursorCoords};

/// drag the selected module(s) around
#[allow(clippy::too_many_arguments)]
pub fn drag_selected(
    mut requested_move: ResMut<RequestedMove>,
    mouse_pos: Res<CursorCoords>,
    mouse_buttons: Res<Input<MouseButton>>,
    selected: Res<SelectedModules>,
    keyboard: Res<Input<KeyCode>>,
    q_children: Query<&Children>,
    q_transform: Query<&Transform>,
    mut active: Local<bool>,
    mut starting_pos: Local<Vec2>,
    mut q_interactive_rot: Query<&mut InteractiveRotation>,
    mut prev: Local<Vec2>,
    // returns an option to pipe into
) {
    let snapping = if keyboard.pressed(KeyCode::LShift) {
        8.0
    } else {
        1.0
    };

    // basically: if active is not true it needs these specific conditions to become true, or else the system will not run
    if !*active {
        if selected.is_changed() && mouse_buttons.pressed(MouseButton::Left) && let Some(selected) = selected.selected{
            *active = true;
            *starting_pos = **mouse_pos - q_transform.entity(selected).translation.truncate();
        } else {
            *active = false;
            return;
        }
    }

    // if we let go of the left mouse button, return
    if !mouse_buttons.pressed(MouseButton::Left) {
        *active = false;
        return;
    }

    let Some(selected) = selected.selected else {*active = false; return};

    if let Ok(mut i_rot) = q_interactive_rot.get_mut(selected) {
        if keyboard.just_pressed(KeyCode::Q) {
            i_rot.rot += TAU / 8.0;
        } else if keyboard.just_pressed(KeyCode::E) {
            i_rot.rot -= TAU / 8.0;
        }
    }

    let mut transform = *q_transform.entity(selected);
    let Vec2 { x, y } = **mouse_pos - *starting_pos;

    // rounding x and y to the nearest snapping #
    let round = Vec2::new(
        (x / snapping).round() * snapping,
        (y / snapping).round() * snapping,
    );
    // let round = Vec2::new(x, y);

    if round != *prev {
        transform.translation.x = round.x;
        transform.translation.y = round.y;

        requested_move.requesting = selected;
        requested_move.to = transform;

        *prev = round;
    }
}
