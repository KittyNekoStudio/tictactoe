/// Creates a new board
pub fn board_new() -> [[u8; 3]; 3] {
    return [[0; 3]; 3];
}
// An enum to track witch player we are refering to
#[derive(Debug, PartialEq)]
pub enum Player {
    PlayerOne,
    PlayerTwo 
}