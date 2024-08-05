use super::{Color, Piece};
use bevy::prelude::*;

#[derive(Component)]
pub struct Knight {
    position: (u8, u8),
    color: Color,
}

impl Piece for Knight {
    const BLACK_SPRITE_POSITION: (u8, u8) = (3, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (3, 1);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(1, 0), (6, 0)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(1, 7), (6, 7)];

    fn new(position: (u8, u8), color: Color) -> Knight {
        Knight { position, color }
    }
}
