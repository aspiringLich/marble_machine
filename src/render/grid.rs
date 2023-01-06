use std::f32::consts::PI;

use crate::*;

// these really should be in a resource but im too lazy
#[allow(non_upper_case_globals)]
pub const size: f32 = 128.0;
#[allow(non_upper_case_globals)]
pub const ext: f32 = 3.0;
#[allow(non_upper_case_globals)]
const grid: u32 = 32;

pub fn spawn_background(mut commands: Commands) {
    // let bg_texture: Handle<Image> = assets.load("back.png");
    // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // let z = 4.0;
    // mesh.insert_attribute(
    //     Mesh::ATTRIBUTE_POSITION,
    //     vec![
    //         [size, size, z],
    //         [size, -size, z],
    //         [-size, -size, z],
    //         [-size, size, z],
    //         [size * ext, size * ext, z],
    //         [size * ext, -size * ext, z],
    //         [-size * ext, -size * ext, z],
    //         [-size * ext, size * ext, z],
    //     ],
    // );
    // mesh.set_indices(Some(Indices::U32(vec![
    //     5, 4, 0, 0, 1, 5, 5, 1, 6, 6, 1, 2, 2, 3, 6, 6, 3, 7, 7, 3, 0, 7, 0, 4,
    // ])));

    let mut grid_builder = PathBuilder::new();
    let grid_size = (size * 2.0) / grid as f32;
    let base = Vec2::new(-size, -size);
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
    border_builder.move_to([-size, -size + grid_size].into());
    border_builder.line_to([-size, size - grid_size].into());
    border_builder.arc(
        [-size + grid_size, size - grid_size].into(),
        Vec2::ONE * grid_size,
        -PI / 2.0,
        1.0,
    );
    border_builder.line_to([size - grid_size, size].into());
    border_builder.arc(
        [size - grid_size, size - grid_size].into(),
        Vec2::ONE * grid_size,
        -PI / 2.0,
        1.0,
    );
    border_builder.move_to([size, size - grid_size].into());
    border_builder.line_to([size, -size + grid_size].into());
    border_builder.move_to([size - grid_size, -size].into());
    border_builder.arc(
        [size - grid_size, -size + grid_size].into(),
        Vec2::ONE * grid_size,
        PI / 2.0,
        1.0,
    );
    border_builder.line_to([size, -size + grid_size].into());
    border_builder.move_to([size - grid_size, -size].into());
    border_builder.line_to([-size + grid_size, -size].into());
    border_builder.arc(
        [-size + grid_size, -size + grid_size].into(),
        Vec2::ONE * grid_size,
        -PI / 2.0,
        1.0,
    );

    let options = StrokeOptions::default()
        .with_line_cap(LineCap::Round)
        .with_line_width(3.0)
        .with_line_join(LineJoin::Round)
        .with_tolerance(0.01);
    let stroke_mode = StrokeMode {
        color: Color::hsl(216.0, 0.25, 0.36),
        options,
    };

    commands.spawn(GeometryBuilder::build_as(
        &border_builder.build(),
        DrawMode::Stroke(stroke_mode),
        Transform::from_xyz(0.0, 0.0, ZOrder::Border.f32()),
    ));
}
