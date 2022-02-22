use crate::suit;
use crate::value::Value;
use std::fmt;
use suit::Suit;

#[derive(Debug, Copy, Clone)]
/// Represents a value and a suit.
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}
// Displays the card nicely
impl fmt::Display for Card {
    /// fmt Card returns the string representation of the card.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}
