use bevy::prelude::*;

use crate::board::get_pixels_by_pos;

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
    fn get_initial_board_position_indices(&self) -> (Vec<IVec2>, Vec<IVec2>) {
        match self {
            Kind::King => (vec![IVec2::new(4, 0)], vec![IVec2::new(4, 7)]),
            Kind::Queen => (vec![IVec2::new(3, 0)], vec![IVec2::new(3, 7)]),
            Kind::Bishop => (
                vec![IVec2::new(2, 0), IVec2::new(5, 0)],
                vec![IVec2::new(2, 7), IVec2::new(5, 7)],
            ),
            Kind::Knight => (
                vec![IVec2::new(1, 0), IVec2::new(6, 0)],
                vec![IVec2::new(1, 7), IVec2::new(6, 7)],
            ),
            Kind::Rook => (
                vec![IVec2::new(0, 0), IVec2::new(7, 0)],
                vec![IVec2::new(0, 7), IVec2::new(7, 7)],
            ),
            // #[rustfmt::skip]
            Kind::Pawn => (
                vec![
                    IVec2::new(0, 1),
                    IVec2::new(1, 1),
                    IVec2::new(2, 1),
                    IVec2::new(3, 1),
                    IVec2::new(4, 1),
                    IVec2::new(5, 1),
                    IVec2::new(6, 1),
                    IVec2::new(7, 1),
                ],
                vec![
                    IVec2::new(0, 6),
                    IVec2::new(1, 6),
                    IVec2::new(2, 6),
                    IVec2::new(3, 6),
                    IVec2::new(4, 6),
                    IVec2::new(5, 6),
                    IVec2::new(6, 6),
                    IVec2::new(7, 6),
                ],
            ),
        }
    }
}

#[derive(Component, Debug)]
pub struct Piece {
    pub position: IVec2,
    pub color: Color,
    pub kind: Kind,
}

impl Piece {
    pub fn get_valid_moves(&self, pieces_on_board: Vec<&Piece>) -> Vec<IVec2> {
        let current_position = self.position;
        let mut valid_moves: Vec<IVec2> = vec![];

        match self.kind {
            Kind::King => {
                let king_moves = [
                    IVec2::new(0, 1),
                    IVec2::new(1, 1),
                    IVec2::new(1, 0),
                    IVec2::new(1, -1),
                    IVec2::new(1, -1),
                    IVec2::new(0, -1),
                    IVec2::new(-1, -1),
                    IVec2::new(-1, 0),
                    IVec2::new(-1, 1),
                ];

                for movement in king_moves {
                    let new_position = current_position + movement;
                    if (0..8).contains(&new_position.x) && (0..8).contains(&new_position.y) {
                        let target_piece = pieces_on_board
                            .iter()
                            .find(|piece| piece.position == new_position);

                        if target_piece.map_or(true, |p| p.color != self.color) {
                            valid_moves.push(new_position);
                        }
                    }
                }
            }
            Kind::Queen => {
                let directions = [
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(0, 1),
                    IVec2::new(0, -1),
                    IVec2::new(1, 1),
                    IVec2::new(-1, 1),
                    IVec2::new(1, -1),
                    IVec2::new(-1, -1),
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
                let directions = [
                    IVec2::new(1, 1),
                    IVec2::new(-1, 1),
                    IVec2::new(1, -1),
                    IVec2::new(-1, -1),
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
            Kind::Knight => {
                let knight_moves = [
                    IVec2::new(-2, 1),
                    IVec2::new(-1, 2),
                    IVec2::new(1, 2),
                    IVec2::new(2, 1),
                    IVec2::new(2, -1),
                    IVec2::new(1, -2),
                    IVec2::new(-1, -2),
                    IVec2::new(-2, -1),
                ];

                for movement in knight_moves {
                    let new_position = current_position + movement;
                    if (0..8).contains(&new_position.x) && (0..8).contains(&new_position.y) {
                        let target_piece = pieces_on_board
                            .iter()
                            .find(|piece| piece.position == new_position);

                        if target_piece.map_or(true, |p| p.color != self.color) {
                            valid_moves.push(new_position);
                        }
                    }
                }
            }
            Kind::Rook => {
                let directions = [
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(0, 1),
                    IVec2::new(0, -1),
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
            Kind::Pawn => {
                let vertical_move = match self.color == Color::White {
                    true => IVec2::new(0, 1),
                    false => IVec2::new(0, -1),
                };

                // Check if it can advance
                let new_position = current_position + vertical_move;
                let target_piece = pieces_on_board
                    .iter()
                    .find(|piece| piece.position == new_position);

                if target_piece.is_none() {
                    valid_moves.push(new_position);

                    // Check if it can advance an extra tile if first pawn move
                    if (self.color == Color::Black && self.position.y == 6)
                        || (self.color == Color::White && self.position.y == 1)
                    {
                        let new_position = new_position + vertical_move;
                        let target_piece = pieces_on_board
                            .iter()
                            .find(|piece| piece.position == new_position);
                        if target_piece.is_none() {
                            valid_moves.push(new_position);
                        }
                    }
                }

                // Check if it can capture a piece
                for horizontal_move in [IVec2::new(-1, 0), IVec2::new(1, 0)] {
                    let new_position = current_position + vertical_move + horizontal_move;
                    let target_piece = pieces_on_board
                        .iter()
                        .find(|piece| piece.position == new_position);

                    if target_piece.is_some_and(|p| p.color != self.color) {
                        valid_moves.push(new_position);
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

    for position in w_positions {
        let pixel_pos = get_pixels_by_pos(position);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pixel_pos.x, pixel_pos.y, 1.),
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    rect: Some(white_sprite),
                    ..default()
                },
                ..default()
            },
            Piece {
                position,
                color: Color::White,
                kind: kind.clone(),
            },
        ));
    }

    for position in b_positions {
        let pixel_pos = get_pixels_by_pos(position);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pixel_pos.x, pixel_pos.y, 1.),
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    rect: Some(black_sprite),
                    ..default()
                },
                ..default()
            },
            Piece {
                position,
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
    current_position: IVec2,
    direction: IVec2,
    color: &Color,
    pieces_on_board: &[&Piece],
    valid_moves: &mut Vec<IVec2>,
) {
    let mut new_position = current_position + direction;
    while (0..8).contains(&(new_position.x)) && (0..8).contains(&new_position.y) {
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

        new_position += direction;
    }
}
