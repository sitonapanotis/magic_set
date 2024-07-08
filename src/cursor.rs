//cursor

use crate::tiles::{position_to_transform, Board, Position, BOARD_SIZE, TILE_SIZE};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CursorAction>::default());
        // app.add_systems(Startup, spawn_cursor);
    }
}

#[derive(Component)]
pub struct Cursor;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CursorAction {
    Move,
    Select,
}

pub fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // board: Res<Board>,
) {
    let handle: Handle<Image> = asset_server.load("cursor_w.png");
    let pos = Position::new(0, 0);

    let input_map = InputMap::new([CursorAction::Select, KeyCode::Space]);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite::default(),
            texture: handle.clone(),
            transform: position_to_transform(&pos),
            ..default()
        })
        .insert(Cursor)
        .insert(pos);
}
