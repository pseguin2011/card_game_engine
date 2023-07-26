use crate::models::{card::DeckType, deck::Deck};

/// This trait builds an initial game state
pub trait GameBuilder {
    /// Error type
    type E;
    /// Game State type
    type S: Clone;
    /// This function initializes the game with an initial state
    fn initialize_game() -> Result<Self::S, Self::E>;
}

pub struct DefaultBuilder;

impl GameBuilder for DefaultBuilder {
    type E = crate::error::DefaultCardGameError;
    type S = crate::state::GameState;

    fn initialize_game() -> Result<Self::S, Self::E> {
        let mut draw = Deck::new(DeckType::WithJokers);
        draw.extend(crate::models::deck::Deck::new(
            crate::models::card::DeckType::WithJokers,
        ));
        draw.shuffle();
        let mut players = vec![];
        for i in 0..4 {
            players.push(crate::models::player::Player {
                name: format!("Player {}", i),
                hand: draw.pop_n(10)?,
            });
        }

        let state = Self::S {
            draw,
            discard: Deck::new(crate::models::card::DeckType::Empty),
            players,
            turn: 0,
        };

        Ok(state)
    }
}
