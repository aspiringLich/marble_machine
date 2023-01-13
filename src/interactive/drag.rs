use std::f32::consts::TAU;

use crate::{query::QueryQuerySimple, *};

use super::{interact::InteractiveRotation, select::CursorCoords};

/// drag the selected module(s) around
#[allow(clippy::too_many_arguments)]
pub fn drag_selected(
    mouse_pos: Res<CursorCoords>,
    mouse_buttons: Res<Input<MouseButton>>,
    selected: Res<SelectedModules>,
    keyboard: Res<Input<KeyCode>>,
    q_children: Query<&Children>,
    mut q_transform: Query<&mut Transform>,
    mut active: Local<bool>,
    mut starting_pos: Local<Vec2>,
    mut q_interactive_rot: Query<&mut InteractiveRotation>,
    // returns an option to pipe into
) -> Option<Entity> {
    let snapping = if keyboard.pressed(KeyCode::LShift) {
        8.0
    } else {
        1.0
    };

    // basically: if active is not true it needs these specific conditions to become true, or else the system will not run
    if !*active {
        if selected.is_changed() && mouse_buttons.pressed(MouseButton::Left) {
            *active = true;
            *starting_pos = **mouse_pos;
        } else {
            *active = false;
            return None;
        }
    }

    // if we let go of the left mouse button, return
    if !mouse_buttons.pressed(MouseButton::Left) {
        *active = false;
        return None;
    }

    let Some(selected) = selected.selected else {*active = false; return None};

    if let Ok(mut i_rot) = q_interactive_rot.get_mut(selected) {
        if keyboard.just_pressed(KeyCode::Q) {
            i_rot.rot += TAU / 8.0;
        } else if keyboard.just_pressed(KeyCode::E) {
            i_rot.rot -= TAU / 8.0;
        }
    }

    let pos = &mut q_transform.entity_mut(selected).translation;
    let Vec2 { x, y } = **mouse_pos - *starting_pos;

    // rounding x and y to the nearest snapping #
    let round = Vec2::new(
        (x / snapping).round() * snapping,
        (y / snapping).round() * snapping,
    );
    // let round = Vec2::new(x, y);
    let mut changed = false;
    if round.x != 0.0 {
        starting_pos.x += round.x;
        pos.x += round.x;
        changed = true;
    }
    if round.y != 0.0 {
        starting_pos.y += round.y;
        pos.y += round.y;
        changed = true;
    }

    if changed {
        return Some(selected);
    } else {
        return None;
    }
}
