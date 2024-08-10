mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
mod utils;

use bevy::prelude::*;
use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

use crate::board::get_pixels_by_pos;

const SPRITE_SIZE: f32 = 480.0;

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

trait Kind {
    fn get_valid_moves(
        &self,
        current_position: IVec2,
        color: &Color,
        pieces_on_board: Vec<&Piece>,
    ) -> Vec<IVec2>;
}

trait BuildPieceKind {
    fn new() -> Box<impl Kind + Send + Sync + 'static>;
    fn get_initial_board_position_indices() -> (Vec<IVec2>, Vec<IVec2>);
    fn get_sprites() -> (Rect, Rect);
}

#[derive(Component)]
pub struct Piece {
    pub position: IVec2,
    pub color: Color,
    kind: Box<dyn Kind + Send + Sync + 'static>,
}

impl Piece {
    pub fn get_valid_moves(&self, pieces_on_board: Vec<&Piece>) -> Vec<IVec2> {
        self.kind
            .get_valid_moves(self.position, &self.color, pieces_on_board)
    }

    fn new<T: BuildPieceKind>(position: IVec2, color: Color) -> Piece {
        Piece {
            position,
            color,
            kind: T::new(),
        }
    }

    pub fn spawn_pieces(mut commands: Commands, server: Res<AssetServer>) {
        let texture = server.load("sprites/pieces.png");

        spawn_piece::<King>(&mut commands, texture.clone());
        spawn_piece::<Queen>(&mut commands, texture.clone());
        spawn_piece::<Bishop>(&mut commands, texture.clone());
        spawn_piece::<Knight>(&mut commands, texture.clone());
        spawn_piece::<Rook>(&mut commands, texture.clone());
        spawn_piece::<Pawn>(&mut commands, texture.clone());
    }
}

fn spawn_piece<T: BuildPieceKind>(commands: &mut Commands, texture: Handle<Image>) {
    let (white_sprite, black_sprite) = T::get_sprites();
    let (w_positions, b_positions) = T::get_initial_board_position_indices();

    for position in w_positions {
        let pixel_pos = get_pixels_by_pos(position);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pixel_pos.x, pixel_pos.y, 1.),
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    rect: Some(white_sprite),
                    ..default()
                },
                ..default()
            },
            Piece::new::<T>(position, Color::White),
        ));
    }

    for position in b_positions {
        let pixel_pos = get_pixels_by_pos(position);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pixel_pos.x, pixel_pos.y, 1.),
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    rect: Some(black_sprite),
                    ..default()
                },
                ..default()
            },
            Piece::new::<T>(position, Color::Black),
        ));
    }
}
