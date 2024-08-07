use bevy::prelude::*;

use crate::board::{get_position_by_index, HALF_TILE, TILE_SIZE};

const SPRITE_SIZE: f32 = 480.0;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
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
    fn get_sprite_indices(&self) -> ((i8, i8), (i8, i8)) {
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
    fn get_initial_board_position_indices(&self) -> (&[(i8, i8)], &[(i8, i8)]) {
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

#[derive(Component, Debug)]
pub struct Piece {
    pub position: (i8, i8),
    pub color: Color,
    pub kind: Kind,
}

impl Piece {
    pub fn get_valid_moves(&self, pieces_on_board: Vec<&Piece>) -> Vec<(i8, i8)> {
        let current_position = self.position;
        let mut valid_moves: Vec<(i8, i8)> = vec![];

        match self.kind {
            Kind::King => {
                let king_moves = [
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                ];

                for movement in king_moves {
                    let new_position = (
                        current_position.0 + movement.0,
                        current_position.1 + movement.1,
                    );
                    if (0..8).contains(&new_position.0) && (0..8).contains(&new_position.1) {
                        let target_piece = pieces_on_board
                            .iter()
                            .find(|piece| piece.position == new_position);

                        if let Some(piece) = target_piece {
                            if self.color != piece.color {
                                valid_moves.push(new_position);
                            }
                        } else {
                            valid_moves.push(new_position);
                        }
                    }
                }
            }
            Kind::Queen => {
                let directions = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                    (-1, -1),
                ];

                for direction in directions {
                    add_moves_in_direction(
                        current_position,
                        direction,
                        &self.color,
                        &pieces_on_board,
                        &mut valid_moves,
                    )
                }
            }
            Kind::Bishop => {
                let directions = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

                for direction in directions {
                    add_moves_in_direction(
                        current_position,
                        direction,
                        &self.color,
                        &pieces_on_board,
                        &mut valid_moves,
                    )
                }
            }
            Kind::Knight => {
                let knight_moves = [
                    (-2, 1),
                    (-1, 2),
                    (1, 2),
                    (2, 1),
                    (2, -1),
                    (1, -2),
                    (-1, -2),
                    (-2, -1),
                ];

                for movement in knight_moves {
                    let new_position = (
                        current_position.0 + movement.0,
                        current_position.1 + movement.1,
                    );
                    if (0..8).contains(&new_position.0) && (0..8).contains(&new_position.1) {
                        let target_piece = pieces_on_board
                            .iter()
                            .find(|piece| piece.position == new_position);

                        if let Some(piece) = target_piece {
                            if self.color != piece.color {
                                valid_moves.push(new_position);
                            }
                        } else {
                            valid_moves.push(new_position);
                        }
                    }
                }
            }
            Kind::Rook => {
                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for direction in directions {
                    add_moves_in_direction(
                        current_position,
                        direction,
                        &self.color,
                        &pieces_on_board,
                        &mut valid_moves,
                    )
                }
            }
            Kind::Pawn => {
                let y = match self.color == Color::White {
                    true => 1,
                    false => -1,
                };

                // Check if it can advance
                let new_position = (current_position.0, current_position.1 + y);
                let target_piece = pieces_on_board
                    .iter()
                    .find(|piece| piece.position == new_position);

                if target_piece.is_none() {
                    valid_moves.push(new_position);

                    // Move two squares if first pawn move
                    if (self.color == Color::Black && self.position.1 == 6)
                        || (self.color == Color::White && self.position.1 == 1)
                    {
                        let new_position = (new_position.0, new_position.1 + y);
                        let target_piece = pieces_on_board
                            .iter()
                            .find(|piece| piece.position == new_position);
                        if target_piece.is_none() {
                            valid_moves.push(new_position);
                        }
                    }
                }

                // Check if it can capture a piece
                for x in [-1, 1] {
                    let new_position = (current_position.0 + x, current_position.1 + y);

                    let target_piece = pieces_on_board
                        .iter()
                        .find(|piece| piece.position == new_position);

                    if let Some(piece) = target_piece {
                        if self.color != piece.color {
                            valid_moves.push(new_position);
                        }
                    }
                }
            }
        };

        valid_moves
    }
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
        let pos = get_position_by_index(*x, *y);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.0, pos.1, 1.),
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
fn get_sprite_by_index(xi: i8, yi: i8) -> Rect {
    let xi = xi as f32;
    let yi = yi as f32;

    Rect::new(
        xi * SPRITE_SIZE,
        yi * SPRITE_SIZE,
        (xi + 1.0) * SPRITE_SIZE,
        (yi + 1.0) * SPRITE_SIZE,
    )
}

fn add_moves_in_direction(
    current_position: (i8, i8),
    direction_step: (i8, i8),
    color: &Color,
    pieces_on_board: &[&Piece],
    valid_moves: &mut Vec<(i8, i8)>,
) {
    let (mut xi, mut yi) = (0, 0);
    while (0..8).contains(&(current_position.0 + xi)) && (0..8).contains(&(current_position.1 + yi))
    {
        let new_position = (current_position.0 + xi, current_position.1 + yi);
        let target_piece = pieces_on_board
            .iter()
            .find(|piece| piece.position == new_position);

        if let Some(piece) = target_piece {
            if color != &piece.color {
                valid_moves.push(new_position);
            }
            break;
        }
        valid_moves.push(new_position);

        xi += direction_step.0;
        yi += direction_step.1;
    }
}
