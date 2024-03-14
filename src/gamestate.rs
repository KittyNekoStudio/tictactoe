use crate::def::{Board, Player};
use std::io;
// Print the board to the console
pub fn print_board(board: Board) {
    println!("{:?}", board);
}
// Swap players
fn switch_player_turn(player: Player) -> Player {
    match player {
        Player::PlayerOne => Player::PlayerTwo,
        Player::PlayerTwo => Player::PlayerOne,
    }
}
// Recive player input
pub fn recive_input() -> String {
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).unwrap();
    let player_input_clean: String = player_input.trim().parse().unwrap();
    // Calls recive input again if player inputed non allowed string
    if check_player_input(&player_input_clean) == false {
        return recive_input();
    }
    return player_input_clean;
}
// Checks if input has already been recived
pub fn if_input_exsits(all_inputs: &Vec<String>, new_input: String) -> bool {
    let mut i = 0;
    while i < all_inputs.len() {
        if all_inputs[i] == new_input {
            return true;
        }
        i += 1;
    }
    return false;
}
// Checks that player input is one of the allowed strings
pub fn check_player_input(player_input: &String) -> bool {
    match player_input.as_str() {
        "a1" => true,
        "a2" => true,
        "a3" => true,
        "b1" => true,
        "b2" => true,
        "b3" => true,
        "c1" => true,
        "c2" => true,
        "c3" => true,
        _ => false
    }
}
// Pushes 1 or 0 to the cell corresponding to the player type
pub fn update_board_state(board: &mut Board, player: &Player, player_input: &String) {
    if player == &Player::PlayerOne {
        match player_input.as_str() {
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
        match player_input.as_str() {
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
// User chooses player by typeing either X or O
pub fn choose_player() -> Player {
    println!("To choose player please type X(first player) or O(second player)");
    let mut player_input = String::new().to_lowercase();
    io::stdin().read_line(&mut player_input).unwrap();
    let player_input_clean: String = player_input.trim().parse().unwrap();
    match player_input_clean.as_str() {
        "x" => Player::PlayerOne,
        "o" => Player::PlayerTwo,
        _ => choose_player()
    }
}
// A function that checks if any of the winning positions are reached
pub fn did_win(board: &Board, player: &Player) -> bool {
    let mut num = 0;
    
    if player == &Player::PlayerOne {
        num = 1;
    } else {
        num = 0;
    }
    if board.row1.a == vec![num] && board.row2.a == vec![num] && board.row3.a == vec![num] {
        return true;
    } else if board.row1.b == vec![num] && board.row2.b == vec![num] && board.row3.b == vec![num] {
        return true;
    } else if board.row1.c == vec![num] && board.row2.c == vec![num] && board.row3.c == vec![num] {
        return true;
    }  else if board.row1.a == vec![num] && board.row1.b == vec![num] && board.row1.c == vec![num] {
        return true;
    } else if board.row2.a == vec![num] && board.row2.b == vec![num] && board.row2.c == vec![num] {
        return true;
    } else if board.row3.a == vec![num] && board.row3.b == vec![num] && board.row3.c == vec![num] {
        return true;
    } else if board.row1.a == vec![num] && board.row2.b == vec![num] && board.row3.c == vec![num] {
        return true;
    } else if board.row3.a == vec![num] && board.row2.b == vec![num] && board.row1.c == vec![num] {
        return true;
    } else {
        return false;
    }
}
// Check if none of the winning positions are met when all moves are made
pub fn is_draw(board: &Board, player1: &Player, player2: &Player) -> bool {
    if did_win(&board, &player1) == false && did_win(&board, &player2) == false {
        return true;
    } else {
        return false;
    }
}
// Function that combines all functions to make the program run
pub fn play_game() {
    let mut board = Board::new();
    // Holds all input used to check if already have input and for the main loop condition
    let mut all_inputs: Vec<String> = Vec::new();
    let mut player = choose_player();
    // Variables to check for draw
    let player_x = Player::PlayerOne;
    let player_o = Player::PlayerTwo;
    // Have the code run once without the loop activated to be able to use the function if_input_exsits
    let player_input = recive_input();
    update_board_state(&mut board, &player, &player_input);
    all_inputs.push(player_input.clone());
    println!("{:?}", player);
    player = switch_player_turn(player);
    println!("{:?}", all_inputs);
    
    loop {
        // Loop breaks if board becomes full
        if all_inputs.len() == 9 {
            break;
        }
        // Recive user input
        let mut player_input = recive_input();
        // Checks if cell is already filled if it is the call recive_input again
        while if_input_exsits(&all_inputs, player_input.clone()) {
            player_input = recive_input();
        }
        // Update the board with the input
        update_board_state(&mut board, &player, &player_input);
        all_inputs.push(player_input.clone());
        // If win break loop
        if did_win(&board, &player) {
            break;
        }
        // Code for debugging to be able to read the player turn and all input
        println!("{:?}", player);
        // Switch player turn
        player = switch_player_turn(player);
        println!("{:?}", all_inputs);

    }
        // If win check witch player and print the corrasoponding string
        if did_win(&board, &player) {
            if player == Player::PlayerOne {
                return println!("Good job on winning player X! Thanks for playing. Hope to see you around again.");
            } else {
                return println!("Good job on winning player O! Thanks for playing. Hope to see you around again.");
            }
        }
        // If draw print the string
        if is_draw(&board, &player_x, &player_o) {
            return println!("Too bad! You drew. Try winning next time.");
        }
    }
