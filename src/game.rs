extern crate rustbox;

use std::cell::RefCell;
use std::rc::Rc;

use creature::Creature;
use point::Point;
use world::World;

pub struct Game {
    pub world: World,
    pub game_info: Rc<RefCell<GameInfo>>
}

impl Game {
    pub fn new() -> Game {
        let mut world = World::generate();

        let location = world.random_empty_location();
        let game_info = Rc::new(RefCell::new(GameInfo::new(location.clone())));

        let player = Box::new(Creature::player(location.x, location.y, game_info.clone()));
        world.creatures.push(player);

        let loc = world.random_empty_location();
        let kobold = Box::new(Creature::kobold(loc.x, loc.y, game_info.clone()));
        world.creatures.push(kobold);

        Game {
            world: world,
            game_info: game_info,
        }
    }
}

pub struct GameInfo {
    pub keypress: Option<rustbox::Key>,
    pub player_location: Point,
}

impl GameInfo {
    pub fn new(player_location: Point) -> GameInfo {
        GameInfo {
            keypress: None,
            player_location: player_location,
        }
    }
}
