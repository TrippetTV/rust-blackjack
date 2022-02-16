use crate::ai::AI;
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
                write!(
                    f,
                    "{}'s Turn, Score: {}",
                    dealer.player.name, dealer.player.score
                )
            }
            Member::Player(player) => {
                write!(f, "{}s Turn, Score: {}", player.name, player.score)
            }
        }
    }
}

impl Member {
    /// Turn behaviour
    pub(crate) fn turn(&mut self, ctx: &mut Deck) -> usize {
        // Return if hit or pass
        return match self {
            Member::Dealer(dealer) => {
                if dealer.player.score >= 17 {
                    dealer.player.pass(false);
                    1
                } else {
                    dealer.player.hit(ctx, false);
                    0
                }
            }
            Member::Player(player) => {
                let ai: AI = AI {
                    deck: ctx.clone(),
                    hand: (*player.hand.to_vec()).to_owned(),
                };

                let odds = ai.odds().floor();
                if odds >= 50.0 {
                    println!("{}: The odds are in my favor! {}%", player.name, odds);
                    player.hit(ctx, false);
                    0
                } else {
                    player.pass(false);
                    1
                }
            }
        };
    }
}
