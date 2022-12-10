use crate::{
    module::param::{QueryQueryIter, QueryQuerySimple},
    *,
};
use bevy::render::camera::RenderTarget;
use iyes_loopless::prelude::{ConditionHelpers, IntoConditionalSystem};

/// if:
///     SelectedModules is in place mode,
///     SelectedModules has something selected
fn place(res: Res<SelectedModules>) -> bool {
    res.place && res.selected.is_some()
}

fn egui_wants_cursor(mut ctx: ResMut<bevy_egui::EguiContext>) -> bool {
    ctx.ctx_mut().wants_pointer_input()
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(get_selected.run_if_not(place).run_if_not(egui_wants_cursor))
        .with_system(drag_selected.run_if_not(place))
        .with_system(place_selected.run_if(place).run_if_not(egui_wants_cursor))
        .after(ui::inspector_ui)
}
/// update SelectedModule whenever the left cursor is clicked
pub fn get_selected(
    mut selected: ResMut<SelectedModules>,
    rapier_context: Res<RapierContext>,
    buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<CursorCoords>,
    q_parent: Query<&Parent>,
    has_body: Query<With<marker::ModuleBody>>,
    has_interactive: Query<With<interact::Interactive>>,
    mut interactive_selected: ResMut<interact::InteractiveSelected>,
) {
    // if clicky click
    if buttons.just_pressed(MouseButton::Left) {
        let mut found = false;
        rapier_context.intersections_with_point(**mouse_pos, default(), |entity| {
            if has_body.has(entity) {
                // get the parent of the main body and set that as the selected module
                // main body is assumed to be the child of the overall module parent entity
                *selected = SelectedModules::from_entity(q_parent.entity(entity).get());
                // eprintln!("selected {}", q_name.get(entity).unwrap());
                found = true;
                // stop we found it
                false
            }
            // if we hit a interactive thingy thats the thing we select instead, ignore the body thing
            else if has_interactive.has(entity) {
                **interactive_selected = Some(entity);
                found = true;
                false
            } else {
                // havent found it yet
                true
            }
        });
        if !found {
            selected.clear_selected()
        }
    }
}

/// drag the selected module(s) around
pub fn drag_selected(
    mouse_pos: Res<CursorCoords>,
    mouse_buttons: Res<Input<MouseButton>>,
    selected: Res<SelectedModules>,
    mut q_transform: Query<&mut Transform>,
    mut active: Local<bool>,
    mut starting_pos: Local<Vec2>,
) {
    let snapping = 8.0;

    // basically: if active is not true it needs these specific conditions to become true, or else the system will not run
    if !*active {
        if selected.is_changed() && mouse_buttons.pressed(MouseButton::Left) {
            *active = true;
            *starting_pos = **mouse_pos;
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
    let pos = &mut q_transform.entity_mut(selected).translation;
    let Vec2 { x, y } = **mouse_pos;

    // rounding x and y to the nearest snapping #
    let (rx, ry) = (
        (x / snapping).round() * snapping,
        (y / snapping).round() * snapping,
    );
    if rx != x || ry != y {
        pos.x = rx;
        pos.y = ry;
    }
}

/// runs if SelectedModules's place flag is true
/// place the selected module somewhere
fn place_selected(
    mouse_pos: Res<CursorCoords>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut selected: ResMut<SelectedModules>,
    mut q_transform: Query<&mut Transform>,
) {
    let snapping = 8.0;

    // if we click then place the module
    if mouse_buttons.pressed(MouseButton::Left) {
        // dont be confused, set selected.place to false so that it now the place_selected fn no longer runs
        selected.place = false
    }
    // else the module follows the mouse
    else {
        let Some(selected) = selected.selected else { unreachable!() };
        let pos = &mut q_transform.entity_mut(selected).translation;
        let Vec2 { x, y } = **mouse_pos;

        // rounding x and y to the nearest snapping #
        let (rx, ry) = (
            (x / snapping).round() * snapping,
            (y / snapping).round() * snapping,
        );
        if rx != x || ry != y {
            pos.x = rx;
            pos.y = ry;
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
