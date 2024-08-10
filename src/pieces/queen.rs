use super::{add_moves_in_direction, get_sprite_by_index, BuildPieceKind, Color, Kind, Piece};
use bevy::prelude::*;

pub struct Queen;

impl Kind for Queen {
    fn get_valid_moves(
        &self,
        current_position: IVec2,
        color: &Color,
        pieces_on_board: Vec<&Piece>,
    ) -> Vec<IVec2> {
        let mut valid_moves: Vec<IVec2> = vec![];

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
                color,
                &pieces_on_board,
                &mut valid_moves,
            )
        }

        valid_moves
    }
}

impl BuildPieceKind for Queen {
    fn new() -> Box<impl Kind> {
        Box::new(Queen)
    }
    fn get_initial_board_position_indices() -> (Vec<IVec2>, Vec<IVec2>) {
        (vec![IVec2::new(3, 0)], vec![IVec2::new(3, 7)])
    }

    fn get_sprites() -> (Rect, Rect) {
        let (w_index, b_index) = (IVec2::new(1, 0), IVec2::new(1, 1));
        (get_sprite_by_index(w_index), get_sprite_by_index(b_index))
    }
}
