use crate::models::player::Player;
use crate::models::deck::Deck;

pub struct GameState {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub turn: usize,
}

impl GameState {
    pub fn next_turn(&mut self) {
        self.turn = (self.turn + 1) % self.players.len();
    }
}