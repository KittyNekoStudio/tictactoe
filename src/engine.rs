use std::io;
use rand::seq::SliceRandom;
use crate::def::Player;
use crate::gamestate::{did_win, if_input_exists, switch_player_turn, update_board_state};
/// Function that allows the choice of whether to play the engine or not
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
/// Generates a random number
pub fn random_string_gen() -> &'static str {
    let vec = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let random_num = vec.choose(&mut rand::thread_rng());
    match random_num {
    Some(i) => i,
    None => random_string_gen()        
    }
}
/// Generates the first move as all corner moves are the best
// If this wasn't here engine would play top right every time
pub fn random_first_move() -> &'static str {
    let vec = vec!["1", "3", "7", "9"];
    let random_num = vec.choose(&mut rand::thread_rng());
    match random_num {
    Some(i) => i,
    None => random_string_gen()        
    }
}
/// A function that asks for the type of engine
pub fn if_random() -> bool {
    println!("Play against a random engine.");
    println!("Yes(y) or No(n).");
    let mut difficulty = String::new().to_lowercase();
    io::stdin().read_line(&mut difficulty).unwrap();
    let clean_difficulty: String = difficulty.trim().parse().unwrap();
    match clean_difficulty.as_str() {
        "y" => true,
        "n" => false,
        _ => if_random()
    }
}
/// A function that returns the winning conditions
pub fn result(board: &[[u8; 3]; 3], player: &Player) -> String {
    // Returns if either player wins
    if did_win(board, player) {
        return String::from("yes");
    } else if did_win(board, &switch_player_turn(player)) {
        return String::from("no");
    }
    // Holds the counter
    let mut counter = 0;
    for i in 0..3 {
        for j in 0..3 {
            // If a move was made
            if board[i][j] != 0 {
                counter += 1
            }
        }
    }
    // If counter is 9 all moves have been made
    // Making this a draw
    if counter == 9 {
        return String::from("draw");
    }
        return String::from(" ");
    
}
/// A minimax program that searches for the best move
pub fn minimax(mut board: [[u8; 3]; 3], engine_player: &Player, user_player: &Player, all_inputs: &mut Vec<String>, max: bool, depth: i32) -> i32 {
    // Returns the score depending on the winning condition
    let result = result(&board, engine_player);
    if result != " " {
        if result == "draw" {
            return 0;
        } else if result == "yes" {
            return 100;
        } else if result == "no" {
            return -100;
        }
    }
    // If max it's engine turn
    if max {
        // Set a low score to be compared agianst
    let mut best = -100;
        // Iterate over the board
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    // Update the board
                    if engine_player == &Player::PlayerOne {
                        board[i][j] = 1;
                    } else {
                        board[i][j] = 2;
                    }
                    // Calls minimax again to deepen the search
                    let best_score = minimax(board, &engine_player, &user_player, all_inputs, false, depth + 1);
                    // Reset the board after the search is finished
                    board[i][j] = 0;
                    // Compares the search against the low number
                    // To find if the move is worth playing
                    if best_score > best {
                        // Make the score equal to the search
                        // To compare against future searches
                        best = best_score;
                    }
                }
            }
        }
        // Return the score minus the depth
        // This makes the engine choose the fastest way to win
        return best - depth;
        // Same as above but swaped to the user turn
    } else {
        let mut best = 100;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    if user_player == &Player::PlayerOne {
                        board[i][j] = 1;
                    } else {
                        board[i][j] = 2;
                    }
                    let best_score = minimax(board, &engine_player, &user_player, all_inputs, true, depth + 1);
                    board[i][j] = 0;
                    if best_score < best {
                        best = best_score;
                    }
                }
            }
        }
        return best - depth;
    }
    
}
/// A function that takes indexes and converts it into data that can be inputed into the board struct
pub fn find_move(i: usize, j: usize) -> i32 {
    // At index 0 of i
    if i == 0 {
        // And index 0 of j
        if j == 0 {
            // Return 1
            return 1;
        } else if j == 1 {
            return 2;
        } else {
            return 3;
        }
    } else if i == 1 {
        if j == 0 {
            return 4;
        } else if j == 1 {
            return 5;
        } else {
            return 6;
        }
    } else if i == 2 {
        if j == 0 {
            return 7;
        } else if j == 1 {
            return 8;
        } else {
            return 9;
        }
    } else {
        return 0;
    }
}
/// A function that finds the best move
pub fn best_move(mut board: [[u8; 3]; 3], engine_player: &Player, user_player: &Player, all_inputs: &mut Vec<String>) -> (i32, i32) {
    // A low score to compare against
    let mut best_eval = -100;
    // Initialize a variable to hold the move
    let mut best_move_i = 0;
    let mut best_move_j = 0;
    for i  in 0..3 {
        for j in 0..3  {
            // If the cell is empty
            if board[i][j] == 0 {
            // Update the board
            if engine_player == &Player::PlayerOne {
                board[i][j] = 1;
            } else {
                board[i][j] = 2;
            }
            // Get the evaluation of the move
            let eval_move = minimax(board, engine_player, user_player, all_inputs, false, 1);
            // Reset the board
            board[i][j] = 0;
           // println!("Eval: {}, Move_i: {}, Move_j: {}", eval_move, best_move_i, best_move_j);
                // If the moves has a higher score than the previous move
                if eval_move > best_eval {
                    // Use the new move
                    best_move_i = i;
                    best_move_j = j;
                    // Change the score to the new score
                    best_eval = eval_move;
            }
            }    
        }
    }
    // Return the best move
    return (best_move_i as i32, best_move_j as i32);
}
/// A function to get a random move
pub fn random_move(mut board: [[u8; 3]; 3], player: &Player, mut all_inputs: Vec<String>) -> ([[u8; 3]; 3], Vec<String>) {
    // Get a random num
        let mut random_num = random_string_gen();
        while if_input_exists(&all_inputs, random_num.to_string()) {
            // If it has already been played
            // Call again
            random_num = random_string_gen();
        }
        // Update the board
        update_board_state(&mut board, &player, &random_num.to_string());
        // Update all_inputs
        all_inputs.push(random_num.to_string());
        return (board.clone(), all_inputs);
}
/// A function that combines all functions so the engine runs
pub fn run_engine(mut board: [[u8; 3]; 3], player: &Player, engine_player: &Player, user_player: &Player, if_random: bool, all_inputs: &mut Vec<String>) -> ([[u8; 3]; 3], Vec<String>)  {
    // Checks for random first because it runs different code than the other difficulties
    if if_random {
        // Calls random_move
        return random_move(board, engine_player, all_inputs.to_vec());
    } else {
        // If it's the first move
        if all_inputs.len() == 0 {
            // Call random_first_move
            let engine_move = random_first_move();
            // Make the move
            update_board_state(&mut board, player, &engine_move.to_string());
            all_inputs.push(engine_move.to_string());
            return (board.clone(), all_inputs.to_vec());
        } 
       // Let engine move equal the result of minimax
       let engine_move = best_move(board, engine_player, user_player, all_inputs);
       let i = engine_move.0 as usize;
       let j = engine_move.1 as usize;
       // Make the move
       if engine_player == &Player::PlayerOne {
            board[i][j] = 1;
        } else {
            board[i][j] = 2;
        }
       all_inputs.push(find_move(i, j).to_string());
       // Return board and all_inputs inside a tuple
       // You do this as all_inputs cannot be updated from here as it's not in scope
       // And you also need to return the board to continue on with the game
       // Then you update board and all_inputs by destructuring the result of run_engine
       return (board.clone(), all_inputs.to_vec());
    }
}