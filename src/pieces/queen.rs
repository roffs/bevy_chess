use super::Behavior;

pub struct Queen;

impl Behavior for Queen {
    const BLACK_SPRITE_POSITION: (u8, u8) = (1, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (1, 1);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(3, 0)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(3, 7)];

    fn new() -> Self {
        Self
    }
}
