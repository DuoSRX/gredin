use rand::{thread_rng, Rng};
use tile::Tile;

const WORLD_WIDTH: i16 = 100;
const WORLD_HEIGHT: i16 = 100;

pub struct World {
    pub tiles: Vec<Vec<Tile>>
}

impl World {
    pub fn generate() -> World {
        let mut tiles = Vec::with_capacity(50);
        let mut rng = thread_rng();

        for _ in 0..WORLD_HEIGHT {
            tiles.push(World::random_row(&mut rng));
        }

        World { tiles: tiles }
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

    fn block_at(&self, x: i16, y: i16) -> Vec<Tile> {
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
