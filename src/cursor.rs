//cursor

use crate::tiles::{position_to_transform, Board, Position, BOARD_SIZE};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CursorAction>::default());
        app.add_systems(Update, (move_cursor, mark).chain());
    }
}

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct Mark;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CursorAction {
    Move,
    Select,
}

#[derive(Component)]
enum State {
    Idle,
    Select,
}

pub fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Image> = asset_server.load("cursor_w.png");
    let pos = Position::new(0, 0);

    let input_map = InputMap::default()
        .insert(CursorAction::Select, KeyCode::Space)
        .insert(CursorAction::Move, VirtualDPad::wasd())
        .build();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite::default(),
            texture: handle.clone(),
            transform: position_to_transform(&pos),
            ..default()
        },
        Cursor,
        pos,
        State::Idle,
        InputManagerBundle::with_map(input_map),
    ));
}

fn move_cursor(
    mut query: Query<(&mut Position, &mut Transform, &ActionState<CursorAction>), With<Cursor>>,
) {
    for (mut position, mut transform, action_state) in query.iter_mut() {
        if action_state.just_pressed(&CursorAction::Move) {
            let direction = action_state
                .clamped_axis_pair(&CursorAction::Move)
                .unwrap_or_default()
                .xy();
            position.0 = position
                .0
                .saturating_add_signed(direction.as_ivec2())
                .min(BOARD_SIZE - UVec2::new(1, 1));
            *transform = position_to_transform(&position);
        }
    }
}

pub fn mark(
    query: Query<(&Position, &ActionState<CursorAction>), With<Cursor>>,
    board: Res<Board>,
    mut commands: Commands,
) {
    for (position, action_state) in query.iter() {
        if action_state.just_pressed(&CursorAction::Select) {
            if let Some(Some(entity)) = board.tiles.get(position.x, position.y) {
                commands.entity(*entity).despawn();
                // commands.entity(*entity).insert(Mark);
                // println!("marked tile at {:?}", position);
                // println!("despawned tile at {:?}", position);
                // } else {
                // println!("no tile to despawn at {:?}", position);
            }
        }
    }
}

// fn set_mark(
//     mut move_reader: EventReader<MoveEvent>,
//     // query: Query<&TilePos2d, With<Cursor>>,
//     //
//     tile_storage_query: Query<&TileStorage>,
//     marked_tile_query: Query<&TilePos, With<Mark>>,
//     mut commands: Commands,
//     // keys: Res<Input<KeyCode>>,
// ) {
//     let tile_storage = tile_storage_query.single();
//     for evt in move_reader.iter() {
//         if let Some(tile_entity) = tile_storage.get(&evt.0) {
//             if !marked_tile_query.contains(tile_entity) {
//                 commands.entity(tile_entity).insert(Mark);
//             }
//         }
//     }
// }
