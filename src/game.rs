use crate::dealer::Dealer;
use crate::player::Player;
use crate::playerlist::PlayerList;
use crate::Deck;

///GameHandler handles the game
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
    /// Starts a new game with new players
    pub(crate) fn start(&mut self) {
        self.current_game.set_table()
    }
    /// Cleans "the table"
    pub(crate) fn end(&self) {
        todo!("End Game not yet implemented")
    }
}
/// Game is the players and the deck
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
    /// "Sets the table", i.e. shuffles the deck, and resets the players
    fn set_table(&mut self) {
        self.deck = Deck::new();
        self.deck.deck_shuffle();
        self.players = PlayerList::new();
        self.players
            .add_player(Player::new("Jack Black the Dealer".to_string()))
    }
}
