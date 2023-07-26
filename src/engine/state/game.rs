use super::Team;
use crate::models::deck::Deck;
use crate::state::Player;

#[derive(Clone)]
pub struct Game<'a> {
    pub draw: Deck,
    pub discard: Deck,
    pub players: Vec<Player>,
    pub teams: Vec<Team<'a>>,
    pub turn: usize,
}

impl<'a> Game<'a> {
    pub fn next_turn(&mut self) {
        self.turn = (self.turn + 1) % self.players.len();
    }
}
