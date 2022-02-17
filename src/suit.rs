use std::fmt;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
/// Suit represents one of four types in a standard deck of cards.
pub(crate) enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

//Implements a way to display the suit
impl fmt::Display for Suit {
    /// format suit returns the string representation of the suit.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
