use crate::deck::{Deck, DeckType};
use crate::error::CardGameError;
use crate::player::{Player};


pub trait GameBuilder {
    fn initialize_game() -> Result<Game, CardGameError>;
    fn player_move(&mut self, game: &mut Game);
}

#[derive(Clone, Copy)]
pub enum DefaultPlayerMoves {
    Draw,
    Discard(usize),
}

impl GameBuilder for DefaultPlayerMoves {
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
            turn: 0,
        })
    }
    fn player_move(&mut self, game: &mut Game) {
        match self {
            DefaultPlayerMoves::Draw => {
                if let Some(card) = game.deck.draw_card() {
                    game.players[game.turn].add_card_to_hand(card);
                }
            },
            DefaultPlayerMoves::Discard(card_index) => {
                let card = game.players[game.turn].play_card_from_hand(*card_index);
                println!("{:?}",&card);
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
    pub fn new<B: GameBuilder>() -> Result<Game, CardGameError> {
        B::initialize_game()
    }
    
    pub fn player_move<B: GameBuilder>(&mut self, mut builder: B) {
        builder.player_move(self);
    }

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
