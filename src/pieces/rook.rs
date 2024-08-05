use super::Behavior;

pub struct Rook;

impl Behavior for Rook {
    const BLACK_SPRITE_POSITION: (u8, u8) = (4, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (4, 1);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(0, 0), (7, 0)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(0, 7), (7, 7)];

    fn new() -> Self {
        Self
    }
}
