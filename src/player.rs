use crate::{Card, Deck};

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
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
    pub(crate) fn hit(&mut self, ctx: &mut Deck) {
        draw_card(ctx);
        // remove card from deck
        // fn add_card_to_player()
        // add card score to player score
        todo!()
    }
    pub(crate) fn pass(&self) {
        // skip to next turn
        todo!()
    }
}

fn draw_card(ctx: &mut Deck) {
    todo!()
}
