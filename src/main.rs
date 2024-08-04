use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800.0, 800.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_board))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Tile;

const TILE_SIZE: f32 = 100.0;

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));

    // Distribute colors evenly across the rainbow.
    let dark_material = materials.add(Color::linear_rgb(
        187.0 / 255.0,
        190.0 / 255.0,
        100.0 / 255.0,
    ));

    let light_material = materials.add(Color::linear_rgb(
        234.0 / 255.0,
        240.0 / 255.0,
        206.0 / 255.0,
    ));

    for i in 0..8 {
        for j in 0..8 {
            let material = match (i + j) % 2 == 0 {
                true => dark_material.clone(),
                false => light_material.clone(),
            };

            let x = TILE_SIZE * (i - 4) as f32 + TILE_SIZE / 2.0;
            let y = TILE_SIZE * (j - 4) as f32 + TILE_SIZE / 2.0;

            let transform = Transform::from_xyz(x, y, 0.0);

            commands.spawn((
                Tile,
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(mesh.clone()),
                    material,
                    transform,
                    ..default()
                },
            ));
        }
    }
}
