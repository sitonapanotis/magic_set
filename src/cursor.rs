//cursor

use crate::tiles::{Board, Position};
use bevy::prelude::*;

fn spawn_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
    // query: Query<Entity, (With<TileStorage>, Added<TileStorage>)>,
    // grid_size: Res<TilemapGridSize>,
    // tilemap_size: Res<TilemapSize>,
) {
    // if query.get_single().is_ok() {
    // let handle: Handle<Image> = asset_server.load("cursor_w.png");
    // let tile_pos = TilePos { x: 0, y: 0 };

    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             // anchor: Anchor::BottomLeft,
    //             ..default()
    //         },
    //         texture: handle.clone(),
    //         transform: get_tilemap_center_transform(
    //             &TILEMAP_METADATA.size,
    //             &TILEMAP_METADATA.grid_size,
    //             &TilemapType::default(),
    //             1.0,
    //         ),
    //         ..default()
    //     })
    //     .insert(Cursor)
    //     .insert(tile_pos);
    // }
}
