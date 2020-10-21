use crate::models::deck::{Deck, DeckType};
use crate::models::player::{Player};
use crate::error::{DefaultCardGameError};
use crate::moves::{GameMove};

/// This trait handles all game logic when implemented
pub trait GameBuilder {
    type E;
    type G;
    /// This function initializes the game with an initial state
    fn initialize_game() -> Result<Self::G, Self::E>;
}

pub struct DefaultGameBuilder;

impl GameBuilder for DefaultGameBuilder {
    type E = DefaultCardGameError;
    type G = Game<DefaultGameState>;
    /// Initializes the game with a single deck with Jokers, 2 players and 10 cards each,
    /// and flips over the top card of the deck to the discard pile.
    fn initialize_game() -> Result<Self::G, Self::E> {
        let mut deck = Deck::new(DeckType::WithJokers);
        let players = vec![
            Player::new("Player 1", deck.draw_cards(10)?),
            Player::new("Player 2", deck.draw_cards(10)?),
        ];

        if let Some(top_card) = deck.draw_card() {
            deck.discard_card(top_card);
        }
        
        let state = 
            DefaultGameState {
                players,
                deck,
                turn: 0,
        };

        Ok(Game { state })
    }
}

pub trait GameState {
    fn end_turn(&mut self);
}

#[derive(Clone)]
pub struct DefaultGameState {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub turn: usize,
}

impl GameState for DefaultGameState {
    fn end_turn(&mut self) {
        self.turn = (self.turn + 1) % self.players.len();
    }
}

pub trait GameRunner {
    type State: GameState;
    /// Delegates the current player's move to the provided GameBuilder implementor
    /// 
    /// # Argument
    /// `builder` - A GameBuilder implementor that manipulates the game based on the player move
    /// 
    /// # Returns
    /// Whether the player move succeeded
    fn player_move<M: GameMove<Self::State>>(&mut self, action: M) -> Result<(), M::E> {
        action.handle_move(self.get_game_state())
    }

    /// Ends turn for the current player by incrementing the turn index
    fn end_turn(&mut self) {
        Self::get_game_state(self).end_turn();
    }

    /// returns the current state of the game
    fn get_game_state(&mut self) -> &mut Self::State;
}

#[derive(Clone)]
pub struct Game<S: GameState> {
    pub state: S,
}

impl <S: GameState> Game <S> {
    /// Delegates the creation of a new game to a GameBuilder
    pub fn new_game<B: GameBuilder>() -> Result<B::G, B::E> {
        B::initialize_game()
    }
}

impl <S: GameState>GameRunner for Game<S> {
    type State = S;
    fn get_game_state(&mut self) -> &mut Self::State {
        &mut self.state
    }
}

#[test]
fn test_builder() -> Result<(), DefaultCardGameError> {
    let mut game = Game::<DefaultGameState>::new_game::<DefaultGameBuilder>()?;

    game.player_move(crate::moves::DefaultMove::Draw)?;
    assert_eq!(game.state.players[game.state.turn].hand.len(), 11);

    let first_card = game.state.players[game.state.turn].hand[0].clone();
    game.player_move(crate::moves::DefaultMove::Discard(0))?;

    assert_ne!(first_card, game.state.players[game.state.turn].hand[0]);
    assert_eq!(Some(&first_card), game.state.deck.peek_top_discarded_card());
    assert_eq!(game.state.players[game.state.turn].hand.len(), 10);

    game.end_turn();

    assert_eq!(game.state.turn, 1);
    Ok(())

}
