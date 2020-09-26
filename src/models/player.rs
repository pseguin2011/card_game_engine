use crate::models::deck::Card;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new<S: Into<String>>(name: S, mut hand: Vec<Card>) -> Self {
        hand.sort();
        Player {
            name: name.into(),
            hand,
        }
    }

    /// Adds the card to the player's hand and sorts the hand again
    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
        self.hand.sort();
    }

    /// Removes the card from the player's hand
    /// 
    /// # Returns
    /// The card being played
    pub fn play_card_from_hand(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }

    /// Takes hand from player and re-initializes a new hand
    /// 
    /// # Returns
    /// The full hand of the player
    pub fn take_hand(&mut self) -> Vec<Card> {
        let hand = std::mem::take(&mut self.hand);
        self.hand = Vec::new();
        hand
    }
}