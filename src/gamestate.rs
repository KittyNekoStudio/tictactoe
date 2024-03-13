use crate::def::{self, Board, Circles, Player, Player::PlayerOne, Xs};
use std::io;
// Print the board to the console
pub fn print_board() {
    println!("")
}
// Swap players
fn switch_player_turn(player: Player) -> Player {
    match player {
        Player::PlayerOne(Xs) => Player::PlayerTwo(Circles),
        Player::PlayerTwo(Circles) => Player::PlayerOne(Xs),
    }
}
// Recive player input
pub fn recive_input() -> String {
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).unwrap();
    let player_input_clean: String = player_input.trim().parse().unwrap();
    return player_input_clean;
}
// Pushes 1 or 0 to the cell corresponding to the player type
pub fn update_board_state(board: &mut Board, player: Player) {
    if player == PlayerOne(Xs) {
        match recive_input().as_str() {
            "a1" => board.row1.a.push(1),
            "a2" => board.row2.a.push(1),
            "a3" => board.row3.a.push(1),
            "b1" => board.row1.b.push(1),
            "b2" => board.row2.b.push(1),
            "b3" => board.row3.b.push(1),
            "c1" => board.row1.c.push(1),
            "c2" => board.row2.c.push(1),
            "c3" => board.row3.c.push(1),
            _ => ()
        };
    } else {
        match recive_input().as_str() {
            "a1" => board.row1.a.push(0),
            "a2" => board.row2.a.push(0),
            "a3" => board.row3.a.push(0),
            "b1" => board.row1.b.push(0),
            "b2" => board.row2.b.push(0),
            "b3" => board.row3.b.push(0),
            "c1" => board.row1.c.push(0),
            "c2" => board.row2.c.push(0),
            "c3" => board.row3.c.push(0),
            _ => ()
        };
    }}
// Function that combines all functions to make the program run
pub fn play_game() {
        
    }
