use crate::models::card::{Card, CardValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::Player;

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
pub struct Team<'a> {
    pub player_a: Arc<Mutex<&'a mut Player>>,
    pub player_b: Arc<Mutex<&'a mut Player>>,
    partner_points: HashMap<CardValue, Vec<Card>>,
}

impl<'a> PartnerPoints for Team<'a> {
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
