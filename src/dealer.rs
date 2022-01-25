//TODO: A different version of player

use crate::Card;

pub struct Dealer {
    name: String,
    hand: Vec<Card>,
    score: u8,
}

impl Dealer {
    pub(crate) fn new(name: &str) -> Dealer {
        Dealer {
            name: name.to_string(),
            hand: vec![],
            score: 0,
        }
    }
}
