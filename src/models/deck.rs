use crate::error::DefaultCardGameError;

use super::card::{Card, CardSuit, CardValue, DeckType, CARDS, SUITS};
#[derive(Debug, Clone)]
pub struct Deck(pub Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        Self::new(DeckType::Empty)
    }
}

impl Deck {
    pub fn new(deck_type: DeckType) -> Deck {
        if let DeckType::Empty = deck_type {
            return Deck(Vec::new());
        }

        let mut deck: Vec<Card> = Vec::new();
        for suit in SUITS.to_vec() {
            for value in CARDS.to_vec() {
                deck.push(Card { value, suit });
            }
        }
        if let DeckType::WithJokers = deck_type {
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
        }
        Deck(deck)
    }

    /// Extends the deck with another deck
    pub fn extend(&mut self, new_deck: Deck) {
        self.0.extend(new_deck.0);
    }

    /// Shuffles the deck by making 1000 random swaps
    pub fn shuffle(&mut self) {
        for _ in 0..1000 {
            let index_a = rand::random::<usize>() % self.0.len();
            let index_b = rand::random::<usize>() % self.0.len();
            self.0.swap(index_a, index_b);
        }
    }

    /// Peeks the top of the deck
    pub fn peek(&self) -> Option<&Card> {
        if self.0.is_empty() {
            None
        } else {
            Some(&self.0[0])
        }
    }

    /// Draws a single card from the deck
    pub fn pop(&mut self) -> Option<Card> {
        self.0.pop()
    }

    /// Cycles through the amount of requested cards and returns the top `n` cards
    ///
    /// ## Arguments
    /// `amount` - the number of cards drawn
    ///
    /// ## Returns
    /// The `n` cards drawn from the deck
    pub fn pop_n(&mut self, amount: usize) -> Result<Vec<Card>, DefaultCardGameError> {
        if amount > self.0.len() {
            return Err(DefaultCardGameError::IncorrectCardNumberRequest);
        }
        Ok(self.0.split_off(self.0.len() - amount))
    }

    /// Inserts a card to the top of the deck
    pub fn push(&mut self, card: Card) {
        self.0.push(card)
    }

    pub fn take(mut self) -> Deck {
        let returned_pile = std::mem::take(&mut self);
        returned_pile
    }
}
