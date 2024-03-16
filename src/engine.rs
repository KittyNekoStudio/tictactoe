use std::io;
use rand::seq::SliceRandom;
use crate::def::{Board, Player};
use crate::gamestate::{self, print_board};
// Function that allows the choice of whether to play the engine or not
pub fn use_engine() -> bool {
    println!("Do you want to play against an engine. Type Y or N to choose.");
    let mut yes_or_no = String::new().to_lowercase();
    io::stdin().read_line(&mut yes_or_no).unwrap();
    let clean_yes_or_no: String = yes_or_no.trim().parse().unwrap();
    match clean_yes_or_no.as_str() {
        "y" => true,
        "n" => false,
        _ => use_engine()
    }
}
pub fn make_move(mut board: &mut Board, player: &Player) -> Board {
    return board.clone();
}
// Generates a random number
pub fn random_string_gen() -> &'static str {
    let vec = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let random_num = vec.choose(&mut rand::thread_rng());
    match random_num {
    Some(i) => *i,
    None => random_string_gen()        
    }
}
// A function that combines all functions so the engine runs
pub fn run_engine(mut board: &mut Board, player: &Player) {
    print_board(&board);
}