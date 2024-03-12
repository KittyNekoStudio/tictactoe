mod def;
mod gamestate;
use def::Board;

use crate::def::Row;

fn main() {
    let mut board = Board::new();
    board[0].push("row1", 'b', 1);
    board[0].row1 = Row{a: Box::new([0]), ..board[0].row1.clone()};
    println!("{:?}", board[0].row1);
}
