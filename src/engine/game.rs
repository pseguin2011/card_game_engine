/// This trait handles all game logic when implemented
pub trait GameBuilder {
    type E;
    type G;
    /// This function initializes the game with an initial state
    fn initialize_game() -> Result<Self::G, Self::E>;
}

#[derive(Clone)]
pub struct Game<S> {
    pub state: S,
}

impl <S> Game <S> {
    /// Delegates the creation of a new game to a GameBuilder
    pub fn new_game<B: GameBuilder>() -> Result<B::G, B::E> {
        B::initialize_game()
    }
}