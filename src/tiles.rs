use bevy::prelude::*;
use grid::{Grid, Order};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::cursor::mark;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(12, 12));
        app.observe(on_add_tile);
        app.observe(on_remove_tile);
        // app.add_systems(Startup, setup_board);
        // app.add_systems(Update, (set_tiles, gravity));
        app.add_systems(Update, (set_tiles, gravity.after(mark)));
    }
}

pub const TILE_SIZE: UVec2 = UVec2::new(48, 54);
pub const BOARD_SIZE: UVec2 = UVec2::new(12, 12);

#[derive(Component, Default)]
pub struct Tile;
// state:

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    position: Position,
    color: Color,
    shape: Shape,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
}

#[derive(Component, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Position(pub UVec2);

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
        // self.0.y.partial_cmp(&other.0.y)
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.y.cmp(&other.0.y)
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position(UVec2 {
            x: x as u32,
            y: y as u32,
        })
    }
}

#[derive(Event)]
pub struct Move;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Blue,
    Red,
    Yellow,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shape {
    Diamond,
    Circle,
    Triangle,
    //Star,
    //Cross,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=2) {
            0 => Color::Blue,
            1 => Color::Red,
            _ => Color::Yellow,
        }
    }
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..=2) {
            0 => Shape::Diamond,
            1 => Shape::Circle,
            _ => Shape::Triangle,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Board {
    pub tiles: Grid<Option<Entity>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        Board {
            tiles: Grid::new_with_order(rows, cols, Order::ColumnMajor),
        }
    }
    // fn get_tile(position: Position)
}

pub fn spawn_board(
    mut commands: Commands,
    board: Res<Board>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let shapes: Handle<Image> = asset_server.load("card_shapes.png");
    let atlas_layout = TextureAtlasLayout::from_grid((48, 56).into(), 3, 3, None, None);
    let atlas_layouts = layouts.add(atlas_layout);
    for ((x, y), e) in board.tiles.indexed_iter() {
        let pos = Position::new(x, y);
        // println!("{:?}, {:?}", x, y);
        commands.spawn(TileBundle {
            tile: Tile,
            color: rand::random::<Color>(),
            shape: rand::random::<Shape>(),
            sprite: SpriteBundle {
                texture: shapes.clone(),
                transform: position_to_transform(&pos),
                ..default()
            },
            position: pos,
            atlas: TextureAtlas {
                layout: atlas_layouts.clone(),
                index: 0,
            },
        });
    }
}

//when should it run? every frame?
fn set_tiles(mut query: Query<(&Color, &Shape, &mut TextureAtlas)>) {
    for (color, shape, mut atlas) in query.iter_mut() {
        let c = match color {
            Color::Blue => 0,
            Color::Red => 1,
            Color::Yellow => 2,
        };
        let s = match shape {
            Shape::Diamond => 0,
            Shape::Circle => 3,
            Shape::Triangle => 6,
        };
        let index = c + s;
        atlas.index = index;
    }
}

pub fn position_to_transform(position: &Position) -> Transform {
    let board_size = BOARD_SIZE.as_vec2() * TILE_SIZE.as_vec2();
    let offset = board_size / 2. - TILE_SIZE.as_vec2() / 2.;
    let pos = position.as_vec2() * TILE_SIZE.as_vec2();
    Transform::from_xyz(pos.x - offset.x, pos.y - offset.y, 0.0)
}

//add tile to the board at its position
fn on_add_tile(
    trigger: Trigger<OnAdd, Tile>,
    query: Query<&Position, With<Tile>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
) {
    let tile_pos = query.get(trigger.entity()).unwrap();
    // println!("tile_pos on add {:?}", tile_pos);
    if let Some(slot) = board.tiles.get_mut(tile_pos.x, tile_pos.y) {
        *slot = Some(trigger.entity());
    }
    commands.entity(trigger.entity()).observe(on_move_tile);
    //set transform from position?
}

fn on_remove_tile(
    trigger: Trigger<OnRemove, Tile>,
    query: Query<&Position, With<Tile>>,
    mut board: ResMut<Board>,
) {
    let tile_pos = query.get(trigger.entity()).unwrap();
    if let Some(slot) = board.tiles.get_mut(tile_pos.x, tile_pos.y) {
        println!("{:?}", slot);
        *slot = None;
    }
    // println!("despawned entity {:?}", trigger.entity());
    // println!("with tile_pos {:?}", tile_pos);
}

fn on_move_tile(
    trigger: Trigger<Move>,
    mut query: Query<(Entity, &mut Transform, &Position), With<Tile>>,
    board: ResMut<Board>,
) {
    // println!("on_move triggered");
    if let Ok((entity, mut transform, position)) = query.get_mut(trigger.entity()) {
        *transform = position_to_transform(position);
        // println!("transform updated {:?}", transform);
    }
    //moev the tile, set a moving state, do some animation shit?
}

fn gravity(
    mut query: Query<(Entity, &mut Position), With<Tile>>,
    mut board: ResMut<Board>,
    mut commands: Commands,
) {
    //iterate slots starting from 2nd row
    for x in 0..BOARD_SIZE.x as usize {
        for y in 1..BOARD_SIZE.y as usize {
            //if no tile is below
            if board.tiles[(x, y - 1)].is_none() {
                //and there is one here
                if let Some(tile_entity) = board.tiles[(x, y)] {
                    //swap them
                    (board.tiles[(x, y)], board.tiles[(x, y - 1)]) =
                        (board.tiles[(x, y - 1)], board.tiles[(x, y)]);
                    if let Ok((e, mut position)) = query.get_mut(tile_entity) {
                        position.y -= 1;
                        commands.trigger_targets(Move, e);
                    }
                }
            }
        }
    }
}
