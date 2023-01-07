use std::f32::consts;

use crate::{
    module::{param::ModuleResources, Module},
    spawn::SpawnModule,
    *,
};
use bevy::prelude::Image;
use bevy_egui::*;

pub fn ui(mut egui_context: ResMut<EguiContext>) {
    egui::SidePanel::left("spawning")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.set_style(egui::Style {
                override_text_style: None,
                override_font_id: None,
                text_styles: default(),
                wrap: None,
                spacing: default(),
                interaction: default(),
                visuals: egui::Visuals {
                    panel_fill: egui::Color32::from_black_alpha(127),
                    dark_mode: true,
                    ..default()
                },
                animation_time: 0.0,
                debug: default(),
                explanation_tooltips: false,
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}
