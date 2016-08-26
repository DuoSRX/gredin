extern crate gredin;
extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

use gredin::point::Point;
use gredin::world::World;

const SCREEN_WIDTH: i16 = 80;
const SCREEN_HEIGHT: i16 = 21;

struct Player {
    location: Point
}

// struct Game {
//     world: World,
//     player: Player
// }

impl Player {
    fn move_to(&mut self, world: &mut World, dest: Point) {
        let tile = world.at(dest.x, dest.y);

        if tile.can_move_through() {
            self.location = dest;
        } else if tile.diggable() {
            world.dig(dest.x, dest.y);
        }
    }
}

fn draw_world(rustbox: &rustbox::RustBox, world: &World) {
    for (y, row) in world.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            rustbox.print(x as usize, y as usize, rustbox::RB_BOLD, Color::White, Color::Black, tile.to_string());
        }
    }
}

fn draw_player(rustbox: &rustbox::RustBox, player: &Player) {
    rustbox.print(player.location.x as usize, player.location.y as usize, rustbox::RB_BOLD, Color::Red, Color::Black, "@");
}

fn draw_ui(rustbox: &rustbox::RustBox, player: &Player) {
    let loc = format!("Coords: [{} - {}]", player.location.x, player.location.y);
    rustbox.print(20, 20, rustbox::RB_BOLD, Color::White, Color::Black, loc.as_ref());
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let location = Point::new(2, 2);
    let mut player = Player { location: location };

    let mut world = World::generate();
    //let game = Game { world: world, player: player };

    loop {
        rustbox.clear();

        draw_world(&rustbox, &world);
        draw_player(&rustbox, &player);
        // draw_ui(&rustbox, &player);

        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Left | Key::Char('h') => {
                        let loc = player.location.left();
                        player.move_to(&mut world, loc);
                    }
                    Key::Right | Key::Char('l') => {
                        let loc = player.location.right();
                        player.move_to(&mut world, loc);
                    }
                    Key::Up | Key::Char('k') => {
                        let loc = player.location.up();
                        player.move_to(&mut world, loc);
                    }
                    Key::Down | Key::Char('j') => {
                        let loc = player.location.down();
                        player.move_to(&mut world, loc);
                    }
                    Key::Char('s') => { world.smooth() }
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
