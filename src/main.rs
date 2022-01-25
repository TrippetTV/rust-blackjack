mod card;
mod dealer;
mod deck;
mod game;
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
    let game = GameHandler::new();
    

    println!("{}", game.current_game.deck.cards[0]);
    println!("{:-<1$}", "", 50);
}
