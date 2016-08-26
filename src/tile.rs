#[derive(PartialEq, Clone, Copy)]
pub enum Tile {
    OutOfBound,
    Empty,
    Wall,
}

impl Tile {
    pub fn to_string(&self) -> &str {
        match *self {
            Tile::Wall => "#",
            Tile::Empty => ".",
            Tile::OutOfBound => "-"
        }
    }

    pub fn can_move_through(&self) -> bool {
        match *self {
            Tile::Empty => true,
            _ => false
        }
    }

    pub fn diggable(&self) -> bool {
        *self == Tile::Wall
    }
}
