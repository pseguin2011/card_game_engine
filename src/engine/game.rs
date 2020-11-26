use std::marker::PhantomData;

use crate::engine::builder::GameBuilder;
use crate::engine::rules::GameRules;

/// A game is the `modifier` of game states based on the rules and state builder provided
/// It requires the state used in the rules to be the same type as the one in the builder.
#[derive(Clone)]
pub struct Game<B: GameBuilder, R: GameRules<B::S, B::E>> {
    /// We require a builder to be specified statically
    phantom_builder: PhantomData<B>,
    /// We require a rule set to be specified statically
    phantom_rules: PhantomData<R>
}

impl <B: GameBuilder, R: GameRules<B::S, B::E>> Game<B, R> {
    /// Initializes and returns a state for a new game as specified by the ruleset
    pub fn new_game() -> Result<B::S, B::E> {
        B::initialize_game()
    }

    /// Applies the provided action to the state granted that no end case has occured
    /// 
    /// # Arguments
    /// `action` - The action taken by the ruleset (usually a game move)
    /// `state` - The state being modified by the action
    /// 
    /// # Failure
    /// An error will be returned that the action cannot be executed if:
    ///     * The game state is game over or round over
    ///     * An error occurred when applying the action to the state
    pub fn game_action(action: R, state: &mut B::S) -> Result<(), B::E>{
        R::is_game_over(state)?;
        R::is_round_over(state)?;
        R::handle_move(&action, state)?;
        Ok(())
    } 

    /// Ends turn for the current state as specified by the ruleset
    /// 
    /// # Arguments
    /// `state` - The state being modified by the action
    pub fn end_turn(state: &mut B::S) {
        R::end_turn(state);
    }
}



#[test]
fn test_builder() -> Result<(), crate::error::DefaultCardGameError> {
    type TestGame = Game::<crate::builder::DefaultBuilder, crate::rules::DefaultMove>;
    let mut game_state = TestGame::new_game()?;

    TestGame::game_action(crate::rules::DefaultMove::Draw, &mut game_state)?;
    assert_eq!(game_state.players[game_state.turn].hand.len(), 11);

    let first_card = game_state.players[game_state.turn].hand[0].clone();
    TestGame::game_action(crate::rules::DefaultMove::Discard(0), &mut game_state)?;

    assert_ne!(first_card, game_state.players[game_state.turn].hand[0]);
    assert_eq!(Some(&first_card), game_state.deck.peek_top_discarded_card());
    assert_eq!(game_state.players[game_state.turn].hand.len(), 10);

    TestGame::end_turn(&mut game_state);

    assert_eq!(game_state.turn, 1);
    Ok(())

}
