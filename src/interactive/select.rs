use std::f32::consts::TAU;

use crate::{misc::RapierContextMethods, query::QueryQuerySimple, *, modules::BodyType};

use super::{hover::HoveredEntities, intersect::{RequestedMove, MoveType}};

/// update SelectedModule whenever the left cursor is clicked
#[allow(clippy::too_many_arguments)]
pub fn get_selected(
    mut windows: ResMut<Windows>,
    mut selected: ResMut<SelectedModules>,
    buttons: Res<Input<MouseButton>>,
    hovered: Res<HoveredEntities>,
    q_parent: Query<&Parent>,
    has_body: Query<With<BodyType>>,
    has_interactive: Query<With<interact::Interactive>>,
    mut interactive_selected: ResMut<interact::InteractiveSelected>,
) {
    // get that window
    let Some(window) = windows.get_primary_mut() else {
        error!("no window you dingus");
        return;
    };

    // the entity, if applicable, that we may want to apply glow to to show were hovering over it
    let glow: Entity;
    // prioritize interactive elements
    if let Some(&e) = hovered.iter().find(|e| has_interactive.has(**e)) {
        glow = e;
        // if clicky click, set interactive_selected
        if buttons.just_pressed(MouseButton::Left) {
            **interactive_selected = Some(e);
        }
    } else if
    // then check if weve selected a body
    let Some(&e) = hovered.iter().find(|e| has_body.has(**e)) {
        glow = e;
        // if clicky click, set selected modules
        if buttons.just_pressed(MouseButton::Left) {
            *selected = SelectedModules::from_entity(q_parent.entity(e).get());
        } else if !buttons.pressed(MouseButton::Left) {
            **interactive_selected = None;
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
        if let Some(e) = **interactive_selected {
            glow = e;
        } else if let Some(e) = selected.selected && buttons.pressed(MouseButton::Left) {
            glow = e;
        } else {
            // dont need to bother with this stuff
            // leave
            window.set_cursor_icon(CursorIcon::Default);
            return;
        }
    }

    if has_interactive.has(glow) {
        window.set_cursor_icon(CursorIcon::Hand);
    } else if buttons.pressed(MouseButton::Left) {
        window.set_cursor_icon(CursorIcon::Grabbing);
    } else {
        window.set_cursor_icon(CursorIcon::Grab);
    }
}

/// runs if SelectedModules's place flag is true
/// place the selected module somewhere
#[allow(clippy::too_many_arguments)]
pub fn place_selected(
    rapier_ctx: Res<RapierContext>,
    q_collider: Query<(Entity, &Collider), Without<Sensor>>,
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mouse_pos: Res<CursorCoords>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut selected: ResMut<SelectedModules>,
    mut q_transform: Query<&mut Transform>,
    q_global_transform: Query<&GlobalTransform>,
    q_children: Query<&Children>,
    has_io: Query<Or<(With<marker::Input>, With<marker::Output>)>>,
    grid_info: Res<grid::GridInfo>,
    mut q_visibility: Query<&mut Visibility>, 
    has_rigidbody: Query<With<RigidBody>>,
    mut requested_move: EventWriter<RequestedMove>,
) {
    let snapping = if keyboard.pressed(KeyCode::LShift) {
        8.0
    } else {
        1.0
    };

    // if we click then place the module
    if mouse_buttons.just_pressed(MouseButton::Left)
        && f32::max(mouse_pos.x.abs(), mouse_pos.y.abs()) < grid_info.half_size
    {
        // check if any of the colliders are colliding with a rigidbody, ignoring the colliders of the module itself
        let s_entity = selected.selected.unwrap();
        let colliders = q_children
            .iter_descendants(s_entity)
            .filter_map(|e| q_collider.get(e).ok())
            .collect::<Vec<_>>();
        let ignore = colliders.iter().map(|(e, _)| *e).collect::<Vec<_>>();
        let predicate = |e| !ignore.contains(&e) && has_rigidbody.get(e).is_ok();
        let filter = QueryFilter::only_fixed()
            .exclude_sensors()
            .predicate(&predicate);

        // if were clear
        if !colliders.iter().any(|(e, c)| {
            rapier_ctx
                .intersection_with_shape_transform(
                    q_global_transform.entity(*e).compute_transform(),
                    c,
                    filter,
                )
                .is_some()
        }) {
            // dont be confused, set selected.place to false so that it now the place_selected fn no longer runs
            selected.place = false;
            return;
        }
    }
    // else the module follows the mouse
    let Some(sel_entity) = selected.selected else {
        unreachable!()
    };
    
    // set it to visibile cuz reasons
    *q_visibility.get_mut(sel_entity).expect("sel_entity is a sprite") = Visibility::VISIBLE;

    // if escape is pressed, then clear and return
    if keyboard.pressed(KeyCode::Escape) {
        commands.entity(sel_entity).despawn_recursive();
        selected.clear_selected();
        selected.place = false;
        return;
    }

    let io = q_children
        .entity(sel_entity)
        .iter()
        .filter(|e| has_io.has(**e));
    if keyboard.just_pressed(KeyCode::Q) {
        for &e in io {
            let mut tf = q_transform.entity_mut(e);
            tf.rotate_z(TAU / 8.0);
        }
    } else if keyboard.just_pressed(KeyCode::E) {
        for &e in io {
            let mut tf = q_transform.entity_mut(e);
            tf.rotate_z(-TAU / 8.0);
        }
    }

    let Vec2 { x, y } = **mouse_pos - 0.5;

    // rounding x and y to the nearest snapping #
    let round = Vec2::new(
        (x / snapping).round() * snapping + 0.5,
        (y / snapping).round() * snapping + 0.5,
    );
    requested_move.send(
        RequestedMove::new(selected.selected.unwrap(), MoveType::TranslateTo(round.extend(0.0))).snapping(),
    )
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
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return };

    let Some(window) = windows.get_primary() else {
        error!("no window you dingus");
        return;
    };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        coords.0 = world_pos;
    }
}
