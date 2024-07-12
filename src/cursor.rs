//cursor

use crate::tiles::{
    position_and_z_to_transform, position_to_transform, Board, Color, Position, Shape, Tile,
    BOARD_SIZE,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CursorAction>::default());
        app.add_systems(Update, (move_cursor, mark, check_match).chain());
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
            transform: position_and_z_to_transform(&pos, 1.),
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
            *transform = position_and_z_to_transform(&position, 1.);
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
                // commands.entity(*entity).despawn();
                commands.entity(*entity).insert(Mark);
                // println!("marked tile at {:?}", position);
                // println!("despawned tile at {:?}", position);
                // } else {
                // println!("no tile to despawn at {:?}", position);
            }
        }
    }
}

fn check_match(
    query: Query<(&Color, &Shape), (With<Tile>, With<Mark>)>,
    entities: Query<Entity, (With<Mark>)>,
    mut commands: Commands,
) {
    let (colors, shapes): (Vec<&Color>, Vec<&Shape>) = query.iter().unzip();
    //TODO:
    if colors.len() < 3 {
        return;
    }
    let mut unique_colors = colors.clone();
    unique_colors.sort();
    unique_colors.dedup();
    let color_match = colors.len() == unique_colors.len() || unique_colors.len() == 1;
    let mut shape_match = false;
    let mut unique_shapes = shapes.clone();
    unique_shapes.sort();
    unique_shapes.dedup();
    let shape_match = shapes.len() == unique_shapes.len() || unique_shapes.len() == 1;

    if shape_match && color_match {
        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }
    }
}

// fn check_match(
//     query: Query<(&Color, &Shape), With<Mark>>,
//     entities: Query<(Entity, &TilePos), (With<Mark>, With<Color>, With<Shape>)>,
//     mut match_event: EventWriter<RemoveEvent>,
//     mut commands: Commands,
// ) {
//     let (mut colors, mut shapes): (Vec<&Color>, Vec<&Shape>) = query.iter().unzip();
//     let mut color_match = false;
//     colors.sort_unstable();
//     // println!("{:?}", colors);
//     let mut unique_colors = colors.clone();
//     unique_colors.dedup();
//     // println!("{:?}", unique_colors);
//     if colors.len() == unique_colors.len() || unique_colors.len() == 1 {
//         color_match = true;
//     } else {
//         // println!("no match")
//     }
//     let mut shape_match = false;
//     shapes.sort_unstable();
//     // println!("{:?}", shapes);
//     let mut unique_shapes = shapes.clone();
//     unique_shapes.dedup();
//     // println!("{:?}", unique_shapes);
//     if shapes.len() == unique_shapes.len() || unique_shapes.len() == 1 {
//         shape_match = true;
//     } else {
//         // println!("no match")
//     }
//     if shape_match && color_match {
//         println!("Match!!");
//         for (entity, pos) in entities.iter() {
//             match_event.send(RemoveEvent(entity, *pos))
//         }
//     }
//     commands.insert_resource(NextState(GameState::Match));
// }
