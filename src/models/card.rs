#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CardValue {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Joker = 14,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CardSuit {
    Spades = 1,
    Clubs = 2,
    Hearts = 3,
    Diamonds = 4,
    Black = 5,
    Red = 6,
}

pub const CARDS: [CardValue; 13] = [
    CardValue::Ace,
    CardValue::Two,
    CardValue::Three,
    CardValue::Four,
    CardValue::Five,
    CardValue::Six,
    CardValue::Seven,
    CardValue::Eight,
    CardValue::Nine,
    CardValue::Ten,
    CardValue::Jack,
    CardValue::Queen,
    CardValue::King,
];

pub const SUITS: [CardSuit; 4] = [
    CardSuit::Spades,
    CardSuit::Clubs,
    CardSuit::Hearts,
    CardSuit::Diamonds,
];

pub enum DeckType {
    Normal,
    WithJokers,
    Empty,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}
