use std::cell::RefCell;
use std::rc::Rc;

use movement::{MovementComponent, RandomMovementComponent, PlayerMovementComponent};
use point::Point;
use game::{Game, GameInfo};

pub struct Creature {
    glyph: char,
    pub location: Point,
    pub is_player: bool,
    pub movement_component: Box<MovementComponent + 'static>,
}

impl Creature {
    pub fn new(x: i16, y: i16, glyph: char, is_player: bool, movement_component: Box<MovementComponent + 'static>) -> Creature {
        Creature {
            location: Point { x: x, y: y },
            glyph: glyph,
            is_player: is_player,
            movement_component: movement_component
        }
    }

    pub fn player(x: i16, y: i16, game_info: Rc<RefCell<GameInfo>>) -> Creature {
        let mc = Box::new(PlayerMovementComponent{ game_info: game_info.clone() });
        Creature::new(x, y, '@', true, mc)
    }

    pub fn kobold(x: i16, y: i16, _game_info: Rc<RefCell<GameInfo>>) -> Creature {
        let mc = Box::new(RandomMovementComponent {});
        Creature::new(x, y, 'k', false, mc)
    }

    pub fn to_string(&self) -> String {
        self.glyph.to_string()
    }

    pub fn tick(&self, game: &Game) -> Creature {
        let location = self.movement_component.tick(self.location, &game.world);
        Creature { location: location, .. self.clone() }
    }
}

impl Clone for Creature {
    fn clone(&self) -> Creature {
        Creature {
            location: self.location,
            glyph: self.glyph,
            is_player: self.is_player,
            movement_component: self.movement_component.box_clone(),
        }
    }
}

