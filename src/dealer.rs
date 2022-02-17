use crate::player::Player;

#[derive(Debug)]
/// Dealer wraps a new player
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
