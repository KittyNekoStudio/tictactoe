use core::fmt;
use crate::gamestate::print_board;
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
// Display for board
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_board();
        Ok(())
    }
}
// An enum to track witch player we are refering to
#[derive(Debug, PartialEq)]
pub enum Player {
    PlayerOne(Xs),
    PlayerTwo(Circles) 
}
// The Xs are player 1
#[derive(Debug, PartialEq)]
pub struct Xs;
impl Xs {
    pub const MOVE: usize = 0;
}
// The Circles are player 2
#[derive(Debug, PartialEq)]
pub struct Circles;
impl Circles {
    pub const MOVE: usize = 1;
}
