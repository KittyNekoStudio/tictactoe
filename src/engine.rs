use std::{io, mem};
use rand::seq::SliceRandom;
use crate::def::{Board, Player};
use crate::gamestate::{did_win, if_input_exsits, reset_board_state, update_board_state};
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
    println!("Play against a random engine.");
    println!("Yes(y) or No(n).");
    let mut difficulty = String::new().to_lowercase();
    io::stdin().read_line(&mut difficulty).unwrap();
    let clean_difficulty: String = difficulty.trim().parse().unwrap();
    match clean_difficulty.as_str() {
        "y" => "y",
        "n" => "n",
        _ => choose_difficulty()
    }
}
pub fn moves_left(all_inputs: &Vec<String>) -> Vec<i32> {
    let mut moves_left: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut i = 0;
    while i < all_inputs.len() {
        let mut j = 0;
        while j < moves_left.len() {
            if all_inputs[i] == moves_left[j].to_string() {
                moves_left.remove(j);
            }
            j += 1;
        }
        i += 1;
    }
    return moves_left;
}
pub fn eval(board: &Board, engine_player: &Player, user_player: &Player, all_inputs: &mut Vec<String>) -> i32 {
    if did_win(board, &engine_player) {
        return 1;
    } else if did_win(board, &user_player) {
        return -1;
    } else if moves_left(&all_inputs).len() == 0 {
        return 0;
    } else {
        return 10;
    }
}
pub fn is_max(player: &Player) -> bool {
    if player == &Player::PlayerOne {
        return true;
    } else {
        return false;
    }
}
pub fn convert_to_vector(board: &Board) -> Vec<Vec<i32>> {
    let mut vector = vec![vec![10; 3]; 3];
    let mut _got = 0;
    if board.row1.a == vec![1] {
    _got = mem::replace(&mut vector[0][0], 1);
    } 
    if board.row1.b == vec![1] {
    _got = mem::replace(&mut vector[0][1], 1);
    }
    if board.row1.c == vec![1] {
    _got = mem::replace(&mut vector[0][2], 1);
    }
    if board.row2.a == vec![1] {
    _got = mem::replace(&mut vector[1][0], 1);
    }
    if board.row2.b == vec![1] {
    _got = mem::replace(&mut vector[1][1], 1);
    }
    if board.row2.c == vec![1] {
    _got = mem::replace(&mut vector[1][2], 1);
    }
    if board.row3.a == vec![1] {
    _got = mem::replace(&mut vector[2][0], 1);
    }
    if board.row3.b == vec![1] {
    _got = mem::replace(&mut vector[2][1], 1);        
    }
    if board.row3.c == vec![1] {
    _got = mem::replace(&mut vector[2][2], 1);
    }
    if board.row1.a == vec![0] {
    _got = mem::replace(&mut vector[0][0], 0);    
    } 
    if board.row1.b == vec![0] {
    _got = mem::replace(&mut vector[0][1], 0);
    }
    if board.row1.c == vec![0] {
    _got = mem::replace(&mut vector[0][2], 0);
    }
    if board.row2.a == vec![0] {
    _got = mem::replace(&mut vector[1][0], 0);        
    }
    if board.row2.b == vec![0] {
    _got = mem::replace(&mut vector[1][1], 0);
    }
    if board.row2.c == vec![0] {
    _got = mem::replace(&mut vector[1][2], 0);        
    }
    if board.row3.a == vec![0] {
    _got = mem::replace(&mut vector[2][0], 0);
    }
    if board.row3.b == vec![0] {
    _got = mem::replace(&mut vector[2][1], 0);        
    }
    if board.row3.c == vec![0] {
    _got = mem::replace(&mut vector[2][2], 0);        
    }
    return vector;
}
pub fn minimax(board: &mut Board, engine_player: &Player, user_player: &Player, all_inputs: &mut Vec<String>, max: bool, depth: i32) -> i32 {
    let score = eval(&board, &engine_player, &user_player, all_inputs);
    if score != 10 {
        if score == 1 {
        return 100;
    } else if score == -1 {
        return -100;
    } else if score == 0 {
        return 0;
    }
}
    let board_vec = convert_to_vector(board);
    if max {
    let mut best = -100;
        for i in 0..3 {
            for j in 0..3 {
                if board_vec[i][j] == 10 {
                    let engine_move = find_move(i, j);
                    update_board_state(board, &user_player, &engine_move.to_string());
                    let best_score = minimax(board, &engine_player, &user_player, all_inputs, false, depth + 1);
                    reset_board_state(board, &engine_move.to_string());
                    if best_score > best {
                        best = best_score;
                    }
                }
            }
        }
        return best - depth;
    } else {
        let mut best = 100;
        for i in 0..3 {
            for j in 0..3 {
                if board_vec[i][j] == 10 {
                    let engine_move = find_move(i, j);
                    update_board_state(board, &engine_player, &engine_move.to_string());
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
pub fn find_move(i: usize, j: usize) -> i32 {
    if i == 0 {
        if j == 0 {
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
pub fn best_move(board: &mut Board, engine_player: &Player, user_player: &Player, all_inputs: &mut Vec<String>) -> i32 {
    let mut best_eval = -100;
    let mut best_move = 0;
    let board_vec = convert_to_vector(board);
    for i  in 0..3 {
        for j in 0..3  {
            if board_vec[i][j] == 10 {
            let engine_move = find_move(i, j);
            update_board_state(board, engine_player, &engine_move.to_string());
            let eval_move = minimax(board, engine_player, user_player, all_inputs, false, 1);
            reset_board_state(board, &engine_move.to_string());
            println!("Eval: {}", eval_move);
                if eval_move > best_eval {
                    best_move = engine_move;
                    best_eval = eval_move;
            }
            }    
        }
    }
    println!("{}", best_move);
    return best_move.try_into().unwrap();
}
// A function to get a radnom move
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
pub fn first_move(board: &mut Board) -> bool {
    if board.row1.a != vec![] {
        return true;
    } else if board.row1.c != vec![] {
        return true;
    } else if board.row3.a != vec![] {
        return true;
    } else if board.row3.c != vec![] {
        return true;
    } else {
        return false;
    }
    
}
// A function that combines all functions so the engine runs
pub fn run_engine(board: &mut Board, player: &Player, engine_player: &Player, user_player: &Player, difficulty: &str, all_inputs: &mut Vec<String>) -> (Board, Vec<String>)  {
    // Checks for random first because it runs different code than the other difficulties
    if difficulty == "y" {
        // Calls random_move
        return random_move(board, engine_player, all_inputs.to_vec());
    } else {
        if moves_left(&all_inputs).len() == 8 && first_move(board) == true {
            let first_move = 5;
            update_board_state(board, player, &first_move.to_string());
            all_inputs.push(first_move.to_string());
            return (board.clone(), all_inputs.to_vec());
        }
       let engine_move = best_move(board, engine_player, user_player, all_inputs);
       update_board_state(board, player, &engine_move.to_string());
       all_inputs.push(engine_move.to_string());
       println!("{:?}", all_inputs);
       return (board.clone(), all_inputs.to_vec());
    }
}