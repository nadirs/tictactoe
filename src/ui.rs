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

    fn parse_game_players(&self, input: &str) -> Result<(Player, Player), String> {
        match input {
            "1" => Ok((Player::Human, Player::Computer)),
            "2" => Ok((Player::Human, Player::Human)),
            "3" => Ok((Player::Computer, Player::Computer)),
            _ => Err(["Unable to parse game players from string index:", input].join(" ")),
        }
    }

    pub fn show_board(&self) -> String {
        let rows: Vec<String> = vertical_positions().into_iter().map(|y| {
            self.show_row(y)
        }).collect();
        rows.join("\n-----\n")
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

    #[test]
    fn parse_valid_string_index_into_valid_game_players() {
        let ui = Ui::default();
        {
            let players = ui.parse_game_players("1").expect("Unable to parse game players from string index");
            assert_eq!(players, (Player::Human, Player::Computer));
        }
        {
            let players = ui.parse_game_players("2").expect("Unable to parse game players from string index");
            assert_eq!(players, (Player::Human, Player::Human));
        }
        {
            let players = ui.parse_game_players("3").expect("Unable to parse game players from string index");
            assert_eq!(players, (Player::Computer, Player::Computer));
        }
    }

    #[test]
    fn parse_invalid_string_index_into_err() {
        let ui = Ui::default();
        {
            let input = "foobar";
            let err = ui.parse_game_players(input);
            assert_eq!(err, Err("Unable to parse game players from string index: foobar".to_string()));
        }
        {
            let input = "";
            let err = ui.parse_game_players(input);
            assert_eq!(err, Err("Unable to parse game players from string index: ".to_string()));
        }
        {
            let input = "1 would have been 'fine' if I stopped at the first \"char\"";
            let err = ui.parse_game_players(input);
            assert_eq!(err, Err("Unable to parse game players from string index: 1 would have been 'fine' if I stopped at the first \"char\"".to_string()));
        }
    }
}
