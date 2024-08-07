use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::{board::get_position_by_index, pieces::Piece};

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
            let x = (position.x / 100.0) as i8;
            let y = 7 - (position.y / 100.0) as i8;
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
            if let Some(position) = windows_query.single().cursor_position() {
                let x = (position.x / 100.0) as i8;
                let y = 7 - (position.y / 100.0) as i8;

                let target_position = (x, y);

                // Get all valid movements for the selected piece
                let valid_moves =
                    piece.get_valid_moves(pieces_query.iter().map(|(_, p)| p).collect());

                // Check if move is one from the valid ones
                let move_is_valid = valid_moves
                    .iter()
                    .any(|valid_pos| valid_pos == &target_position);

                if move_is_valid {
                    piece.position = target_position;

                    // Despawn piece if there is any in a valid spot
                    if let Some((target_entity, _)) = pieces_query
                        .iter()
                        .find(|(_, piece)| piece.position == target_position)
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
            let pos = get_position_by_index(piece.position.0, piece.position.1);
            transform.translation.x = pos.0;
            transform.translation.y = pos.1;
            transform.translation.z = 1.0;
        }
    }
}
