use std::collections::VecDeque;

use crate::*;
use iyes_loopless::prelude::*;

pub mod drag;
pub mod hover;
pub mod interact;
pub mod intersect;
pub mod select;
pub mod tracer;
pub mod pan;

/// if:
///     SelectedModules is in place mode,
///     SelectedModules has something selected
fn place(res: Res<SelectedModules>) -> bool {
    res.place && res.selected.is_some()
}

/// if we've selected something
fn select(res: Res<SelectedModules>) -> bool {
    res.selected.is_some()
}

#[derive(Deref, DerefMut)]
struct BoolBuf(VecDeque<bool>);

impl Default for BoolBuf {
    fn default() -> Self {
        Self(vec![false; 4].into())
    }
}

// this is a hack idk why it doesnt work like i want
fn egui(
    mut ctx: ResMut<bevy_egui::EguiContext>,
    mut before: Local<BoolBuf>,
    mut q_pancam: Query<&mut PanCam>
) -> bool {
    let out = ctx.ctx_mut().wants_pointer_input();
    before.pop_front();
    before.push_back(out);
    let out = before.iter().any(|b| *b);

    q_pancam.single_mut().enabled = !out;
    out
}

fn init_res(mut commands: Commands) {
    commands.init_resource::<tracer::TracerEntities>();
}

pub fn app(app: &mut App) {
    app.add_event::<intersect::RequestedMove>()
        .init_resource::<select::CursorCoords>()
        .init_resource::<hover::HoveredEntities>()
        .init_resource::<interact::InteractiveSelected>()
        .add_startup_system_to_stage(StartupStage::Startup, init_res);

    app.add_system_set_to_stage(
        CoreStage::First,
        SystemSet::new()
            .with_system(select::get_cursor_pos.label("select::get_cursor_pos"))
            .with_system(pan::pan_camera.before("select::get_cursor_pos"))
    ).add_system_set_to_stage(
        CoreStage::Update,
        SystemSet::new()
            .with_system(hover::get_hovered_entities)
            .with_system(
                select::get_selected
                    .run_if_not(place)
                    .run_if_not(egui)
                    .label("select::get_selected")
            )
            .with_system(
                drag::drag_selected
                    .run_if_not(place)
                    .run_if_not(egui)
                    .before("intersect::do_requested_move")
                    .label("select::drag_selected")
            )
            .with_system(
                select::place_selected
                    .run_if(place)
                    .run_if_not(egui)
                    .after("select::drag_selected")
                    .before("intersect::do_requested_move")
            )
            .with_system(
                interact::spawn_despawn_interactive_components.before("interact::use_widgets")
            )
            .with_system(
                interact::use_widgets
                    .after("select::get_selected")
                    .before("intersect::do_requested_move")
                    .label("interact::use_widgets")
            )
            .with_system(
                interact::do_interactive_rotation
                    .after(interact::use_widgets)
                    .before("intersect::do_requested_move")
                    .label("interact::do_interactive_rotation")
            )
            .with_system(intersect::do_requested_move.label("intersect::do_requested_move"))
            .with_system(
                tracer::tracer
                    .run_if(select)
                    .run_if(|r: Res<SelectedModules>| !r.place)
                    .after("select::get_selected")
                    .after("select::drag_selected")
            )
            .with_system(hover::draw_selection_on_hovered)
    );
}