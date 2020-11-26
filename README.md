<p align="center">
  <h3 align="center">Card Game Engine</h3>

  <p align="center">
    A barebones card game engine that takes an initial state builder and a rule set to build a STATELESS full card game.
    Best used with web API based games, checkout <a href="https://github.com/pseguin2011/dame_de_pique/"/>Dame de Pique</a>
    <br />
    <a href="https://github.com/pseguin2011/card_game_engine/"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/pseguin2011/card_game_engine/">View Demo</a>
    ·
    <a href="https://github.com/pseguin2011/card_game_engine/issues">Report Bug</a>
    ·
    <a href="https://github.com/pseguin2011/card_game_engine/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

Here's a blank template to get started:
**To avoid retyping too much info. Do a search and replace with your text editor for the following:**
`github_username`, `repo_name`, `twitter_handle`, `email`, `Card Game Engine`, `project_description`


### Built With

* Rust [https://www.rust-lang.org/]()

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* Rust
  ```sh
  cargo build 
  ```

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/pseguin2011/card_game_engine.git
   ```
2. Install Rust
   ```sh
   cargo build
   ```



<!-- USAGE EXAMPLES -->
## Usage

Write an implementation of the `Game Builder`:

```sh
use card_game_engine::models::deck::{Deck, DeckType};
use card_game_engine::models::player::Player;
use card_game_engine::DefaultCardGameError;

#[derive(Clone)]
pub struct State {
    deck: Deck,
    players: Vec<Player>,
    turn: usize
}

pub struct MyAwesomeBuilder;

impl GameBuilder for MyAwesomeBuilder {
    type E = DefaultCardGameError;
    type S = State;

    fn initialize_game() -> Result<Self::S, Self::E> {
        let mut deck = Deck::new(DeckType::WithJokers);
        deck.shuffle();

        let mut players = vec![
            Player {
                name: format!("Player 1"),
                hand: deck.draw_cards(10)?,
            },
            Player {
                name: format!("Player 2"),
                hand: deck.draw_cards(10)?,
            },
        }

        let state = Self::S {
            deck,
            players,
            turn: 0,
        };

        Ok(state)
    }
}
```

Write an implementation of `GameRules`:
```sh
#[derive(Clone, Copy)]
pub enum MyAwesomeGameMoves {
    Draw,
    Discard(usize),
}

impl GameRules<State, DefaultCardGameError> for MyAwesomeGameMoves {
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

    fn is_game_over(state: &mut State) -> Result<bool, DefaultCardGameError> {
        for player in state.players {
            if player.hand.is_empty() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn is_round_over(state: &mut State) -> Result<bool, DefaultCardGameError> {
        for player in state.players {
            if player.hand.is_empty() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn end_turn(state: &mut State) {
        state.turn = (state.turn + 1) % state.players.len();
    }
}
```

Start using the `Game` struct to manipulate the game state in your API:
```sh
    type MyAwesomeGame = Game<MyAwesomeBuilder, MyAwesomeGameMoves>;
    let mut game_state = MyAwesomeGame::new_game()?;
    MyAwesomeGame::game_action(MyAwesomeGameMoves::Draw, &mut game_state)?;
    MyAwesomeGame::game_action(MyAwesomeGameMoves::Discard(0), &mut game_state)?;
    MyAwesomeGame::end_turn(&mut game_state);
```

<!-- ROADMAP -->
## Roadmap

See the [open issues](https://github.com/pseguin2011/card_game_engine/issues) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Pierre Seguin - pseguin2011@protonmail.com

Project Link: [https://github.com/pseguin2011/card_game_engine](https://github.com/pseguin2011/card_game_engine)


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/github_username/repo.svg?style=for-the-badge
[contributors-url]: https://github.com/github_username/repo/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/github_username/repo.svg?style=for-the-badge
[forks-url]: https://github.com/github_username/repo/network/members
[stars-shield]: https://img.shields.io/github/stars/github_username/repo.svg?style=for-the-badge
[stars-url]: https://github.com/github_username/repo/stargazers
[issues-shield]: https://img.shields.io/github/issues/github_username/repo.svg?style=for-the-badge
[issues-url]: https://github.com/github_username/repo/issues
[license-shield]: https://img.shields.io/github/license/github_username/repo.svg?style=for-the-badge
[license-url]: https://github.com/github_username/repo/blob/master/LICENSE.txt
[linkedin-url]: https://linkedin.com/in/pierreseg
