use crate::{Card, Deck};

pub(crate) struct AI {
    pub(crate) deck: Deck,
    pub(crate) hand: Vec<Card>,
}

impl AI {
    pub fn odds(&self) -> f32 {
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

        let math = (count as f32) / (self.deck.cards.iter().count() as f32) * 100.0;
        // DEBUG
        // println!("Counted {}", count);
        // println!("Total {}", self.deck.cards.iter().count());
        // println!("Percent {}", math);
        return math;
    }
}
