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
        match self {
            Member::Dealer(dealer) => {
                write!(f, "{}'s Turn", dealer.player.name)
            }
            Member::Player(player) => {
                write!(
                    f,
                    "{}s Turn, Score: {},  Hand: {:?}",
                    player.name, player.score, player.hand
                )
            }
        }
    }
}

impl Member {
    pub(crate) fn turn(&mut self, ctx: &mut Deck) -> usize {
        match self {
            Member::Dealer(dealer) => {
                return if dealer.player.score >= 17 {
                    dealer.player.pass();
                    1
                } else {
                    dealer.player.hit(ctx);
                    0
                }
            }
            Member::Player(player) => {
                //TODO implement the AI to calculate the odds of winning and determine behaviour accordingly.
                return if player.score <= 15 {
                    player.hit(ctx);
                    0
                } else {
                    player.pass();
                    1
                };
            }
        }
    }
}
