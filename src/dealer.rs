//TODO: A different version of player

use crate::player::Player;

#[derive(Debug)]
pub struct Dealer {
    pub(crate) player: Player,
}

impl Dealer {
    pub(crate) fn new(name: &str) -> Dealer {
        Dealer {
            player: Player::new(name.to_string()),
        }
    }
}
