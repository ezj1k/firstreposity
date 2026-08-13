mod player;
mod gun;
mod level;
mod utils;
mod game;

use level::Levels;
use game::Game;

fn main() {
    println!("Select level (1, 2, 3): ");

    let selected_level = loop {
        if let Some(l) = Levels::from_str(&utils::input()) {
            break l;
        }
        println!("Incorect! Select 1, 2 or 3:");
    };

    let mut game = Game::new(selected_level);
    
    game.start();
}