use std::iter::*;
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq)]
enum MoveMark { X, O }

#[derive(Debug, PartialEq)]
enum Player { Computer, Human }

#[derive(Debug, PartialEq)]
struct Board {
    cells: [Option<MoveMark>; 9]
}

impl Board {
    pub fn default() -> Board {
        Board {
            cells: [
                None, None, None,
                None, None, None,
                None, None, None
            ]
        }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    player_1: Player,
    player_2: Player,
    board: Board,
    current_turn: MoveMark
}

type BoardCell = usize;

impl Game {
    pub fn default() -> Game {
        Game {
            player_1: Player::Computer,
            player_2: Player::Computer,
            board: Board::default(),
            current_turn: MoveMark::X
        }
    }

    pub fn next_mark(&mut self) -> MoveMark {
        if self.current_turn == MoveMark::X {
            self.current_turn = MoveMark::O;
            MoveMark::X
        } else {
            self.current_turn = MoveMark::X;
            MoveMark::O
        }
    }

    pub fn fetch_move(&mut self) -> (BoardCell, MoveMark) {
        (0, self.next_mark())
    }
}

#[test]
fn it_works() {
    assert!(true)
}

#[test]
fn default_computer_vs_computer() {
    let game = Game::default();
    assert_eq!(game.player_1, Player::Computer);
    assert_eq!(game.player_2, Player::Computer);
}

#[test]
fn starter_is_cross() {
    let mut game = Game::default();
    let (_, first_mark) = game.fetch_move();
    assert_eq!(first_mark, MoveMark::X);
}

#[test]
fn second_is_nought() {
    let mut game = Game::default();
    let (_, _) = game.fetch_move();
    let (_, second_mark) = game.fetch_move();
    assert_eq!(second_mark, MoveMark::O);
}
