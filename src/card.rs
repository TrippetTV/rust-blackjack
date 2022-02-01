use crate::suit;
use crate::value::Value;
use std::fmt;
use suit::Suit;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Card {
    pub(crate) value: Value,
    pub(crate) suit: Suit,
}
// Displays the card nicely
impl fmt::Display for Card {
    /// fmt Card returns the string representation of the card.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

impl Card {
    /// Create a new Card with the given parameters.
    pub(crate) fn create(mut value: Value, suit: Suit) -> Card {
        value = check_value(value);
        return Card { value, suit };
    }
}

///
///
/// # Arguments
///
/// * `value`: The value to check against.
///
/// returns: Value if it is valid.
///
/// # Examples
///
/// ```
/// let card = Card::create(Value::Eight, Suit::Hearts);
/// ```
fn check_value(value: Value) -> Value {
    if value.get_number() > 13 {
        panic!(
            "Value must be between 0 and 13 but got {}",
            value.get_number()
        )
    }
    return value;
}
