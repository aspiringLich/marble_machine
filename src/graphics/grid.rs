use std::f32::consts::PI;

use crate::*;

use super::atlas::{basic, AtlasDictionary};

#[derive(Resource)]
pub struct GridInfo {
    pub half_size: f32,
    pub ext: f32,
    pub grid_size: f32,
}

impl GridInfo {
    pub fn in_bounds(&self, mut pos: Vec2) -> bool {
        pos = pos.abs();
        pos.x <= self.half_size && pos.y <= self.half_size
    }
}

impl Default for GridInfo {
    fn default() -> Self {
        Self {
            half_size: 96.0,
            ext: 3.0,
            grid_size: 8.0,
        }
    }
}

pub fn spawn_background(mut commands: Commands, grid_info: Res<GridInfo>) {
    if !grid_info.is_changed() {
        return;
    }

    let GridInfo {
        half_size: size,
        ext: _,
        grid_size,
    } = *grid_info;

    // the grid of the background
    let mut grid_builder = PathBuilder::new();
    let base = Vec2::new(-size, -size);
    let grid = (size / grid_size) as u32 * 2;
    // dbg!(grid);
    for x in 1..grid {
        let moved = base + Vec2::X * grid_size * x as f32;
        grid_builder.move_to(moved);
        grid_builder.line_to(moved + Vec2::Y * size * 2.0);
    }
    for y in 1..grid {
        let moved = base + Vec2::Y * grid_size * y as f32;
        grid_builder.move_to(moved);
        grid_builder.line_to(moved + Vec2::X * size * 2.0);
    }

    // let geometry = GeometryBuilder::new().add(path_builder.build()).build();
    let grid_shape = grid_builder.build();
    commands.spawn(GeometryBuilder::build_as(
        &grid_shape,
        DrawMode::Stroke(StrokeMode::new(Color::rgba_u32(0xffffff07), 1.0)),
        Transform::from_xyz(0.0, 0.0, ZOrder::Background.f32()),
    ));

    let mut border_builder = PathBuilder::new();

    let color = Color::hsl(216.0, 0.25, 0.36);
    let options = StrokeOptions::default()
        .with_line_cap(LineCap::Square)
        .with_line_width(2.0)
        .with_tolerance(0.01);
    let stroke_mode = StrokeMode { color, options };

    let curve_segments = 10;
    let mut collider_vertices = vec![];

    // build the border
    let mut flag = false;
    let mut rotation = 0.0;
    let mut iter = [(1., 1.), (1., -1.), (-1., -1.), (-1., 1.), (1., 1.)]
        .windows(2)
        .enumerate();

    let extend = 0.1;

    while let Some((i, [(a, b), (next_a, next_b)])) = iter.next() {
        // get point and next point
        let n = size - 0.5;
        let mut point = Vec2::new(n * a, n * b);
        let mut next_point = Vec2::new(n * next_a, n * next_b);
        if flag {
            point.x -= grid_size * a;
            next_point.x -= grid_size * next_a;
        } else {
            point.y -= grid_size * b;
            next_point.y -= grid_size * next_b;
        }

        // extend the collider
        collider_vertices.extend(&[point, next_point]);
        // build the line
        border_builder.move_to(point + Vec2::new(1.0 * a, 1.0 * b));
        border_builder.line_to(
            next_point
                + Vec2::new(
                    [1.0, 2.0][flag as usize] * next_a,
                    [1.0, 2.0][!flag as usize] * next_b,
                ),
        );

        // the "curve" at the corners of the collider
        let radius = grid_size - 0.5 + extend;
        let center = Vec2::new((size - grid_size) * next_a, (size - grid_size) * next_b);
        let mut angle = rotation;
        for _ in 1..curve_segments {
            angle += -PI / 2.0 / curve_segments as f32;
            // dbg!(angle);
            collider_vertices.push(center + Vec2::new(f32::cos(angle), f32::sin(angle)) * radius);
        }
        flag = !flag;
        rotation -= PI / 2.0;

        // spawn the corner
        let n = size - grid_size / 2. + 1.5;
        let translation = Vec2::new(n * next_a, n * next_b);
        let (texture_atlas, index) = basic::corner.info();
        commands
            .spawn((SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index,
                    color,
                    flip_x: i == 2 || i == 1,
                    flip_y: i & 2 != 0,
                    ..default()
                },
                texture_atlas,
                transform: Transform::from_translation(translation.extend(ZOrder::Border.f32())),
                ..default()
            },))
            .name("corner.sprite");
    }
    let mut indices = vec![];
    for i in 1..collider_vertices.len() as u32 {
        indices.push([i - 1, i]);
    }
    indices.push([collider_vertices.len() as u32 - 1, 0]);

    commands
        .spawn((
            GeometryBuilder::build_as(
                &border_builder.build(),
                DrawMode::Stroke(stroke_mode),
                Transform::from_xyz(0.0, 0.0, ZOrder::Border.f32()),
            ),
            Collider::polyline(collider_vertices, Some(indices)),
            RigidBody::Fixed,
        ))
        .name("back.line");

    commands
        .spawn((
            Collider::cuboid(size, 10.0),
            TransformBundle::from_transform(Transform::from_xyz(0.0, -size - 10.0 + 0.5, 0.0)),
            RigidBody::Fixed,
        ))
        .name("bottom.collider");
}
