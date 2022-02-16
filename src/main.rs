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

fn main() {
    let mut game = GameHandler::new();
    game.start();

    println!("{:-<1$}", "", 50);
}
