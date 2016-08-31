extern crate gredin;
extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use gredin::game::Game;

// const SCREEN_WIDTH: i16 = 80;
// const SCREEN_HEIGHT: i16 = 21;

fn draw_world(rustbox: &rustbox::RustBox, game: &Game) {
    for (y, row) in game.world.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            rustbox.print(x as usize, y as usize, rustbox::RB_BOLD, Color::White, Color::Black, tile.to_string());
        }
    }
}

fn draw_creatures(rustbox: &rustbox::RustBox, game: &Game) {
    for c in game.world.creatures.iter() {
        rustbox.print(c.location.x as usize, c.location.y as usize, rustbox::RB_BOLD, Color::Yellow, Color::Black, c.to_string().as_ref())
    }
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut game = Game::new();

    loop {
        rustbox.clear();

        draw_world(&rustbox, &game);
        draw_creatures(&rustbox, &game);

        // let playerloc = game_info.borrow().player_location;
        // let locstring = format!("Coords: [{} - {}]", playerloc.x, playerloc.y);
        // rustbox.print(20, 20, rustbox::RB_BOLD, Color::White, Color::Black, locstring.as_ref());

        rustbox.present();

        game.game_info.borrow_mut().keypress = None;

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    key => { game.game_info.borrow_mut().keypress = Some(key) }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }

        let mut new_creatures = Vec::new();
        for creature in game.world.creatures.iter() {
            let c = creature.tick(&game);
            new_creatures.push(Box::new(c));
        }
        game.world.creatures = new_creatures;

        // for creature in game.world.creatures.iter() {
        //     creature.tick(game);
        // }
    }
}
