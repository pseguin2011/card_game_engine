use crate::error::DefaultCardGameError;
use crate::state::Game;

type State = Game<'static>;

/// A game status representation
#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Active,
    RoundOver,
    GameOver,
}

/// A game rule implementation is the representation of how a game is played.
/// It manipulates a game state in the handle move and requires the implementation of how
/// a game is ended and a round is ended

pub trait GameRules<State: Clone, E> {
    /// This function is a delegate function that handles all player moves
    /// defined for the game
    ///
    /// # Arguments
    /// `game` - The game state being manipulated for the move provided
    fn handle_move(&self, game: &mut State) -> Result<GameStatus, E>;

    /// This function describes the end case of a game for the provided game state
    ///
    /// # Note
    /// The DefaultCardGameError enum includes a GameOver error that should be thrown if the assertion fails
    ///
    /// # Arguments
    /// `game` - The game state being manipulated for the move provided
    fn is_game_over(game: &mut State) -> bool;

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
    fn is_round_over(game: &mut State) -> bool;

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
    fn handle_move(&self, state: &mut State) -> Result<GameStatus, DefaultCardGameError> {
        match self {
            Self::Draw => {
                if let Some(card) = state.draw.pop() {
                    state.players[state.turn].add_card_to_hand(card);
                } else {
                    return Err(DefaultCardGameError::DeckEmpty);
                }
            }
            Self::Discard(card_index) => {
                let card = state.players[state.turn].play_card_from_hand(*card_index);
                state.draw.push(card);
            }
        }
        Ok(GameStatus::Active)
    }

    /// The game is over
    fn is_game_over(state: &mut State) -> bool {
        for p in state.players.iter() {
            if p.hand.is_empty() {
                return true;
            }
        }
        false
    }

    /// The round is over when any player no longer has cards in their hand
    fn is_round_over(state: &mut State) -> bool {
        for p in state.players.iter() {
            if p.hand.is_empty() {
                return true;
            }
        }
        false
    }

    fn end_turn(state: &mut State) {
        state.next_turn();
    }
}

#[test]
fn test_status_assertions() {
    type TestGame =
        crate::engine::game::Game<crate::builder::DefaultBuilder, crate::rules::DefaultMove>;
    let mut game = TestGame::new_game().unwrap();
    assert!(!DefaultMove::is_game_over(&mut game));
    assert!(!DefaultMove::is_round_over(&mut game));
    game.players[0].hand.clear();
    assert!(DefaultMove::is_game_over(&mut game));
    assert!(DefaultMove::is_round_over(&mut game));
}
