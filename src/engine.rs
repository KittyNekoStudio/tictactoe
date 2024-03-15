use std::io;
use crate::def::Player;
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
pub fn engine_player_type(player: &Player) -> Player {
    match player {
        &Player::PlayerOne => Player::PlayerTwo,
        &Player::PlayerTwo => Player::PlayerOne
    }
}
pub fn run_engine() {
    println!("I'm an engine");
}