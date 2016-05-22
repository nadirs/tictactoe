pub use game::*;

pub struct Ui {
    game: Game
}

impl Ui {
    pub fn default() -> Ui {
        Ui {
            game: Game::default()
        }
    }

    fn show_cell(&self, x: HorizontalPos, y: VerticalPos) -> String {
        let cell = self.game.board.get_cell_at(x, y);
        match cell {
            None => " ".to_string(),
            Some(mark) => {
                format!("{:?}", mark)
            }
        }
    }

    fn show_row(&self, y: VerticalPos) -> String {
        let cells: Vec<String> = horizontal_positions().into_iter().map(|x| {
                self.show_cell(x, y)
            }).collect();
        cells.join("|")
    }

    pub fn show_board(&self) -> String {
        let rows: Vec<String> = vertical_positions().into_iter().map(|y| {
            self.show_row(y)
        }).collect();
        rows.join("\n-----\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_empty_cell() {
        let ui = Ui::default();
        let shown_cell = ui.show_cell(HorizontalPos::Left, VerticalPos::Top);
        assert_eq!(shown_cell, " ");
    }

    #[test]
    fn show_marked_cell() {
        let mut ui = Ui::default();
        {
            let ref mut board = ui.game.board;
            board.set_cell_at(HorizontalPos::Left, VerticalPos::Top, MoveMark::X);
            board.set_cell_at(HorizontalPos::Center, VerticalPos::Center, MoveMark::O);
        }
        let shown_x = ui.show_cell(HorizontalPos::Left, VerticalPos::Top);
        let shown_o = ui.show_cell(HorizontalPos::Center, VerticalPos::Center);
        assert_eq!(shown_x, "X");
        assert_eq!(shown_o, "O");
    }

    #[test]
    fn show_empty_row() {
        let ui = Ui::default();
        let shown_row = ui.show_row(VerticalPos::Top);
        assert_eq!(shown_row, " | | ");
    }

    #[test]
    fn show_marked_row() {
        let mut ui = Ui::default();
        {
            let ref mut board = ui.game.board;
            board.set_cell_at(HorizontalPos::Left, VerticalPos::Top, MoveMark::X);
        }
        let shown_row = ui.show_row(VerticalPos::Top);
        assert_eq!(shown_row, "X| | ");
    }

    #[test]
    fn show_empty_board() {
        let ui = Ui::default();
        let shown_board = ui.show_board();
        assert_eq!(shown_board, " | | \n-----\n | | \n-----\n | | ");
    }
}
