mod card;
mod dealer;
mod deck;
mod game;
mod player;
mod playerlist;
mod suit;
mod value;
mod turn;

use crate::card::Card;
use crate::deck::Deck;
use crate::game::GameHandler;
use crate::suit::Suit;
use crate::value::Value;

fn main() {
    let mut game = GameHandler::new();
    game.start();
    

    println!("{}", game.current_game.deck.cards[0]);
    println!("{:-<1$}", "", 50);
}
