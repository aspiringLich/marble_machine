use std::collections::VecDeque;

use crate::*;
use iyes_loopless::prelude::*;

pub mod interact;
pub mod select;
pub mod tracer;

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
fn egui(mut ctx: ResMut<bevy_egui::EguiContext>, mut before: Local<BoolBuf>) -> bool {
    let out = ctx.ctx_mut().wants_pointer_input();
    before.pop_front();
    before.push_back(out);
    before.iter().any(|b| *b)
}

fn init_res(mut commands: Commands) {
    commands.init_resource::<tracer::TracerEntities>();
}

pub fn app(app: &mut App) {
    app.add_startup_system_to_stage(Label::StartupStageStart, init_res)
        .add_system(select::get_hovered_entities.after("spawn::spawn_modules"))
        .add_system(
            select::get_selected
                .run_if_not(place)
                .run_if_not(egui)
                .label("select::get_selected"),
        )
        .add_system(select::drag_selected.run_if_not(place).run_if_not(egui))
        .add_system(select::place_selected.run_if(place).run_if_not(egui))
        .add_system(interact::spawn_despawn_interactive_components.before("interact::use_widgets"))
        .add_system(
            interact::use_widgets
                .after("select::get_selected")
                .label("interact::use_widgets"),
        )
        .add_system(interact::do_interactive_rotation.after(interact::use_widgets))
        .add_system(
            tracer::tracer
                .run_if(select)
                .run_if(|r: Res<SelectedModules>| !r.place)
                .after("select::get_selected"),
        )
        .init_resource::<interact::InteractiveSelected>();
}
