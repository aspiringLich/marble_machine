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

fn egui(mut ctx: ResMut<bevy_egui::EguiContext>) -> bool {
    ctx.ctx_mut().wants_pointer_input()
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(
            select::get_selected
                .run_if_not(place)
                .run_if_not(egui)
                .label("drag"),
        )
        .with_system(select::drag_selected.run_if_not(place))
        .with_system(select::place_selected.run_if(place).run_if_not(egui))
        .with_system(interact::spawn_despawn_interactive_components)
        .with_system(interact::use_widgets.after(select::get_selected))
        .with_system(interact::do_interactive_rotation.after(interact::use_widgets))
        .with_system(tracer::tracer.run_if(select).after("drag"))
        .after(ui::inspector_ui)
}
