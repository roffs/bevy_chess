use super::Behavior;

pub struct Knight;

impl Behavior for Knight {
    const BLACK_SPRITE_POSITION: (u8, u8) = (3, 0);
    const WHITE_SPRITE_POSITION: (u8, u8) = (3, 1);

    const BLACK_BOARD_POSITION: &'static [(u8, u8)] = &[(1, 0), (6, 0)];
    const WHITE_BOARD_POSITION: &'static [(u8, u8)] = &[(1, 7), (6, 7)];

    fn new() -> Self {
        Self
    }
}
