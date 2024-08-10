use super::{utils::get_sprite_by_index, BuildPieceKind, Color, Kind, Piece};
use bevy::prelude::*;

pub(super) struct Pawn;

impl Kind for Pawn {
    fn get_valid_moves(
        &self,
        current_position: IVec2,
        color: &Color,
        pieces_on_board: Vec<&Piece>,
    ) -> Vec<IVec2> {
        let mut valid_moves: Vec<IVec2> = vec![];

        let vertical_move = match color == &Color::White {
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
            if (color == &Color::Black && current_position.y == 6)
                || (color == &Color::White && current_position.y == 1)
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

            if target_piece.is_some_and(|p| p.color != *color) {
                valid_moves.push(new_position);
            }
        }

        valid_moves
    }
}

impl BuildPieceKind for Pawn {
    fn new() -> Box<impl Kind> {
        Box::new(Pawn)
    }

    fn get_initial_board_position_indices() -> (Vec<IVec2>, Vec<IVec2>) {
        (
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
        )
    }

    fn get_sprites() -> (Rect, Rect) {
        let (w_index, b_index) = (IVec2::new(5, 0), IVec2::new(5, 1));
        (get_sprite_by_index(w_index), get_sprite_by_index(b_index))
    }
}
