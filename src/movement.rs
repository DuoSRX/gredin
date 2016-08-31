use rustbox::Key;

use std::cell::RefCell;
use std::rc::Rc;

use point::Point;
use game::GameInfo;
use world::World;

pub trait MovementComponent {
    fn tick(&self, point: Point, _world: &World) -> Point { point }
    fn box_clone(&self) -> Box<MovementComponent>;
}

#[derive(Clone)]
pub struct RandomMovementComponent {}

impl MovementComponent for RandomMovementComponent {
    fn box_clone(&self) -> Box<MovementComponent> {
        Box::new(RandomMovementComponent {})
    }

    fn tick(&self, point: Point, world: &World) -> Point {
        world.random_free_square(point.x, point.y)
    }
}

#[derive(Clone)]
pub struct PlayerMovementComponent {
    pub game_info: Rc<RefCell<GameInfo>>
}

impl MovementComponent for PlayerMovementComponent {
    fn box_clone(&self) -> Box<MovementComponent> {
        Box::new(PlayerMovementComponent { game_info: self.game_info.clone() })
    }

    fn tick(&self, point: Point, world: &World) -> Point {
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
