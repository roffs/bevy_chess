use bevy::{prelude::*, window::PrimaryWindow};

use crate::pieces::Piece;

pub fn mouse_input(
    buttons: Res<ButtonInput<MouseButton>>,
    pieces_query: Query<&Piece>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = windows_query.single().cursor_position() {
            let x = (position.x / 100.0) as u8;
            let y = 7 - (position.y / 100.0) as u8;
            println!("Cursor is inside the primary window, at index {:?}", (x, y));

            for piece in &pieces_query {
                if piece.position == (x, y) {
                    println!("Clicked {:?} {:?}", piece.color, piece.kind);
                }
            }
        }
    }
}
