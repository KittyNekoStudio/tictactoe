mod def;
mod gamestate;
use def::Board;
use crate::def::Circles;
use crate::def::Player::PlayerTwo;

use crate::gamestate::update_board_state;

fn main() {
    let mut board = Board::new();
    let player = PlayerTwo(Circles);
    update_board_state(&mut board, player);
    println!("{:?}", board);
}
