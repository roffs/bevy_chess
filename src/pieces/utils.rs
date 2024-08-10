use bevy::prelude::*;

use super::{Color, Piece, SPRITE_SIZE};

pub(super) fn get_sprite_by_index(indices: IVec2) -> Rect {
    let xi = indices.x as f32;
    let yi = indices.y as f32;

    Rect::new(
        xi * SPRITE_SIZE,
        yi * SPRITE_SIZE,
        (xi + 1.0) * SPRITE_SIZE,
        (yi + 1.0) * SPRITE_SIZE,
    )
}

pub(super) fn add_moves_in_direction(
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
