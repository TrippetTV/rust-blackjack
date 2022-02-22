use std::fmt;
use std::fmt::Debug;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
/// Value is an enum to represent the different card values in a deck.
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    /// Returns the number of any given enum, or an error if enum is not found.
    pub fn get_number(&self) -> u8 {
        match self {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
    }
}

impl fmt::Display for Value {
    /// format returns the string representation of the wenum.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
