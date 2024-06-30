use bevy::prelude::*;
use grid::{Grid, Order};
use rand::{distributions::Standard, prelude::Distribution, Rng};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(12, 12));
    }
}

#[derive(Component)]
struct Tile {
    // state:
}

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    sprite: TileSprite,
    //grid position?
    position: Position,
    color: Color,
    shape: Shape,
}

#[derive(Bundle)]
pub struct TileSpriteBundle {
    sprite: SpriteBundle,
    atlas: TextureAtlas,
}

#[derive(Component)]
pub struct TileSprite(Entity);

#[derive(Component)]
struct Position;

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
    tiles: Grid<Option<Entity>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        Board {
            tiles: Grid::new_with_order(rows, cols, Order::ColumnMajor),
        }
    }
}
