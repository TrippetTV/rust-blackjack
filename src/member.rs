use crate::ai::AI;
use crate::dealer::Dealer;
use crate::player::Player;
use crate::Deck;
use std::fmt;

#[derive(Debug)]
/// Wrapper for Dealer and Player.4 
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
    /// Determines whether this member should hit or pass depending on self
    pub(crate) fn turn(&mut self, ctx: &mut Deck) -> usize {
        // Return if hit or pass
        return match self {
            Member::Dealer(dealer) => {
                if dealer.player.score >= 17 {
                    dealer.player.pass(false)
                } else {
                    dealer.player.hit(ctx, false)
                }
            }
            // TODO Comment
            Member::Player(player) => {
                let ai: AI = AI {
                    deck: ctx.clone(),
                    hand: (*player.hand.to_vec()).to_owned(),
                };

                let odds = ai.odds().floor();
                // TODO Comment
                if odds >= 50.0 {
                    println!("{}: The odds are in my favor! {}%", player.name, odds);
                    player.hit(ctx, false)
                }
                // TODO Comment
                else {
                    println!("{}: The odds are not in my favor... {}%", player.name, odds);
                    player.pass(false)
                }
            }
        };
    }
}
