use crate::error::DefaultCardGameError;
use crate::game::DefaultGameState;

pub trait GameMove<State> {
    type E;
    /// This function is a delegate function that handles all player moves
    /// defined for the game
    /// 
    /// # Arguments
    /// `game` - The game state being manipulated for the move provided
    fn handle_move(&self, game: &mut State) -> Result<(), Self::E>;
}

#[derive(Clone, Copy)]
pub enum DefaultMove {
    Draw,
    Discard(usize),
}

impl GameMove<DefaultGameState> for DefaultMove {
    type E = DefaultCardGameError;
    /// Handles the player moves to drawing and discarding
    fn handle_move(&self, game: &mut DefaultGameState) -> Result<(), Self::E>{
        match self {
            Self::Draw => {
                if let Some(card) = game.deck.draw_card() {
                    game.players[game.turn].add_card_to_hand(card);
                } else {
                    return Err(DefaultCardGameError::DeckEmpty);
                }
            },
            Self::Discard(card_index) => {
                let card = game.players[game.turn].play_card_from_hand(*card_index);
                game.deck.discard_card(card);
            },
        }
        Ok(())
    }
}