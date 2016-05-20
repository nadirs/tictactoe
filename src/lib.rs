use std::iter::*;
use std::vec::IntoIter;

#[derive(Clone, Copy, Debug, PartialEq)]
enum MoveMark { X, O }

#[derive(Debug, PartialEq)]
enum Player { Computer, Human }

#[derive(Debug, PartialEq)]
enum HorizontalPos { Left, Center, Right }

#[derive(Debug, PartialEq)]
enum VerticalPos { Top, Center, Bottom }

#[derive(Debug, PartialEq)]
struct Board {
    cell: Option<MoveMark>
}

impl Board {
    pub fn default() -> Board {
        Board {
            cell: None
        }
    }

    pub fn get_cell_at(&self, x: HorizontalPos, y: VerticalPos) -> Option<MoveMark> {
        self.cell
    }

    pub fn set_cell_at(&mut self, x: HorizontalPos, y: VerticalPos, mark: MoveMark) {
        self.cell = Some(mark);
    }
}

struct Game {
    player_1: Player,
    player_2: Player,
    board: Board,
    turns: Cycle<IntoIter<MoveMark>>
}

type BoardCell = usize;

impl Game {
    pub fn default() -> Game {
        Game {
            player_1: Player::Computer,
            player_2: Player::Computer,
            board: Board::default(),
            turns: Game::turns()
        }
    }

    pub fn turns() -> Cycle<IntoIter<MoveMark>> {
        vec![MoveMark::X, MoveMark::O].into_iter().cycle()
    }

    pub fn next_mark(&mut self) -> MoveMark {
        self.turns.next().unwrap()
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

#[test]
fn a_board_cell_starts_empty() {
    let game = Game::default();
    let board = &game.board;
    let cell = board.get_cell_at(HorizontalPos::Left, VerticalPos::Top);
    assert_eq!(cell, None);
}

#[test]
fn can_mark_a_board_cell() {
    let game = Game::default();
    let mut board = game.board;
    let mark = MoveMark::X;
    board.set_cell_at(HorizontalPos::Center, VerticalPos::Center, mark);
    let cell = board.get_cell_at(HorizontalPos::Center, VerticalPos::Center);
    assert_eq!(cell, Some(mark));
}
