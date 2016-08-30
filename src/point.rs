#[derive(PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point { x: x, y: y }
    }

    pub fn offset(&self, dx: i16, dy: i16) -> Point {
        Point { x: self.x + dx, y: self.y + dy, .. *self }
    }

    pub fn left(&self)  -> Point { self.offset(-1, 0) }
    pub fn right(&self) -> Point { self.offset(1, 0) }
    pub fn down(&self)  -> Point { self.offset(0, 1) }
    pub fn up(&self)    -> Point { self.offset(0, -1) }
}
