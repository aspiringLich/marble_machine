use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

fn text_init(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("Inconsolata-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("Inconsolata-Regular.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

/// A unit struct to help identify the FPS UI component, since there may be many Text components
/// also used as the plugin
#[derive(Component)]
pub struct FpsText;

impl Plugin for FpsText {
    fn build(&self, app: &mut App) {
        app.add_startup_system(text_init)
            .add_system(text_update_system);
    }
}
