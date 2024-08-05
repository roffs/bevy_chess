use super::{Color, Piece};
use bevy::prelude::*;

#[derive(Component)]
pub struct King {
    position: (u8, u8),
    color: Color,
}

impl Piece for King {
    const BLACK_SPRITE_POSITION: (u8, u8) = (0, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (0, 1);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(4, 0)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(4, 7)];

    fn new(position: (u8, u8), color: Color) -> King {
        King{ position, color }
    }
}
