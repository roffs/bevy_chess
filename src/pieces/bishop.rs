use super::Behavior;

pub struct Bishop;

impl Behavior for Bishop {
    const BLACK_SPRITE_POSITION: (u8, u8) = (2, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (2, 1);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(2, 0), (5, 0)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(2, 7), (5, 7)];

    fn new() -> Self {
        Self
    }
}
