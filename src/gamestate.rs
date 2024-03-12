use crate::def::{self, Circles, Player, Xs};

pub fn print_board() {
    println!("")
}

fn switch_player_turn(player: Player) -> Player {
    match player {
        Player::PlayerOne(Xs) => Player::PlayerTwo(Circles),
        Player::PlayerTwo(Circles) => Player::PlayerOne(Xs),
    }
}
