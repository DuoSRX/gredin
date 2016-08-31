extern crate rustbox;
use rand::{thread_rng, Rng};

use tile::Tile;
use point::Point;
use creature::Creature;

const WORLD_WIDTH: i16 = 100;
const WORLD_HEIGHT: i16 = 50;

pub struct World {
    pub tiles: Vec<Vec<Tile>>,
    pub creatures: Vec<Box<Creature>>,
}

impl World {
    pub fn dig(&mut self, x: i16, y: i16) {
        if self.at(x, y).diggable() {
            self.tiles[y as usize][x as usize] = Tile::Empty;
        }
    }

    // Yeah don't run that if the world doesn't have an empty tile somewhere...
    pub fn random_empty_location(&self) -> Point {
        let mut rng = thread_rng();
        let mut point: Option<Point> = None;

        while point.is_none() {
            let x = rng.gen_range(0, WORLD_WIDTH - 1);
            let y = rng.gen_range(0, WORLD_HEIGHT - 1);
            let tile = self.at(x, y);
            if tile == Tile::Empty {
                point = Some(Point { x: x, y: y });
            }
        }

        point.unwrap()
    }

    pub fn random_free_square(&self, x: i16, y: i16) -> Point {
        let mut rng = thread_rng();
        let mut block = Vec::new();

        for dy in -1..2 {
            for dx in -1..2 {
                let point = Point { x: x + dx, y: y + dy };
                if self.at(point.x, point.y).can_move_through() {
                    block.push(point);
                }
            }
        }

        let default = Point { x: x, y: y };
        *rng.choose(block.as_slice()).unwrap_or(&default)
    }

    pub fn generate() -> World {
        let mut tiles = Vec::with_capacity(50);
        let mut rng = thread_rng();

        for _ in 0..WORLD_HEIGHT {
            tiles.push(World::random_row(&mut rng));
        }

        let mut world = World {
            tiles: tiles,
            creatures: Vec::new(),
        };

        for _ in 0..7 {
            world.smooth();
        }

        world
    }

    pub fn smooth(&mut self) {
        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                let block = self.block_at(x, y);
                let empty_count = block.iter().filter(|tile| **tile == Tile::Empty).count();
                let wall_count = block.iter().filter(|tile| **tile == Tile::Wall).count();

                if empty_count >= wall_count {
                    self.tiles[y as usize][x as usize] = Tile::Empty;
                } else {
                    self.tiles[y as usize][x as usize] = Tile::Wall;
                }
            }
        }

    }

    pub fn at(&self, x: i16, y: i16) -> Tile {
        *self.tiles.get(y as usize).and_then(|row| row.get(x as usize)).unwrap_or(&Tile::Wall)
    }

    fn random_row(mut rng: &mut Rng) -> Vec<Tile> {
        let mut row: Vec<Tile> = Vec::with_capacity(50);

        for _ in 0..WORLD_WIDTH {
            row.push(World::random_tile(&mut rng))
        }

        row
    }

    fn random_tile(rng: &mut Rng) -> Tile {
        let n = rng.next_f32();
        if n < 0.45 {
            Tile::Wall
        } else {
            Tile::Empty
        }
    }

    pub fn block_at(&self, x: i16, y: i16) -> Vec<Tile> {
        let mut block = Vec::new();

        for dy in -1..2 {
            for dx in -1..2 {
                let tile = self.at(x + dx, y + dy);
                block.push(tile);
            }
        }

        block
    }
}
