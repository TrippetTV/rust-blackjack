use crate::dealer::Dealer;
use crate::player::Player;
use crate::Deck;
use std::fmt;

#[derive(Debug)]
pub enum Member {
    Dealer(Dealer),
    Player(Player),
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}s Turn", self)
    }
}

impl Member {
    pub(crate) fn turn(&mut self, ctx: &mut Deck) -> () {
        match self {
            Member::Dealer(dealer) => {
                if dealer.player.score >= 17 {
                    dealer.player.pass()
                } else {
                    dealer.player.hit(ctx)
                }
            } // See if need to hit, hit until self.dealer.score >= 17
            Member::Player(player) => {} // Hit or Pass :)
        }
    }
}
