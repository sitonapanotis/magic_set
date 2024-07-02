use bevy::prelude::*;
use grid::{Grid, Order};
use rand::{distributions::Standard, prelude::Distribution, Rng};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(12, 12));
        app.observe(on_add_tile);
        app.add_systems(Startup, setup_board);
        app.add_systems(Update, set_tiles);
        //observer?
        // app.add_systems(Update, set_tiles);
    }
}

#[derive(Component, Default)]
struct Tile;
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

#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
}

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

#[derive(Resource)]
pub struct Board {
    pub tiles: Grid<Option<Entity>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        Board {
            tiles: Grid::new_with_order(rows, cols, Order::ColumnMajor),
        }
    }
}

fn setup_board(
    mut commands: Commands,
    board: Res<Board>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let shapes: Handle<Image> = asset_server.load("card_shapes.png");
    let atlas_layout = TextureAtlasLayout::from_grid((48, 56).into(), 3, 3, None, None);
    let atlas_layouts = layouts.add(atlas_layout);
    for ((x, y), e) in board.tiles.indexed_iter() {
        commands.spawn(TileBundle {
            tile: Tile,
            position: Position { x, y },
            color: rand::random::<Color>(),
            shape: rand::random::<Shape>(),
            sprite: SpriteBundle {
                texture: shapes.clone(),
                ..default()
            },
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

//add tile to the board at its position
fn on_add_tile(
    trigger: Trigger<OnAdd, Tile>,
    query: Query<&Position, With<Tile>>,
    mut board: ResMut<Board>,
) {
    let tile_pos = query.get(trigger.entity()).unwrap();
    if let Some(mut entity) = board.tiles.get_mut(tile_pos.x, tile_pos.y) {
        *entity = Some(trigger.entity());
    }
    //set transform from position?
}

fn move_tile(mut query: Query<(Entity, &Position, &Tile)>, board: ResMut<Board>) {
    //move event? OnInsert?
    //moev the tile, set a moving state, do some animation shit?
}
