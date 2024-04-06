use std::{io, mem};
use rand::seq::SliceRandom;
use crate::def::{Board, Player};
use crate::gamestate::{did_win, if_input_exsits, reset_board_state, switch_player_turn, update_board_state};
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
/// Converts board to vector
pub fn convert_to_vector(board: &Board) -> Vec<Vec<i32>> {
    // Creates the vector
    let mut vector = vec![vec![0; 3]; 3];
    // Variable needed for mem::replace
    let mut _got = 0;
    // If the move was made with X
    // Replace the the index with 1
    if board.row1.a == 1 {
    _got = mem::replace(&mut vector[0][0], 1);
    } 
    if board.row1.b == 1 {
    _got = mem::replace(&mut vector[0][1], 1);
    }
    if board.row1.c == 1 {
    _got = mem::replace(&mut vector[0][2], 1);
    }
    if board.row2.a == 1 {
    _got = mem::replace(&mut vector[1][0], 1);
    }
    if board.row2.b == 1 {
    _got = mem::replace(&mut vector[1][1], 1);
    }
    if board.row2.c == 1 {
    _got = mem::replace(&mut vector[1][2], 1);
    }
    if board.row3.a == 1 {
    _got = mem::replace(&mut vector[2][0], 1);
    }
    if board.row3.b == 1 {
    _got = mem::replace(&mut vector[2][1], 1);        
    }
    if board.row3.c == 1 {
    _got = mem::replace(&mut vector[2][2], 1);
    }
    // If the move was made with O
    // Replace the index with 2
    if board.row1.a == 2 {
    _got = mem::replace(&mut vector[0][0], 2);    
    } 
    if board.row1.b == 2 {
    _got = mem::replace(&mut vector[0][1], 2);
    }
    if board.row1.c == 2 {
    _got = mem::replace(&mut vector[0][2], 2);
    }
    if board.row2.a == 2 {
    _got = mem::replace(&mut vector[1][0], 2);        
    }
    if board.row2.b == 2 {
    _got = mem::replace(&mut vector[1][1], 2);
    }
    if board.row2.c == 2 {
    _got = mem::replace(&mut vector[1][2], 2);        
    }
    if board.row3.a == 2 {
    _got = mem::replace(&mut vector[2][0], 2);
    }
    if board.row3.b == 2 {
    _got = mem::replace(&mut vector[2][1], 2);        
    }
    if board.row3.c == 2 {
    _got = mem::replace(&mut vector[2][2], 2);        
    }
    return vector;
}
/// A function that returns the winning conditions
pub fn result(board: &Board, player: &Player) -> String {
    // Returns if either player wins
    if did_win(board, player) {
        return String::from("yes");
    } else if did_win(board, &switch_player_turn(player)) {
        return String::from("no");
    }
    // Holds the counter
    let mut counter = 0;
    // Converst board to vec to be iterated over
    let board_vec = convert_to_vector(board);
    for i in 0..3 {
        for j in 0..3 {
            // If a move was made
            if board_vec[i][j] != 0 {
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
pub fn minimax(board: &mut Board, engine_player: &Player, user_player: &Player, all_inputs: &mut Vec<String>, max: bool, depth: i32) -> i32 {
    // Returns the score depending on the winning condition
    let result = result(board, engine_player);
    if result != " " {
        if result == "draw" {
            return 0;
        } else if result == "yes" {
            return 100;
        } else if result == "no" {
            return -100;
        }
    }
    // Converts the board so be iterated over
    let board_vec = convert_to_vector(board);
    // If max it's engine turn
    if max {
        // Set a low score to be compared agianst
    let mut best = -100;
        // Iterate over the board
        for i in 0..3 {
            for j in 0..3 {
                if board_vec[i][j] == 0 {
                    // Calls find_move to find what move it is
                    let engine_move = find_move(i, j);
                    // Update the board
                    update_board_state(board, &engine_player, &engine_move.to_string());
                    // Calls minimax again to deepen the search
                    let best_score = minimax(board, &engine_player, &user_player, all_inputs, false, depth + 1);
                    // Reset the board after the search is finished
                    reset_board_state(board, &engine_move.to_string());
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
                if board_vec[i][j] == 0 {
                    let engine_move = find_move(i, j);
                    update_board_state(board, &user_player, &engine_move.to_string());
                    let best_score = minimax(board, &engine_player, &user_player, all_inputs, true, depth + 1);
                    reset_board_state(board, &engine_move.to_string());
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
            return 4;
        } else {
            return 7;
        }
    } else if i == 1 {
        if j == 0 {
            return 2;
        } else if j == 1 {
            return 5;
        } else {
            return 8;
        }
    } else if i == 2 {
        if j == 0 {
            return 3;
        } else if j == 1 {
            return 6;
        } else {
            return 9;
        }
    } else {
        return 0;
    }
}
/// A function that finds the best move
pub fn best_move(board: &mut Board, engine_player: &Player, user_player: &Player, all_inputs: &mut Vec<String>) -> i32 {
    // A low score to compare against
    let mut best_eval = -100;
    // Initialize a variable to hold the move
    let mut best_move = 0;
    // Convert the board to iterate over
    let board_vec = convert_to_vector(board);
    for i  in 0..3 {
        for j in 0..3  {
            // If the cell is empty
            if board_vec[i][j] == 0 {
            // Find the move
            let engine_move = find_move(i, j);
            // Update the board
            update_board_state(board, engine_player, &engine_move.to_string());
            // Get the evaluation of the move
            let eval_move = minimax(board, engine_player, user_player, all_inputs, false, 1);
            // Reset the board
            reset_board_state(board, &engine_move.to_string());
            // println!("Move: {}, Eval: {}", engine_move, eval_move);
                // If the moves has a higher score than the previous move
                if eval_move > best_eval {
                    // Use the new move
                    best_move = engine_move;
                    // Change the score to the new score
                    best_eval = eval_move;
            }
            }    
        }
    }
    // Return the best move
    return best_move.try_into().unwrap();
}
/// A function to get a random move
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
        return (board.clone(), all_inputs);
}
/// A function that combines all functions so the engine runs
pub fn run_engine(board: &mut Board, player: &Player, engine_player: &Player, user_player: &Player, difficulty: bool, all_inputs: &mut Vec<String>) -> (Board, Vec<String>)  {
    // Checks for random first because it runs different code than the other difficulties
    if difficulty {
        // Calls random_move
        return random_move(board, engine_player, all_inputs.to_vec());
    } else {
        // If it's the first move
        if all_inputs.len() == 0 {
            // Call random_first_move
            let engine_move = random_first_move();
            // Make the move
            update_board_state(board, player, &engine_move.to_string());
            all_inputs.push(engine_move.to_string());
            return (board.clone(), all_inputs.to_vec());
        } 
       // Let engine move equal the result of minimax
       let engine_move = best_move(board, engine_player, user_player, all_inputs);
       // Make the move
       update_board_state(board, player, &engine_move.to_string());
       all_inputs.push(engine_move.to_string());
       // Return board and all_inputs inside a tuple
       // You do this as all_inputs cannot be updated from here as it's not in scope
       // And you also need to return the board to continue on with the game
       // Then you update board and all_inputs by destructuring the result of run_engine
       return (board.clone(), all_inputs.to_vec());
    }
}