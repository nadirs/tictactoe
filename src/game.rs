use std::iter::*;
use std::vec::IntoIter;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveMark { X, O }

#[derive(Debug, PartialEq)]
pub enum Player { Computer, Human }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HorizontalPos { Left, Center, Right }

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VerticalPos { Top, Center, Bottom }

#[derive(Debug, PartialEq)]
pub struct Board {
    cells: HashMap<(HorizontalPos, VerticalPos), MoveMark>
}

impl Board {
    pub fn default() -> Board {
        Board {
            cells: HashMap::new()
        }
    }

    pub fn get_cell_at(&self, x: HorizontalPos, y: VerticalPos) -> Option<&MoveMark> {
        self.cells.get(&(x, y))
    }

    pub fn set_cell_at(&mut self, x: HorizontalPos, y: VerticalPos, mark: MoveMark) {
        if let None = self.get_cell_at(x, y) {
            self.cells.insert((x, y), mark);
        }
    }
}

pub struct Game {
    player_1: Player,
    player_2: Player,
    pub board: Board,
    turns: Cycle<IntoIter<MoveMark>>
}

type BoardCell = usize;

impl Game {
    pub fn new(p1: Player, p2: Player) -> Game {
        Game {
            player_1: p1,
            player_2: p2,
            board: Board::default(),
            turns: Game::turns()
        }
    }

    pub fn default() -> Game {
        Game::new(Player::Computer, Player::Computer)
    }

    fn turns() -> Cycle<IntoIter<MoveMark>> {
        vec![MoveMark::X, MoveMark::O].into_iter().cycle()
    }

    fn next_mark(&mut self) -> MoveMark {
        self.turns.next().unwrap()
    }

    fn fetch_move(&mut self) -> (BoardCell, MoveMark) {
        (0, self.next_mark())
    }
}

pub fn horizontal_positions() -> Vec<HorizontalPos> {
    vec![HorizontalPos::Left, HorizontalPos::Center, HorizontalPos::Right]
}

pub fn vertical_positions() -> Vec<VerticalPos> {
    vec![VerticalPos::Top, VerticalPos::Center, VerticalPos::Bottom]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_computer_vs_computer() {
        let game = Game::default();
        assert_eq!(game.player_1, Player::Computer);
        assert_eq!(game.player_2, Player::Computer);
    }

    #[test]
    fn can_specify_game_players() {
        {
            let game = Game::new(Player::Human, Player::Computer);
            assert_eq!(game.player_1, Player::Human);
            assert_eq!(game.player_2, Player::Computer);
        }
        {
            let game = Game::new(Player::Computer, Player::Human);
            assert_eq!(game.player_1, Player::Computer);
            assert_eq!(game.player_2, Player::Human);
        }
        {
            let game = Game::new(Player::Human, Player::Human);
            assert_eq!(game.player_1, Player::Human);
            assert_eq!(game.player_2, Player::Human);
        }
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
    fn all_board_cells_start_empty() {
        let game = Game::default();
        let board = &game.board;

        assert_eq!(board.get_cell_at(HorizontalPos::Left, VerticalPos::Top), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Left, VerticalPos::Center), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Left, VerticalPos::Bottom), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Center, VerticalPos::Top), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Center, VerticalPos::Center), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Center, VerticalPos::Bottom), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Right, VerticalPos::Top), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Right, VerticalPos::Center), None);
        assert_eq!(board.get_cell_at(HorizontalPos::Right, VerticalPos::Bottom), None);
    }

    #[test]
    fn can_mark_a_board_cell() {
        let game = Game::default();
        let mut board = game.board;
        let mark = MoveMark::X;
        board.set_cell_at(HorizontalPos::Center, VerticalPos::Center, mark);
        let cell = board.get_cell_at(HorizontalPos::Center, VerticalPos::Center);
        assert_eq!(cell, Some(&mark));
    }

    #[test]
    fn can_mark_two_board_cells_and_get_back_the_correct_marks() {
        let game = Game::default();
        let mut board = game.board;
        board.set_cell_at(HorizontalPos::Center, VerticalPos::Center, MoveMark::X);
        board.set_cell_at(HorizontalPos::Left, VerticalPos::Center, MoveMark::O);
        let first_cell = board.get_cell_at(HorizontalPos::Center, VerticalPos::Center);
        let second_cell = board.get_cell_at(HorizontalPos::Left, VerticalPos::Center);
        assert_eq!(first_cell, Some(&MoveMark::X));
        assert_eq!(second_cell, Some(&MoveMark::O));
    }

    #[test]
    fn can_mark_all_board_cells_and_get_back_the_correct_marks() {
        let game = Game::default();
        let mut board = game.board;
        board.set_cell_at(HorizontalPos::Left, VerticalPos::Top, MoveMark::X);
        board.set_cell_at(HorizontalPos::Left, VerticalPos::Center, MoveMark::O);
        board.set_cell_at(HorizontalPos::Left, VerticalPos::Bottom, MoveMark::X);
        board.set_cell_at(HorizontalPos::Center, VerticalPos::Top, MoveMark::X);
        board.set_cell_at(HorizontalPos::Center, VerticalPos::Center, MoveMark::O);
        board.set_cell_at(HorizontalPos::Center, VerticalPos::Bottom, MoveMark::X);
        board.set_cell_at(HorizontalPos::Right, VerticalPos::Top, MoveMark::O);
        board.set_cell_at(HorizontalPos::Right, VerticalPos::Center, MoveMark::X);
        board.set_cell_at(HorizontalPos::Right, VerticalPos::Bottom, MoveMark::O);

        assert_eq!(board.get_cell_at(HorizontalPos::Left, VerticalPos::Top), Some(&MoveMark::X));
        assert_eq!(board.get_cell_at(HorizontalPos::Left, VerticalPos::Center), Some(&MoveMark::O));
        assert_eq!(board.get_cell_at(HorizontalPos::Left, VerticalPos::Bottom), Some(&MoveMark::X));
        assert_eq!(board.get_cell_at(HorizontalPos::Center, VerticalPos::Top), Some(&MoveMark::X));
        assert_eq!(board.get_cell_at(HorizontalPos::Center, VerticalPos::Center), Some(&MoveMark::O));
        assert_eq!(board.get_cell_at(HorizontalPos::Center, VerticalPos::Bottom), Some(&MoveMark::X));
        assert_eq!(board.get_cell_at(HorizontalPos::Right, VerticalPos::Top), Some(&MoveMark::O));
        assert_eq!(board.get_cell_at(HorizontalPos::Right, VerticalPos::Center), Some(&MoveMark::X));
        assert_eq!(board.get_cell_at(HorizontalPos::Right, VerticalPos::Bottom), Some(&MoveMark::O));
    }

    #[test]
    fn cannot_overwrite_a_marked_cell() {
        let game = Game::default();
        let mut board = game.board;
        board.set_cell_at(HorizontalPos::Left, VerticalPos::Top, MoveMark::X);
        board.set_cell_at(HorizontalPos::Left, VerticalPos::Top, MoveMark::O);
        assert_eq!(board.get_cell_at(HorizontalPos::Left, VerticalPos::Top), Some(&MoveMark::X));
    }
}
