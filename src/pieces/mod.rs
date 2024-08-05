use bevy::prelude::*;

use crate::board::{HALF_TILE, TILE_SIZE};

const SPRITE_SIZE: f32 = 480.0;

#[derive(Component, Clone, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Debug)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl Kind {
    fn get_sprite_indices(&self) -> ((u8, u8), (u8, u8)) {
        match self {
            Kind::King => ((0, 0), (0, 1)),
            Kind::Queen => ((1, 0), (1, 1)),
            Kind::Bishop => ((2, 0), (2, 1)),
            Kind::Knight => ((3, 0), (3, 1)),
            Kind::Rook => ((4, 0), (4, 1)),
            Kind::Pawn => ((5, 0), (5, 1)),
        }
    }

    #[allow(clippy::type_complexity)]
    fn get_initial_board_position_indices(&self) -> (&[(u8, u8)], &[(u8, u8)]) {
        match self {
            Kind::King => (&[(4, 0)], &[(4, 7)]),
            Kind::Queen => (&[(3, 0)], &[(3, 7)]),
            Kind::Bishop => (&[(2, 0), (5, 0)], &[(2, 7), (5, 7)]),
            Kind::Knight => (&[(1, 0), (6, 0)], &[(1, 7), (6, 7)]),
            Kind::Rook => (&[(0, 0), (7, 0)], &[(0, 7), (7, 7)]),
            #[rustfmt::skip]
            Kind::Pawn => (
                &[(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1)],
                 &[(0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6)]
            ),
        }
    }
}

#[derive(Component)]
pub struct Piece {
    pub position: (u8, u8),
    pub color: Color,
    pub kind: Kind,
}

pub fn spawn_pieces(mut commands: Commands, server: Res<AssetServer>) {
    let texture = server.load("sprites/pieces.png");

    spawn_piece(&mut commands, texture.clone(), Kind::King);
    spawn_piece(&mut commands, texture.clone(), Kind::Queen);
    spawn_piece(&mut commands, texture.clone(), Kind::Bishop);
    spawn_piece(&mut commands, texture.clone(), Kind::Knight);
    spawn_piece(&mut commands, texture.clone(), Kind::Rook);
    spawn_piece(&mut commands, texture.clone(), Kind::Pawn);
}

fn spawn_piece(commands: &mut Commands, texture: Handle<Image>, kind: Kind) {
    let (w_sprite_index, b_sprite_index) = kind.get_sprite_indices();

    let white_sprite = get_sprite_by_index(w_sprite_index.0, w_sprite_index.1);
    let black_sprite = get_sprite_by_index(b_sprite_index.0, b_sprite_index.1);

    let (w_positions, b_positions) = kind.get_initial_board_position_indices();

    for (x, y) in w_positions {
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
            Piece {
                position: (*x, *y),
                color: Color::White,
                kind: kind.clone(),
            },
        ));
    }

    for (x, y) in b_positions {
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
            Piece {
                position: (*x, *y),
                color: Color::Black,
                kind: kind.clone(),
            },
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
