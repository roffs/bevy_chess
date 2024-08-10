mod board;
mod input;
mod pieces;

use bevy::{prelude::*, window::WindowResolution};
use board::setup_board;
use input::InputPlugin;
use pieces::Piece;

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
        .add_systems(Startup, (setup_camera, setup_board, Piece::spawn_pieces))
        .add_plugins(InputPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(400.0, 400.0, 0.0),
        ..Default::default()
    });
}
