use crate::{Card, Deck};

/// Gets the odds of hitting a card in the deck without going over 21 depending on any given hand.
pub fn odds(ctx: &mut Deck, hand: &Vec<Card>) -> f32 {
    // Gets the score sum of the current players hand
    let score = hand
        .clone()
        .into_iter()
        .map(|x| x.value.get_number())
        .sum::<u8>();

    // Count each time a card in deck when drawn + score <= 21
    let count = ctx
        .cards
        .iter()
        .filter(|c| c.value.get_number() + score <= 21)
        .count();

    // Gives the odds of hitting a card in the deck that doesn't go over 21
    let math = (count as f32) / (ctx.cards.iter().count() as f32) * 100.0;

    // Return the calculated odds
    return math;
}
