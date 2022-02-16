use crate::member::Member;
use crate::playerlist::PlayerList;
use crate::Deck;

///GameHandler handles the game.
#[derive(Debug)]
pub struct GameHandler {
    pub(crate) current_game: Game,
}

impl GameHandler {
    pub(crate) fn new() -> GameHandler {
        return GameHandler {
            current_game: Game::new(),
        };
    }
    /// Starts a new game with new players.
    pub(crate) fn start(&mut self) {
        self.current_game.set_table();
        self.current_game.deal_cards();
        self.current_game.play_game();
        self.determine_winners();
        //stdin().read_line(&mut String::new());
        //self.end();
    }
    /// Cleans the "table".
    pub(crate) fn end(&mut self) {
        self.current_game.clean();
    }
    fn determine_winners(&self) {
        for member in &self.current_game.players.players {
            match member {
                Member::Dealer(dealer) => {
                    //TODO Check Dealer BUSTED
                }
                Member::Player(player) => {
                    // Compare player.score to dealer.player.score
                    if let Member::Dealer(dealer) = &self.current_game.players.players
                        [self.current_game.players.players.len() - 1]
                    {
                        if player.score > dealer.player.score {
                            println!(
                                "{} won against dealer with score {}",
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
    pub(crate) deck: Deck,
    pub(crate) players: PlayerList,
}

impl Game {
    pub(crate) fn clean(&mut self) {
        self.deck.cards.drain(0..self.deck.cards.len());
        self.players.empty();
        println!("Table successfully cleaned");
    }
}

impl Game {
    fn new() -> Game {
        return Game {
            deck: Deck::new(1),
            players: PlayerList::new(),
        };
    }

    /// "Sets the table", i.e. shuffles the deck, and resets the players.
    fn set_table(&mut self) {
        self.deck = Deck::new(1);
        self.deck.deck_shuffle();
        self.players = PlayerList::new();
    }

    ///
    pub(crate) fn deal_cards(&mut self) {
        for _i in 0..2 {
            for member in &mut self.players.players {
                match member {
                    Member::Dealer(dealer) => dealer
                        .player
                        .hit(&mut self.deck, dealer.player.hand.len() == 1),
                    Member::Player(player) => player.hit(&mut self.deck, false),
                }
            }
        }
    }

    /// Plays the game until and the end is reached.
    fn play_game(&mut self) {
        loop {
            if self.players.current_index > self.players.players.len() - 1 {
                break;
            }

            if let Member::Dealer(dealer) = &self.players.players[self.players.current_index] {
                if self.dealer_is_done() {
                    println!("{} score: {}", dealer.player.name, dealer.player.score);
                    break;
                }
            }

            self.players.turn(&mut self.deck);
        }
    }

    /// Returns whether or not the current dealer should continue playing according to game rules.
    fn dealer_is_done(&self) -> bool {
        return match self.players.current_member() {
            Member::Dealer(dealer) => dealer.player.score >= 17,
            Member::Player(_) => false,
        };
    }
}
