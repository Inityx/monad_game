extern crate rand;
use std::io;

mod game;

fn main() {
    println!("Hello and welcome to the game of monad! Choose the number of players.");
    let num_players = game::read_uint_from_user();
    let mut game = match game::Game::new(num_players){
        Ok(g) => g,
        Err(message) => {
            println!("{}", message);
            return;
        },
    };

    for player in (0..num_players).into_iter().cycle() {
        let mut can_play = true;
        loop {
            println!("Do you want to 1: Draw, 2: Flip, 3: Trade, 4: Buy, or 5: Leap?");
            match game::read_uint_from_user() {
                1 => {
                    if can_play {
                        game.draw(player);
                        break;
                    }
                    println!("You already did something else this turn! You can't draw!");
                },
                2 => {
                    if can_play {
                        game.flip();
                        break;
                    }
                    println!("You already did something else this turn! You can't flip!");
                },
                3 => {
                    if let Err(message) = game.trade(player) {
                        println!("{}", message);
                    }
                    else {
                        can_play = false;
                    }
                },
                4 => {
                    if let Err(message) = game.buy(player) {
                        println!("{}", message);
                    }
                    else {
                        can_play = false;
                    }
                },
                5 => {
                    if let Err(message) = game.leap(player) {
                        println!("{}", message);
                    }
                    else {
                        can_play = false;
                    }
                },
                _ => println!("That's not a valid selection!"),
            }
        }
    }
}
