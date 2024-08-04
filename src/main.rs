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
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_board, setup_pieces))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(400.0, 400.0, 0.0),
        ..Default::default()
    });
}

#[derive(Component)]
struct Tile;

const TILE_SIZE: f32 = 100.0;
const HALF_TILE: f32 = 50.0;

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

            let x = TILE_SIZE * i as f32 + HALF_TILE;
            let y = TILE_SIZE * j as f32 + HALF_TILE;

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

fn setup_pieces(mut commands: Commands, server: Res<AssetServer>) {
    let texture = server.load("sprites/pieces.png");

    let mut spawn_piece = |piece: Rect, position: Vec2| {
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                position.x * TILE_SIZE + HALF_TILE,
                position.y * TILE_SIZE + HALF_TILE,
                1.,
            ),
            texture: texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                rect: Some(piece),
                ..default()
            },
            ..default()
        });
    };

    // Piece sprite positions in texture
    const SPRITE_SIZE: f32 = 480.0;

    let pieces: [Rect; 12] = (0..12)
        .map(|i| {
            let x = i as f32 % 6.0;
            let y = (i / 6) as f32;

            Rect::new(
                x * SPRITE_SIZE,
                y * SPRITE_SIZE,
                (x + 1.0) * SPRITE_SIZE,
                (y + 1.0) * SPRITE_SIZE,
            )
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    #[rustfmt::skip]
    let [
        w_king, w_queen, w_bishop, w_knight, w_rook, w_pawn, 
        b_king, b_queen, b_bishop, b_knight, b_rook, b_pawn
    ] = pieces;

    // Spawn kings
    spawn_piece(w_king, Vec2::new(4.0, 0.0));
    spawn_piece(b_king, Vec2::new(4.0, 7.0));

    // Spawn queens
    spawn_piece(w_queen, Vec2::new(3.0, 0.0));
    spawn_piece(b_queen, Vec2::new(3.0, 7.0));

    // Spawn bishops
    spawn_piece(w_bishop, Vec2::new(2.0, 0.0));
    spawn_piece(w_bishop, Vec2::new(5.0, 0.0));
    spawn_piece(b_bishop, Vec2::new(2.0, 7.0));
    spawn_piece(b_bishop, Vec2::new(5.0, 7.0));

    // Spawn knights
    spawn_piece(w_knight, Vec2::new(1.0, 0.0));
    spawn_piece(w_knight, Vec2::new(6.0, 0.0));
    spawn_piece(b_knight, Vec2::new(1.0, 7.0));
    spawn_piece(b_knight, Vec2::new(6.0, 7.0));

    // Spawn rooks
    spawn_piece(w_rook, Vec2::new(0.0, 0.0));
    spawn_piece(w_rook, Vec2::new(7.0, 0.0));
    spawn_piece(b_rook, Vec2::new(0.0, 7.0));
    spawn_piece(b_rook, Vec2::new(7.0, 7.0));

    // Spawn pawns
    for i in 0..8 {
        spawn_piece(w_pawn, Vec2::new(i as f32, 1.0));
        spawn_piece(b_pawn, Vec2::new(i as f32, 6.0));
    }
}
