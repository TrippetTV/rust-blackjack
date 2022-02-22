use crate::ai;
use crate::dealer::Dealer;
use crate::player::Player;
use crate::Deck;
use ai::odds;
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
    pub fn turn(&mut self, ctx: &mut Deck) -> usize {
        // Return if hit or pass
        return match self {
            Member::Dealer(dealer) => {
                if dealer.player.score >= 17 {
                    dealer.player.pass(false)
                } else {
                    dealer.player.hit(ctx, false)
                }
            }
            // Unwraps the Enum and stores the value in player, also creating the instance of the AI
            Member::Player(player) => {
                // Check ai.odds() for further details
                let odds = odds(ctx, &player.hand).floor();

                // If the odds are in the players favor to not lose, hit
                if odds >= 50.0 {
                    println!("{}: The odds are in my favor! {}%", player.name, odds);
                    player.hit(ctx, false)
                }
                // otherwise pass
                else {
                    println!("{}: The odds are not in my favor... {}%", player.name, odds);
                    player.pass(false)
                }
            }
        };
    }
}
