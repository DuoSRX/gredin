extern crate gredin;
extern crate rustbox;

use std::error::Error;
use std::default::Default;
use std::cell::RefCell;
use std::rc::Rc;

use rustbox::{Color, RustBox};
use rustbox::Key;

use gredin::creature::Creature;
use gredin::point::Point;
use gredin::world::GameInfo;
use gredin::world::World;

// const SCREEN_WIDTH: i16 = 80;
// const SCREEN_HEIGHT: i16 = 21;

// struct Game {
//     world: World,
//     player: Player
// }

fn draw_world(rustbox: &rustbox::RustBox, world: &World) {
    for (y, row) in world.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            rustbox.print(x as usize, y as usize, rustbox::RB_BOLD, Color::White, Color::Black, tile.to_string());
        }
    }
}

fn draw_creatures(rustbox: &rustbox::RustBox, world: &World) {
    for c in world.creatures.iter() {
        rustbox.print(c.location.x as usize, c.location.y as usize, rustbox::RB_BOLD, Color::Yellow, Color::Black, c.to_string().as_ref())
    }
}

// fn draw_ui(rustbox: &rustbox::RustBox, player: &Player) {
// }

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut world = World::generate();

    let location = world.random_empty_location();
    let game_info = Rc::new(RefCell::new(GameInfo::new(location.clone())));

    let mut player = Box::new(Creature::player(location.x, location.y, game_info.clone()));
    world.creatures.push(player);

    let loc = world.random_empty_location();
    let kobold = Box::new(Creature::kobold(loc.x, loc.y, game_info.clone()));
    world.creatures.push(kobold);

    loop {
        rustbox.clear();

        draw_world(&rustbox, &world);
        draw_creatures(&rustbox, &world);
        // draw_ui(&rustbox, &player);

        let playerloc = game_info.borrow().player_location;
        let locstring = format!("Coords: [{} - {}]", playerloc.x, playerloc.y);
        rustbox.print(20, 20, rustbox::RB_BOLD, Color::White, Color::Black, locstring.as_ref());

        rustbox.present();

        game_info.borrow_mut().keypress = None;

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    key => { game_info.borrow_mut().keypress = Some(key) }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }

        for creature in world.creatures.iter_mut() {
            creature.tick(game_info.clone());
        }
    }
}
