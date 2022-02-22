mod ai;
mod card;
mod dealer;
mod deck;
mod game;
mod member;
mod player;
mod playerlist;
mod suit;
mod value;

use crate::card::Card;
use crate::deck::Deck;
use crate::game::GameHandler;
use crate::suit::Suit;
use crate::value::Value;
use std::{thread, time};

fn main() {
    // Flavor print
    println!("{:-<1$}", "", 50);
    // Creates the instance of the game
    let mut game = GameHandler::new();
    // Starts the game
    game.start();
    // Flavor print
    println!("{:-<1$}", "", 50);

    // So that console won't instant shutdown after running .exe
    thread::sleep(time::Duration::from_millis(10000));
}
