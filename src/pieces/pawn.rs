use super::{Color, Piece};
use bevy::prelude::*;

#[derive(Component)]
pub struct Pawn {
    position: (u8, u8),
    color: Color,
}

impl Piece for Pawn {
    const BLACK_SPRITE_POSITION: (u8, u8) = (5, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (5, 1);

    #[rustfmt::skip]
    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1)];
    #[rustfmt::skip]
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6), (7, 6)];

    fn new(position: (u8, u8), color: Color) -> Pawn {
        Pawn { position, color }
    }
}
