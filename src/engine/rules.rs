use crate::error::DefaultCardGameError;
use crate::state::GameState;

type State = GameState;

/// A game rule implementation is the representation of how a game is played.
/// It manipulates a game state in the handle move and requires the implementation of how
/// a game is ended and a round is ended

pub trait GameRules<State: Clone, E> {
    /// This function is a delegate function that handles all player moves
    /// defined for the game
    ///
    /// # Arguments
    /// `game` - The game state being manipulated for the move provided
    fn handle_move(&self, game: &mut State) -> Result<(), E>;

    /// This function describes the end case of a game for the provided game state
    ///
    /// # Note
    /// The DefaultCardGameError enum includes a GameOver error that should be thrown if the assertion fails
    ///
    /// # Arguments
    /// `game` - The game state being manipulated for the move provided
    fn assert_game_over(game: &mut State) -> Result<(), E>;

    /// This function defines if a game round is over for the provided game state
    ///
    /// # Example
    /// A round could be over if a player hand is empty but the game could still be active
    ///
    /// # Note
    /// The DefaultCardGameError enum includes a RoundOver error that should be thrown if the assertion fails
    ///
    /// # Arguments
    /// `game` - The game state being manipulated for the move provided
    fn assert_round_over(game: &mut State) -> Result<(), E>;

    /// This function defines how a turn is ended for a provided game state
    ///
    /// # Arguments
    /// `game` - The game state being manipulated to end the current players turn
    fn end_turn(game: &mut State);
}

/// The move is generally represented as an enum
#[derive(Clone, Copy)]
pub enum DefaultMove {
    Draw,
    Discard(usize),
}

impl GameRules<State, DefaultCardGameError> for DefaultMove {
    /// Handles the player moves to drawing and discarding
    fn handle_move(&self, state: &mut State) -> Result<(), DefaultCardGameError> {
        match self {
            Self::Draw => {
                if let Some(card) = state.deck.draw_card() {
                    state.players[state.turn].add_card_to_hand(card);
                } else {
                    return Err(DefaultCardGameError::DeckEmpty);
                }
            }
            Self::Discard(card_index) => {
                let card = state.players[state.turn].play_card_from_hand(*card_index);
                state.deck.discard_card(card);
            }
        }
        Ok(())
    }

    fn assert_game_over(_state: &mut State) -> Result<(), DefaultCardGameError> {
        // Default game state has no end game case
        Ok(())
    }

    fn assert_round_over(_state: &mut State) -> Result<(), DefaultCardGameError> {
        // Default game state has no round over case
        Ok(())
    }

    fn end_turn(state: &mut State) {
        state.next_turn();
    }
}
