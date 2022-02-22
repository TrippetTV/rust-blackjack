use crate::member::Member;
use crate::playerlist::PlayerList;
use crate::Deck;
use std::{thread, time};

#[derive(Debug)]
///GameHandler handles the game.
pub struct GameHandler {
    pub current_game: Game,
}

impl GameHandler {
    pub fn new() -> GameHandler {
        return GameHandler {
            current_game: Game::new(),
        };
    }
    /// Starts a new game with new players.
    pub fn start(&mut self) {
        self.current_game.set_table();
        self.current_game.deal_cards();
        self.current_game.play_game();
        println!("{:-<1$}", "", 50);
        self.determine_winners();
        // There currently is no reason to end the game. This was implemented as a way to play a
        // second round in the terminal, but never had the time to develop the loops
        self.end();
    }
    /// Cleans the "table".
    pub fn end(&mut self) {
        self.current_game.clean();
    }
    /// Determines all winners through their scores.
    fn determine_winners(&self) {
        for member in &self.current_game.player_list.players {
            match member {
                // If dealer, determine if dealer busted, print that players won if true.
                Member::Dealer(dealer) => {
                    if dealer.player.score > 21 {
                        println!("{} busted, henceforth all players win", dealer.player.name);
                    }
                }
                // If player, determine if player won and print comparison
                Member::Player(player) => {
                    if let Member::Dealer(dealer) = &self.current_game.player_list.players
                        [self.current_game.player_list.players.len() - 1]
                    {
                        // If dealer busted skip this iteration and wait for the dealer to handle this case.
                        if dealer.player.busted {
                            continue;
                        }
                        // If player beat dealer or if dealer busted, print player winning with their score.
                        if (player.score > dealer.player.score && !player.busted)
                            || dealer.player.score > 21
                        {
                            println!(
                                "{} won against dealer with score {}",
                                player.name, player.score
                            )
                        }
                        // If player tied the dealer and didn't bust.
                        else if player.score == dealer.player.score && !player.busted {
                            println!(
                                "{} tied against dealer with score {}",
                                player.name, player.score
                            )
                        }
                        // If player busted.
                        else if player.busted {
                            println!("{} busted with score {}", player.name, player.score)
                        }
                        // Else the player just lost.
                        else {
                            println!(
                                "{} lost against dealer with score {}",
                                player.name, player.score
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Game is the players and the deck.
#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub player_list: PlayerList,
}

impl Game {
    pub fn clean(&mut self) {
        self.deck.cards.drain(0..self.deck.cards.len());
        self.player_list.empty();
        println!("Table successfully cleaned");
    }
}

impl Game {
    fn new() -> Game {
        return Game {
            deck: Deck::new(1),
            player_list: PlayerList::new(),
        };
    }

    /// "Sets the table", i.e. shuffles the deck, and resets the players.
    fn set_table(&mut self) {
        self.deck = Deck::new(10);
        self.deck.deck_shuffle();
        self.player_list = PlayerList::new();
    }

    /// Deal the starting cards for eah player, and hides dealers second card accordingly
    pub fn deal_cards(&mut self) {
        for _i in 0..2 {
            for member in &mut self.player_list.players {
                match member {
                    Member::Dealer(dealer) => dealer
                        .player
                        .hit(&mut self.deck, dealer.player.hand.len() == 1),
                    Member::Player(player) => player.hit(&mut self.deck, false),
                };
                // Dramatic effect
                println!("{:-<1$}", "", 50);
                thread::sleep(time::Duration::from_millis(1000));
            }
        }
    }

    /// Plays the game until and the end is reached.
    fn play_game(&mut self) {
        loop {
            if self.player_list.current_index > self.player_list.players.len() - 1 {
                break;
            }
            // Determines if the last member is a dealer and if dealer is done or not.
            if let Member::Dealer(dealer) =
                &self.player_list.players[self.player_list.current_index]
            {
                if self.dealer_is_done() {
                    println!("{} score: {}", dealer.player.name, dealer.player.score);
                    break;
                }
            }
            self.player_list.turn(&mut self.deck);
        }
    }

    /// Returns whether or not the current dealer should continue playing according to game rules.
    fn dealer_is_done(&self) -> bool {
        return match self.player_list.current_member() {
            Member::Dealer(dealer) => dealer.player.score >= 17,
            Member::Player(_) => false,
        };
    }
}
