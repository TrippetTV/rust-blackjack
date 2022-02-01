use crate::{Card, Deck};
use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub(crate) name: String,
    pub(crate) hand: Vec<Card>,
    pub(crate) score: u8,
}

impl Player {
    pub(crate) fn new(name: String) -> Player {
        Player {
            name,
            hand: vec![],
            score: 0,
        }
    }

    /// Draws the player a card from the deck and into their hand.
    pub(crate) fn hit(&mut self, ctx: &mut Deck) {
        self.draw_card(ctx);
        println!("{} hit {}", self.name, self.hand[self.hand.len() - 1]);
    }

    ///
    pub(crate) fn pass(&self) {
        // skip to next turn

        println!("{} passed", self.name);
    }

    /// Draws the first card in the deck, then calls to remove it, then adds the score to current player.
    fn draw_card(&mut self, ctx: &mut Deck) {
        self.hand.push(ctx.cards[0]);
        Player::remove_card(ctx);
        self.score = self.count_score()
    }

    /// Removes the first card of the deck.
    fn remove_card(ctx: &mut Deck) {
        ctx.cards.drain(0..1);
    }

    /// Adds the score to current player.
    fn count_score(&mut self) -> u8 {
        let mut score = 0;
        for card in &self.hand {
            score += card.value.get_number()
        }
        return score;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
