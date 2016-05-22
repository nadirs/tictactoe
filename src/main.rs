extern crate tictactoe;

use tictactoe::ui::*;

fn main() {
    println!("Tic-tac-toe");
    let ui = Ui::default();
    println!("{}", ui.show_board());
}
