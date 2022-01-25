use std::fmt;

#[derive(Debug)]
enum Turn {
    Dealer,
    Player,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}s Turn", self)
    }
}

impl Turn {
    fn bot_turn(&self) {}
    fn player_turn(&self) {}
}
