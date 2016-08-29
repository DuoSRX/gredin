use point::Point;

pub struct Creature {
    glyph: char,
    pub location: Point,
}

impl Creature {
    pub fn new(x: i16, y: i16, glyph: char) -> Creature {
        Creature {
            location: Point { x: x, y: y },
            glyph: glyph
        }
    }

    pub fn kobold(x: i16, y: i16) -> Creature {
        Creature::new(x, y, 'k')
    }

    pub fn to_string(&self) -> String {
        self.glyph.to_string()
    }
}


