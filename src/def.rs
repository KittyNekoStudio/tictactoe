#[derive(Debug, Clone)]
// The row for the board
pub struct Row {
    pub a: Vec<u32>,
    pub b: Vec<u32>,
    pub c: Vec<u32>
    
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
                a: vec![],
                b: vec![],
                c: vec![],
            },
            row2: Row {
                a: vec![],
                b: vec![],
                c: vec![],
            },
            row3: Row {
                a: vec![],
                b: vec![],
                c: vec![],
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