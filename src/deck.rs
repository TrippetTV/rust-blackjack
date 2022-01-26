use crate::{Card, Suit, Value};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
    pub(crate) cards: Vec<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.cards)
    }
}

impl Deck {
    /// Create a new Deck with a full list of sorted cards.
    pub(crate) fn new() -> Deck {
        let mut card_list: Vec<Card> = vec![];
        // Loop through each Suit and Value and add them to the deck
        for suit in Suit::iter() {
            for value in Value::iter() {
                card_list.push(Card { value, suit });
            }
        }

        return Deck { cards: card_list };
    }
    /// Shuffle the cards of called deck using a crate called SliceRandom
    pub(crate) fn deck_shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
