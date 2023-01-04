use crate::*;

const GRID: i32 = 30;
const GRID_SIZE: f32 = 8.0;

#[derive(Resource)]
pub struct Grid {
    pub size: usize,
    pub active: bool,
}

#[derive(Component)]
pub struct GridMarker;

pub fn spawn_background(
    mut commands: Commands,
    grid: Res<Grid>,
    q_grid_marker: Query<Entity, With<GridMarker>>,
) {
    if !grid.is_changed() || !grid.active {
        return;
    }

    for entity in q_grid_marker.iter() {
        commands.entity(entity).despawn();
    }

    let mut path_builder = PathBuilder::new();
    let g = grid.size as f32 / 2.0 * GRID_SIZE;

    let base_1 = Vec2::new(-g, -g);
    let base_2 = Vec2::new(-g, g);
    for x in 0..=grid.size {
        let offset = Vec2::X * GRID_SIZE * x as f32;
        path_builder.move_to(base_1 + offset);
        path_builder.line_to(base_2 + offset);
    }

    let base_1 = Vec2::new(-g, -g);
    let base_2 = Vec2::new(g, -g);
    for y in 0..=grid.size {
        let offset = Vec2::Y * GRID_SIZE * y as f32;
        path_builder.move_to(base_1 + offset);
        path_builder.line_to(base_2 + offset);
    }

    // TODO: make this an image or something? idk
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(0, 192, 192),
            ..default()
        },
        transform: Transform::from_scale(
            [
                grid.size as f32 * GRID_SIZE,
                grid.size as f32 * GRID_SIZE,
                1.0,
            ]
            .into(),
        ),
        ..default()
    });

    commands.spawn((
        GeometryBuilder::build_as(
            &path_builder.build(),
            DrawMode::Stroke(StrokeMode::new(Color::rgba_u8(0, 127, 127, 20), 2.0)),
            Transform::from_translation(Vec2::ZERO.extend(0.001)),
        ),
        Name::new("grid.geo"),
        GridMarker,
    ));
}
