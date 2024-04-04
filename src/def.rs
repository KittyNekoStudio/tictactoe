#[derive(Debug, Clone)]
// The row for the board
pub struct Row {
    pub a: u8,
    pub b: u8,
    pub c: u8
    
}
#[derive(Debug, Clone)]
// The board thats a Vector
pub struct Board {
    pub row1: Row,
    pub row2: Row,
    pub row3: Row
}
// Makes a new Board Instance
impl Board {
    pub fn new() -> Board {
        Board {
            row1: Row {
                a: 0,
                b: 0,
                c: 0,
            },
            row2: Row {
                a: 0,
                b: 0,
                c: 0,
            },
            row3: Row {
                a: 0,
                b: 0,
                c: 0,
            },
        }
    }
}
// An enum to track witch player we are refering to
#[derive(Debug, PartialEq)]
pub enum Player {
    PlayerOne,
    PlayerTwo 
}