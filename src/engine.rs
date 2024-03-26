use std::io;
use rand::seq::SliceRandom;
use crate::def::{Board, Player};
use crate::gamestate::{self, did_win, if_input_exsits, print_board, reset_board_state, switch_player_turn, update_board_state};
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
pub fn eval(board: &Board, engine_player: &Player) -> i32 {
    if did_win(&board, &engine_player) {
        return 1;
    } if did_win(&board, &switch_player_turn(engine_player)) {
        return -1;
    } 
    return 0;   
}
pub fn search_move(board: &mut Board, player: &Player, engine_player: &Player, user_player: &Player, mut all_inputs: Vec<String>) -> i32 {
    let score = eval(&board, &engine_player);
    if score == 1 {
        return 1;
    } else if score == -1 {
        return -1
    } else if moves_left(&all_inputs).len() == 0 {
        return 0;
    } 

    if player == engine_player {
        let mut i = 0;
        let mut best = -100;
    while i == 0 {
        if board.row1.a == vec![] {
            all_inputs.push(String::from("1"));
            update_board_state(board, player, &1.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("1"));
        } else if board.row2.a == vec![] {
            all_inputs.push(String::from("2"));
            update_board_state(board, player, &2.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("2"));
        } else if board.row3.a == vec![] {
            all_inputs.push(String::from("3"));
            update_board_state(board, player, &3.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("3"));
        } else if board.row1.b == vec![] {
            all_inputs.push(String::from("4"));
            update_board_state(board, player, &4.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("4"));
        } else if board.row2.b == vec![] {
            all_inputs.push(String::from("5"));
            update_board_state(board, player, &5.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("5"));
        } else if board.row3.b == vec![] {
            all_inputs.push(String::from("6"));
            update_board_state(board, player, &6.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("6"));
        } else if board.row1.c == vec![] {
            all_inputs.push(String::from("7"));
            update_board_state(board, player, &7.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("7"));
        } else if board.row2.c == vec![] {
            all_inputs.push(String::from("8"));
            update_board_state(board, player, &8.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("8"));
        } else if board.row3.c == vec![] {
            all_inputs.push(String::from("9"));
            update_board_state(board, player, &9.to_string());
            best = search_move(board, &user_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("9"));
        }
        i += 1;
    }
    return best;
} else {
    let mut i = 0;
    let mut best = 100;
    while i == 0 {
        if board.row1.a == vec![] {
            all_inputs.push(String::from("1"));
            update_board_state(board, player, &1.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("1"));
        } else if board.row2.a == vec![] {
            all_inputs.push(String::from("2"));
            update_board_state(board, player, &2.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("2"));
        } else if board.row3.a == vec![] {
            all_inputs.push(String::from("3"));
            update_board_state(board, player, &3.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("3"));
        } else if board.row1.b == vec![] {
            all_inputs.push(String::from("4"));
            update_board_state(board, player, &4.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("4"));
        } else if board.row2.b == vec![] {
            all_inputs.push(String::from("5"));
            update_board_state(board, player, &5.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("5"));
        } else if board.row3.b == vec![] {
            all_inputs.push(String::from("6"));
            update_board_state(board, player, &6.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("6"));
        } else if board.row1.c == vec![] {
            all_inputs.push(String::from("7"));
            update_board_state(board, player, &7.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("7"));
        } else if board.row2.c == vec![] {
            all_inputs.push(String::from("8"));
            update_board_state(board, player, &8.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("8"));
        } else if board.row3.c == vec![] {
            all_inputs.push(String::from("9"));
            update_board_state(board, player, &9.to_string());
            best = search_move(board, &engine_player, &engine_player, &user_player, all_inputs.clone());
            println!("{}", best);
            reset_board_state(board, &String::from("9"));
        }
        i += 1;
    }
    return best;
}
}
pub fn best_move(board: &mut Board, player: &Player, engine_player: &Player, user_player: &Player, mut all_inputs: Vec<String>) -> i32 { 
    let mut best_eval = -100;
    let moves_left1 = moves_left(&all_inputs);
    let mut best_move = 0;
    let mut i = 0;
    while i < moves_left1.len() {
        let moves_left_inside = moves_left(&all_inputs);
        update_board_state(board, player, &moves_left_inside[0].to_string());
        all_inputs.push(moves_left_inside[0].to_string());
        let eval_move = search_move(board, player, engine_player, user_player, all_inputs.clone());
        reset_board_state(board, &moves_left_inside[0].to_string());
        if eval_move > best_eval {
            best_move = moves_left_inside[0];
            best_eval = eval_move;
        }
        println!("{}", moves_left_inside[0]);
        println!("{}", best_eval);
        i += 1;
    }
   
    return best_move;
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
pub fn run_engine(board: &mut Board, player: &Player, engine_player: &Player, user_player: &Player, difficulty: &str, mut all_inputs: Vec<String>) -> (Board, Vec<String>)  {
    // Checks for random first because it runs different code than the other difficulties
    if difficulty == "y" {
        // Calls random_move
        return random_move(board, engine_player, all_inputs);
    } else {
        if moves_left(&all_inputs).len() == 8 && first_move(board) == true {
            let first_move = 5;
            update_board_state(board, player, &first_move.to_string());
            all_inputs.push(first_move.to_string());
            return (board.clone(), all_inputs);
        }
       let engine_move = best_move(board, player, engine_player, user_player, all_inputs.clone());
       update_board_state(board, player, &engine_move.to_string());
       all_inputs.push(engine_move.to_string());
       return (board.clone(), all_inputs);
    }
}