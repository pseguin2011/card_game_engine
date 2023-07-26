use crate::models::deck::Deck;
use crate::models::player::Player;

#[derive(Clone)]
pub struct GameState {
    pub draw: Deck,
    pub discard: Deck,
    pub players: Vec<Player>,
    pub turn: usize,
}

impl GameState {
    pub fn next_turn(&mut self) {
        self.turn = (self.turn + 1) % self.players.len();
    }
}
