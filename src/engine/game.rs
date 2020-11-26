pub use crate::builder::GameBuilder;

#[derive(Clone)]
pub struct Game<S> {
    pub state: S,
}

impl <S> Game <S> {
    /// Delegates the creation of a new game to a GameBuilder
    pub fn new_game<B: GameBuilder>() -> Result<B::S, B::E> {
        B::initialize_game()
    }
}