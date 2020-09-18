use crate::deck::Card;

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
    pub fn play_card_from_hand(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }
}