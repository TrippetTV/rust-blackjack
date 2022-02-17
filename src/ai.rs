use crate::{Card, Deck};

pub(crate) struct AI {
    pub(crate) deck: Deck,
    pub(crate) hand: Vec<Card>,
}

impl AI {
    /// Gets the odds of hitting a card in the deck without going over 21 depending on any given hand.
    pub fn odds(&self) -> f32 {
        // Gets the score sum of the current players hand
        let score = &self
            .hand
            .clone()
            .into_iter()
            .map(|x| x.value.get_number())
            .sum::<u8>();

        // Count each time a card in deck when drawn + score <= 21
        let count = self
            .deck
            .cards
            .iter()
            .filter(|c| c.value.get_number() + score <= 21)
            .count();

        // Gives the odds of hitting a card in the deck that doesn't go over 21
        let math = (count as f32) / (self.deck.cards.iter().count() as f32) * 100.0;

        // Return the calculated odds
        return math;
    }
}
