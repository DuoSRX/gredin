use rustbox::Key;

use std::cell::RefCell;
use std::rc::Rc;

use point::Point;
use world::GameInfo;
use world::World;

pub struct Creature {
    glyph: char,
    pub location: Point,
    pub is_player: bool,
    pub movement_component: Box<MovementComponent + 'static>,
}

impl Creature {
    pub fn new(x: i16, y: i16, glyph: char, is_player: bool, game_info: Rc<RefCell<GameInfo>>, movement_component: Box<MovementComponent + 'static>) -> Creature {
        Creature {
            location: Point { x: x, y: y },
            glyph: glyph,
            is_player: is_player,
            movement_component: movement_component
        }
    }

    pub fn player(x: i16, y: i16, game_info: Rc<RefCell<GameInfo>>) -> Creature {
        let mc = Box::new(PlayerMovementComponent{ game_info: game_info.clone() });
        Creature::new(x, y, '@', true, game_info, mc)
    }

    pub fn kobold(x: i16, y: i16, game_info: Rc<RefCell<GameInfo>>) -> Creature {
        let mc = Box::new(NoMovementComponent { game_info: game_info.clone() });
        Creature::new(x, y, 'k', false, game_info, mc)
    }

    pub fn to_string(&self) -> String {
        self.glyph.to_string()
    }

    // pub fn move_to(&mut self, world: &mut World, dest: Point) {
    //     // let tile = world.at(dest.x, dest.y);
    //     self.location = dest;

    //     // if tile.can_move_through() {
    //     // } else if tile.diggable() {
    //     //     world.dig(dest.x, dest.y);
    //     // }
    // }

    pub fn tick(&mut self, game_info: Rc<RefCell<GameInfo>>) {
        self.location = self.movement_component.tick(self.location);
    }
}

pub trait MovementComponent {
    fn tick(&self, Point) -> Point;
}

pub struct NoMovementComponent {
    game_info: Rc<RefCell<GameInfo>>
}

impl MovementComponent for NoMovementComponent {
    fn tick(&self, point: Point) -> Point {
        point
    }
}

pub struct PlayerMovementComponent {
    game_info: Rc<RefCell<GameInfo>>
}

impl MovementComponent for PlayerMovementComponent {
    fn tick(&self, point: Point) -> Point {
        let key = self.game_info.borrow().keypress;
        let mut location = point.clone();

        if let Some(key) = key {
            match key {
                Key::Left | Key::Char('h') => {
                    location = point.left()
                }
                Key::Right | Key::Char('l') => {
                    location = point.right()
                }
                Key::Up | Key::Char('k') => {
                    location = point.up()
                }
                Key::Down | Key::Char('j') => {
                    location = point.down()
                }
                _ => {}
            }
        };

        self.game_info.borrow_mut().player_location = location.clone();
        location
    }
}
