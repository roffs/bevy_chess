mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use bevy::prelude::*;
use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

use crate::board::{HALF_TILE, TILE_SIZE};

const SPRITE_SIZE: f32 = 480.0;

#[derive(Component, Clone)]
enum Color {
    Black,
    White,
}

trait Behavior {
    const BLACK_SPRITE_POSITION: (u8, u8);
    const WHITE_SPRITE_POSITION: (u8, u8);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)];

    fn new() -> Self;
}

#[derive(Component)]
struct Piece<T: Behavior + Send + Sync> {
    position: (u8, u8),
    color: Color,
    behavior: T,
}

impl<T: Behavior + Send + Sync> Piece<T> {
    fn new(position: (u8, u8), color: Color) -> Piece<T> {
        Piece {
            position,
            color,
            behavior: T::new(),
        }
    }
}

pub fn spawn_pieces(mut commands: Commands, server: Res<AssetServer>) {
    let texture = server.load("sprites/pieces.png");

    spawn_piece::<King>(&mut commands, texture.clone());
    spawn_piece::<Queen>(&mut commands, texture.clone());
    spawn_piece::<Bishop>(&mut commands, texture.clone());
    spawn_piece::<Knight>(&mut commands, texture.clone());
    spawn_piece::<Rook>(&mut commands, texture.clone());
    spawn_piece::<Pawn>(&mut commands, texture);
}

fn spawn_piece<T: Behavior + Send + Sync + 'static>(
    commands: &mut Commands,
    texture: Handle<Image>,
) {
    let black_sprite = get_sprite_by_index(T::BLACK_SPRITE_POSITION.0, T::BLACK_SPRITE_POSITION.1);
    let white_sprite = get_sprite_by_index(T::WHITE_SPRITE_POSITION.0, T::WHITE_SPRITE_POSITION.1);

    for (x, y) in T::BLACK_BOARD_POSITION {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (*x as f32) * TILE_SIZE + HALF_TILE,
                    (*y as f32) * TILE_SIZE + HALF_TILE,
                    1.,
                ),
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    rect: Some(black_sprite),
                    ..default()
                },
                ..default()
            },
            Piece::<T>::new((*x, *y), Color::Black),
        ));
    }

    for (x, y) in T::WHITE_BOARD_POSITION {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (*x as f32) * TILE_SIZE + HALF_TILE,
                    (*y as f32) * TILE_SIZE + HALF_TILE,
                    1.,
                ),
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    rect: Some(white_sprite),
                    ..default()
                },
                ..default()
            },
            Piece::<T>::new((*x, *y), Color::White),
        ));
    }
}

// Get sprite from the texture according to the indexes
fn get_sprite_by_index(xi: u8, yi: u8) -> Rect {
    let xi = xi as f32;
    let yi = yi as f32;

    Rect::new(
        xi * SPRITE_SIZE,
        yi * SPRITE_SIZE,
        (xi + 1.0) * SPRITE_SIZE,
        (yi + 1.0) * SPRITE_SIZE,
    )
}
