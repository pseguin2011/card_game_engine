use crate::deck::{Deck, DeckType};
use crate::error::CardGameError;
use crate::player::{Player};

/// This trait handles all game logic when implemented
pub trait GameBuilder {
    /// This function initializes the game state
    fn initialize_game() -> Result<Game, CardGameError>;
    /// This function is a delegate function that handles all player moves
    /// defined for the game
    /// 
    /// # Arguments
    /// `game` - The game state being manipulated for the move provided
    fn player_move(&mut self, game: &mut Game);
}

#[derive(Clone, Copy)]
pub enum DefaultPlayerMoves {
    Draw,
    Discard(usize),
}

impl GameBuilder for DefaultPlayerMoves {
    /// Initializes the game with a single deck with Jokers, 4 players and 10 cards each,
    /// and flips over the top card of the deck to the discard pile.
    fn initialize_game() -> Result<Game, CardGameError> {
        let mut deck = Deck::new(DeckType::WithJokers);
        let players = vec![
            Player::new("Player 1", deck.draw_cards(10)?),
            Player::new("Player 2", deck.draw_cards(10)?),
            Player::new("Player 3", deck.draw_cards(10)?),
            Player::new("Player 4", deck.draw_cards(10)?),
        ];

        if let Some(top_card) = deck.draw_card() {
            deck.discard_card(top_card);
        }
        
        Ok(Game {
            players,
            deck,
            // sets the first to play as Player 1
            turn: 0,
        })
    }

    /// Handles the player moves to drawing and discarding
    fn player_move(&mut self, game: &mut Game) {
        match self {
            DefaultPlayerMoves::Draw => {
                if let Some(card) = game.deck.draw_card() {
                    game.players[game.turn].add_card_to_hand(card);
                }
            },
            DefaultPlayerMoves::Discard(card_index) => {
                let card = game.players[game.turn].play_card_from_hand(*card_index);
                game.deck.discard_card(card);
            },
        }
    }
}

pub struct Game {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub turn: usize,
}

impl Game {
    /// Delegates the creation of a new game to a GameBuilder
    pub fn new<B: GameBuilder>() -> Result<Game, CardGameError> {
        B::initialize_game()
    }
    
    /// Delegates the current player's move to the provided GameBuilder implementor
    /// 
    /// # Argument
    /// `builder` - A GameBuilder implementor that manipulates the game based on the player move
    pub fn player_move<B: GameBuilder>(&mut self, mut builder: B) {
        builder.player_move(self);
    }

    /// Ends turn for the current player by incrementing the turn index
    pub fn end_turn(&mut self) {
        self.turn = (self.turn + 1) % self.players.len();
    }
}

#[test]
fn test_builder() -> Result<(), CardGameError> {
    let mut game = Game::new::<DefaultPlayerMoves>()?;

    game.player_move(DefaultPlayerMoves::Draw);
    assert_eq!(game.players[game.turn].hand.len(), 11);

    let first_card = game.players[game.turn].hand[0].clone();
    game.player_move(DefaultPlayerMoves::Discard(0));

    assert_ne!(first_card, game.players[game.turn].hand[0]);
    assert_eq!(Some(&first_card), game.deck.peek_top_discarded_card());
    assert_eq!(game.players[game.turn].hand.len(), 10);

    game.end_turn();

    assert_eq!(game.turn, 1);
    Ok(())

}
