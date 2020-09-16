use crate::deck::Card;

#[derive(Debug)]
pub struct Player {
    name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new<S: Into<String>>(name: S, hand: Vec<Card>) -> Self {
        Player {
            name: name.into(),
            hand,
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
        self.hand.sort();
    }

    pub fn play_card_from_hand(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }
}