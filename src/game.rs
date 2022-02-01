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
    }
    /// Cleans "the table".
    pub(crate) fn end(&self) {
        todo!("End Game not yet implemented")
    }
}

/// Game is the players and the deck.
#[derive(Debug)]
pub struct Game {
    pub(crate) deck: Deck,
    pub(crate) players: PlayerList,
}

impl Game {
    fn new() -> Game {
        return Game {
            deck: Deck::new(),
            players: PlayerList::new(),
        };
    }

    /// "Sets the table", i.e. shuffles the deck, and resets the players.
    fn set_table(&mut self) {
        self.deck = Deck::new();
        self.deck.deck_shuffle();
        self.players = PlayerList::new();
    }

    ///
    pub(crate) fn deal_cards(&mut self) {
        for i in 0..2 {
            for member in &mut self.players.players {
                match member {
                    Member::Dealer(dealer) => dealer.player.hit(&mut self.deck),
                    Member::Player(player) => player.hit(&mut self.deck),
                }
            }
        }
    }

    /// Plays the game until and the end is reached.
    fn play_game(&mut self) {
        loop {
            if let Member::Dealer(dealer) = self.players.current_member() {
                if self.dealer_is_done() {
                    println!("Dealer score: {}", dealer.player.score);
                    break;
                }
            }
            self.players.turn(&mut self.deck);
        }
    }
    fn dealer_is_done(&self) -> bool {
        return match self.players.current_member() {
            Member::Dealer(dealer) => {
                dealer.player.score >= 17
                    && self.players.current_index == self.players.players.len()
            }
            Member::Player(_) => false,
        };
    }
}
