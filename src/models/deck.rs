use crate::error::DefaultCardGameError;

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

const CARDS: [CardValue; 13] = [
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

const SUITS: [CardSuit; 4] = [
    CardSuit::Spades,
    CardSuit::Clubs,
    CardSuit::Hearts,
    CardSuit::Diamonds,
];

pub enum DeckType {
    Normal,
    WithJokers,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

#[derive(Debug, Clone)]
pub struct Deck {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn new(deck_type: DeckType) -> Deck {
        let mut deck: Vec<Card> = Vec::new();
        let discard_pile: Vec<Card> = Vec::new();
        for suit in SUITS.to_vec() {
            for value in CARDS.to_vec() {
                deck.push(Card { value, suit });
            }
        }

        if let DeckType::WithJokers = deck_type {
            deck.push(Card {
                value: CardValue::Joker,
                suit: CardSuit::Black,
            });
            deck.push(Card {
                value: CardValue::Joker,
                suit: CardSuit::Red,
            });
        }
        Deck { deck, discard_pile }
    }

    /// Extends the deck with another deck
    pub fn extend(&mut self, new_deck: Deck) {
        self.deck.extend(new_deck.deck);
    }

    /// Shuffles the deck by making 1000 random swaps
    pub fn shuffle(&mut self) {
        for _ in 0..1000 {
            let index_a = rand::random::<usize>() % self.deck.len();
            let index_b = rand::random::<usize>() % self.deck.len();
            self.deck.swap(index_a, index_b);
        }
    }

    /// Cycles through the amount of requested cards and returns the top `n` cards
    ///
    /// ## Arguments
    /// `amount` - the number of cards drawn
    ///
    /// ## Returns
    /// The `n` cards drawn from the deck
    pub fn draw_cards(&mut self, amount: usize) -> Result<Vec<Card>, DefaultCardGameError> {
        if amount > self.deck.len() {
            return Err(DefaultCardGameError::IncorrectCardNumberRequest);
        }
        Ok(self.deck.split_off(self.deck.len() - amount))
    }

    /// Draws a single card from the deck
    pub fn draw_card(&mut self) -> Option<Card> {
        self.deck.pop()
    }

    /// Adds a card to the discard pile
    pub fn discard_card(&mut self, card: Card) {
        self.discard_pile.push(card);
    }

    /// Returns a reference to the top card of the discard pile
    pub fn peek_top_discarded_card(&self) -> Option<&Card> {
        if self.discard_pile.is_empty() {
            None
        } else {
            Some(&self.discard_pile[self.discard_pile.len() - 1])
        }
    }
    pub fn pop_top_discarded_card(&mut self) -> Option<Card> {
        if self.discard_pile.is_empty() {
            None
        } else {
            Some(self.discard_pile.remove(self.discard_pile.len() - 1))
        }
    }

    pub fn take_discard_pile(&mut self) -> Vec<Card> {
        let returned_pile = std::mem::take(&mut self.discard_pile);
        self.discard_pile = Vec::new();
        returned_pile
    }
}
