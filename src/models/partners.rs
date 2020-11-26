use crate::models::deck::{Card, CardValue};
use crate::models::player::Player;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait PartnerPoints {
    fn add_points(&mut self, cards: Vec<Card>);
}

pub trait PartnerDiscardPile {
    fn add_to_discard(&mut self, card: Card);
    fn take_from_discard(&mut self, index: usize) -> Card;
}

pub trait PartnerSharedHand {
    fn get_shared_hand(&self) -> &[Card];
    fn take_from_shared_hand(&mut self, index: usize) -> Card;
    fn add_to_shared_hand(&mut self, card: Card);
}

#[derive(Clone)]
pub struct Partners<'a> {
    pub player_a: Arc<Mutex<&'a mut Player>>,
    pub player_b: Arc<Mutex<&'a mut Player>>,
    partner_points: HashMap<CardValue, Vec<Card>>,
    partner_discard: Vec<Card>,
    partner_shared_hand: Vec<Card>,
}

impl<'a> PartnerPoints for Partners<'a> {
    fn add_points(&mut self, cards: Vec<Card>) {
        for card in cards {
            match self.partner_points.get_mut(&card.value) {
                Some(v) => v.push(card),
                None => {
                    self.partner_points.insert(card.value, vec![card]);
                }
            }
        }
    }
}

impl<'a> PartnerDiscardPile for Partners<'a> {
    fn add_to_discard(&mut self, card: Card) {
        self.partner_discard.push(card);
    }

    fn take_from_discard(&mut self, index: usize) -> Card {
        self.partner_discard.remove(index)
    }
}

impl<'a> PartnerSharedHand for Partners<'a> {
    fn get_shared_hand(&self) -> &[Card] {
        &self.partner_shared_hand
    }

    fn take_from_shared_hand(&mut self, index: usize) -> Card {
        self.partner_shared_hand.remove(index)
    }

    fn add_to_shared_hand(&mut self, card: Card) {
        self.partner_shared_hand.push(card);
    }
}
