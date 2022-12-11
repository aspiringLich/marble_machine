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
    mut windows: ResMut<Windows>,
    mut selected: ResMut<SelectedModules>,
    buttons: Res<Input<MouseButton>>,
    hovered: Res<HoveredEntities>,
    q_parent: Query<&Parent>,
    has_body: Query<With<marker::ModuleBody>>,
    has_interactive: Query<With<interact::Interactive>>,
    mut interactive_selected: ResMut<interact::InteractiveSelected>,
) {
    // get that window
    let Some(window) = windows.get_primary_mut() else { error!("no window you dingus"); return; };

    // the entity, if applicable, that we may want to apply glow to to show were hovering over it
    // let mut glow: Entity;
    // prioritize interactive elements
    if let Some(&e) = hovered.iter().find(|e| has_interactive.has(**e)) {
        // glow = e;
        // if clicky click, set interactive_selected
        if buttons.just_pressed(MouseButton::Left) {
            **interactive_selected = Some(e);
        }
    }
    // then check if weve selected a body
    else if let Some(&e) = hovered.iter().find(|e| has_body.has(**e)) {
        // glow = e;
        // if clicky click, set selected modules
        if buttons.just_pressed(MouseButton::Left) {
            *selected = SelectedModules::from_entity(q_parent.entity(e).get());
        }
    } else {
        // if clicky click, unselect stuff
        if buttons.just_pressed(MouseButton::Left) {
            selected.clear_selected();
        } else if !buttons.pressed(MouseButton::Left) && let Some(_) = **interactive_selected {
            **interactive_selected = None;
            window.set_cursor_icon(CursorIcon::Default);
            return;
        }

        // wait but stuff might be selected!
        if let Some(_e) = **interactive_selected {
            // glow = e;
        } else if let Some(_e) = selected.selected && buttons.pressed(MouseButton::Left) {
            // glow = e;
        } else {
            // dont need to bother with this stuff
            // leave
            window.set_cursor_icon(CursorIcon::Default);
            return;
        }
    }

    if buttons.pressed(MouseButton::Left) {
        window.set_cursor_icon(CursorIcon::Grabbing);
    } else {
        window.set_cursor_icon(CursorIcon::Grab);
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
    let Vec2 { x, y } = **mouse_pos - *starting_pos;

    // rounding x and y to the nearest snapping #
    let round = Vec2::new(
        (x / snapping).round() * snapping,
        (y / snapping).round() * snapping,
    );
    if round.x != 0.0 {
        starting_pos.x += round.x;
        pos.x += round.x;
    }
    if round.y != 0.0 {
        starting_pos.y += round.y;
        pos.y += round.y;
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

#[derive(Resource, Debug, Deref)]
pub struct CursorCoords(Vec2);

impl Default for CursorCoords {
    fn default() -> Self {
        CursorCoords(Vec2::ZERO)
    }
}
/// get the position of a cursor in absolute world coordinates
pub fn get_cursor_pos(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<marker::Camera>>,
    mut coords: ResMut<CursorCoords>,
) {
    let (camera, camera_transform) = q_camera.single();

    let Some(window) = windows.get_primary() else { error!("no window you dingus"); return; };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        coords.0 = world_pos;
    }
}

#[derive(Default, Deref, DerefMut, Resource)]
pub struct HoveredEntities(Vec<Entity>);

pub fn get_hovered_entities(
    mouse_pos: Res<CursorCoords>,
    rapier_context: Res<RapierContext>,
    mut hovered_entities: ResMut<HoveredEntities>,
) {
    hovered_entities.clear();
    rapier_context.intersections_with_point(**mouse_pos, default(), |entity| {
        hovered_entities.push(entity);
        true
    });
}
