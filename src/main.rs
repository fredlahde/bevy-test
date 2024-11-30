//! Shows how to render a polygonal [`Mesh`], generated from a [`Rectangle`] primitive, in a 2D scene.
//! Adds a texture and colored vertices, giving per-vertex tinting.

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

struct OverlayColor;

impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 42.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                    },
                    text_color: OverlayColor::GREEN,
                    enabled: true,
                },
            },
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Build a default quad mesh
    let mut mesh = Mesh::from(Rectangle::default());
    // Build vertex colors for the quad. One entry per vertex (the corners of the quad)
    let vertex_colors: Vec<[f32; 4]> = vec![
        LinearRgba::RED.to_f32_array(),
        LinearRgba::GREEN.to_f32_array(),
        LinearRgba::BLUE.to_f32_array(),
        LinearRgba::WHITE.to_f32_array(),
    ];
    // Insert the vertex colors as an attribute
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle = meshes.add(mesh);

    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn the quad with vertex colors
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_translation(Vec3::new(-96., 0., 0.)).with_scale(Vec3::splat(128.)),
        Direction::Up,
    ));

    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_translation(Vec3::new(200., 0., 0.)).with_scale(Vec3::splat(128.)),
        Direction::Up,
    ));
}

fn update(time: Res<Time>, mut query: Query<(&mut Direction, &mut Transform), With<Mesh2d>>) {
    for (mut direction, mut transform) in &mut query {
        match *direction {
            Direction::Up => transform.translation.y -= 100. * time.delta_secs(),
            Direction::Down => transform.translation.y += 100. * time.delta_secs(),
        }

        if transform.translation.y < -200. {
            *direction = Direction::Down;
        }
        if transform.translation.y > 200. {
            *direction = Direction::Up;
        }
    }
}
