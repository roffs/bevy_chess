use super::{utils::get_sprite_by_index, BuildPieceKind, Color, Kind, Piece};
use bevy::prelude::*;

pub(super) struct Knight;

impl Kind for Knight {
    fn get_valid_moves(
        &self,
        current_position: IVec2,
        color: &Color,
        pieces_on_board: Vec<&Piece>,
    ) -> Vec<IVec2> {
        let mut valid_moves: Vec<IVec2> = vec![];

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

                if target_piece.map_or(true, |p| p.color != *color) {
                    valid_moves.push(new_position);
                }
            }
        }

        valid_moves
    }
}

impl BuildPieceKind for Knight {
    fn new() -> Box<impl Kind> {
        Box::new(Knight)
    }
    fn get_initial_board_position_indices() -> (Vec<IVec2>, Vec<IVec2>) {
        (
            vec![IVec2::new(1, 0), IVec2::new(6, 0)],
            vec![IVec2::new(1, 7), IVec2::new(6, 7)],
        )
    }

    fn get_sprites() -> (Rect, Rect) {
        let (w_index, b_index) = (IVec2::new(3, 0), IVec2::new(3, 1));
        (get_sprite_by_index(w_index), get_sprite_by_index(b_index))
    }
}
