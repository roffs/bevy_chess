use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::{
    board::{get_pixels_by_pos, get_pos_from_pixel},
    pieces::Piece,
};

#[derive(Component)]
pub struct Selected;

pub fn select_piece(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    pieces_query: Query<(Entity, &Piece)>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(mouse_pos) = windows_query.single().cursor_position() {
            let new_position = get_pos_from_pixel(mouse_pos);
            for (entity, piece) in &pieces_query {
                if piece.position == new_position {
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
            transform.translation.z = 2.0;
        }
    }
}

pub fn release_piece(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut selected_piece_query: Query<(Entity, &mut Piece), With<Selected>>,
    pieces_query: Query<(Entity, &Piece), Without<Selected>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_released(MouseButton::Left) {
        if let Ok((entity, mut piece)) = selected_piece_query.get_single_mut() {
            if let Some(mouse_pos) = windows_query.single().cursor_position() {
                let target_pos = get_pos_from_pixel(mouse_pos);

                // Get all valid movements for the selected piece
                let valid_moves =
                    piece.get_valid_moves(pieces_query.iter().map(|(_, p)| p).collect());

                // Check if move is one from the valid ones
                let move_is_valid = valid_moves.iter().any(|valid_pos| valid_pos == &target_pos);

                if move_is_valid {
                    piece.position = target_pos;

                    // Despawn piece if there is any in a valid spot
                    if let Some((target_entity, _)) = pieces_query
                        .iter()
                        .find(|(_, piece)| piece.position == target_pos)
                    {
                        commands.entity(target_entity).despawn();
                    }
                }
                commands.entity(entity).remove::<Selected>();
            }
        }
    }
}

pub fn set_piece_position(
    mut removed: RemovedComponents<Selected>,
    mut query: Query<(&Piece, &mut Transform)>,
) {
    for entity in removed.read() {
        if let Ok((piece, mut transform)) = query.get_mut(entity) {
            let pos = get_pixels_by_pos(piece.position);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
            transform.translation.z = 1.0;
        }
    }
}
