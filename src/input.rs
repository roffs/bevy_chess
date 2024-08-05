use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::pieces::Piece;

#[derive(Component)]
pub struct Selected;

pub fn select_piece(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    pieces_query: Query<(Entity, &Piece)>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = windows_query.single().cursor_position() {
            let x = (position.x / 100.0) as u8;
            let y = 7 - (position.y / 100.0) as u8;
            for (entity, piece) in &pieces_query {
                if piece.position == (x, y) {
                    commands.entity(entity).insert(Selected);
                }
            }
        }
    }
}

pub fn move_piece(
    mut piece_query: Query<&mut Transform, With<Selected>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if let Ok(mut transform) = piece_query.get_single_mut() {
        for event in mouse_motion.read() {
            transform.translation.x += event.delta.x;
            transform.translation.y -= event.delta.y;
        }
    }
}

pub fn release_piece(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    piece_query: Query<Entity, (With<Piece>, With<Selected>)>,
) {
    if buttons.just_released(MouseButton::Left) {
        if let Ok(entity) = piece_query.get_single() {
            commands.entity(entity).remove::<Selected>();
        }
    }
}
