use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::*;

// these really should be in a resource but im too lazy
pub const size: f32 = 128.0;
pub const ext: f32 = 3.0;
const grid: u32 = 32;

pub fn spawn_background(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    let mut path_builder = PathBuilder::new();
    let grid_size = (size * 2.0) / grid as f32;
    let base = Vec2::new(-size, -size);
    for x in 1..grid {
        let moved = base + Vec2::X * grid_size * x as f32;
        path_builder.move_to(moved);
        path_builder.line_to(moved + Vec2::Y * size * 2.0);
    }
    for y in 1..grid {
        let moved = base + Vec2::Y * grid_size * y as f32;
        path_builder.move_to(moved);
        path_builder.line_to(moved + Vec2::X * size * 2.0);
    }

    // let geometry = GeometryBuilder::new().add(path_builder.build()).build();
    let line = path_builder.build();
    commands.spawn(GeometryBuilder::build_as(
        &line,
        DrawMode::Stroke(StrokeMode::new(Color::rgba_u32(0x00000010), 1.0)),
        Transform::default(),
    ));

    // // spawn the mesh around the background to cover up things that go past it
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: Mesh2dHandle(meshes.add(mesh)),
    //         material: materials.add(Color::hex("282c3c").unwrap().into()),
    //         transform: Transform::from_xyz(0.0, 0.0, 8.0),
    //         ..default()
    //     },
    //     Name::new("back.mesh"),
    // ));
}
