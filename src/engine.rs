use std::io;
use rand::seq::SliceRandom;
use crate::def::{Board, Player};
use crate::gamestate::{self, if_input_exsits, print_board, update_board_state};
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
    Some(i) => i,
    None => random_string_gen()        
    }
}
// A function that asks for the difficulty of the engine
pub fn choose_difficulty() -> &'static str {
    println!("Please choose the difficulty.");
    println!("They are: Easy, Medium, Hard, Very Hard, Random");
    let mut difficulty = String::new().to_lowercase();
    io::stdin().read_line(&mut difficulty).unwrap();
    let clean_difficulty: String = difficulty.trim().parse().unwrap();
    match clean_difficulty.as_str() {
        "easy" => "easy",
        "medium" => "medium",
        "hard" => "hard",
        "very hard" => "very hard",
        "random" => "random",
        _ => choose_difficulty()
    }
}
const MAX_DEPTH: u8 = 9;
pub fn search_move(board: Board, difficulty: &str) -> Board {
    return board;
}
pub fn random_move(board: &mut Board, player: &Player, mut all_inputs: Vec<String>) -> (Board, Vec<String>) {
    // Get a random num
        let mut random_num = random_string_gen();
        while if_input_exsits(&all_inputs, random_num.to_string()) {
            // If it has already been played
            // Call again
            random_num = random_string_gen();
        }
        // Update the board
        update_board_state(board, &player, &random_num.to_string());
        // Update all_inputs
        all_inputs.push(random_num.to_string());
        // Return board and all_inputs inside a tuple
        // You do this as all_inputs cannot be updated from here as it's not in scope
        // And you also need to return the board to continue on with the game
        // Then you update board and all_inputs by destructuring the result of run_engine
        return (board.clone(), all_inputs);
}
// A function that combines all functions so the engine runs
pub fn run_engine(board: &mut Board , player: &Player, difficulty: &str, mut all_inputs: Vec<String>) -> (Board, Vec<String>)  {
    // Checks for random first because it runs different code than the other difficulties
    if difficulty == "random" {
        // Calls random_move
        return random_move(board, player, all_inputs);
    }
    return (board.clone(), all_inputs);
    
}