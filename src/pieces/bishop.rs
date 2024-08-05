use super::{Color, Piece};
use bevy::prelude::*;

#[derive(Component)]
pub struct Bishop {
    position: (u8, u8),
    color: Color,
}

impl Piece for Bishop {
    const BLACK_SPRITE_POSITION: (u8, u8) = (2, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (2, 1);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(2, 0), (5, 0)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(2, 7), (5, 7)];

    fn new(position: (u8, u8), color: Color) -> Bishop {
        Bishop { position, color }
    }
}
