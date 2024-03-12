use core::fmt;
use crate::gamestate::print_board;
/*trait Unbox<T> {
    fn unbox(&self) -> T;
}

impl <T: Clone> Unbox<T> for Box<T> {
    fn unbox(&self) -> T {
        (**self).clone()
    }
}*/
#[derive(Debug, Clone)]
pub struct Row {
    pub a: Box<[u32]>,
    pub b: Box<[u32]>,
    pub c: Box<[u32]>
    
}
#[derive(Debug, Clone)]
pub struct Board {
    pub row1: Row,
    pub row2: Row,
    pub row3: Row
}
impl Board {
    pub fn new() -> Box<[Board]> {
        Box::new([Board {
            row1: Row {
                a: Box::new([0]),
                b: Box::new([0]),
                c: Box::new([0]),
            },
            row2: Row {
                a: Box::new([0]),
                b: Box::new([0]),
                c: Box::new([0]),
            },
            row3: Row {
                a: Box::new([0]),
                b: Box::new([0]),
                c: Box::new([0]),
            },
        }])
    }
}
impl Board {
    pub fn push(mut self, row: &str, field: char, num: u32) {
        match row {
            "row1" =>
        match field {
            'a' => self.row1 = Row{a: Box::new([num]), ..self.row1},
            'b' => self.row1 = Row{b: Box::new([num]), ..self.row1},
            'c' => self.row1 = Row{c: Box::new([num]), ..self.row1},
            _ => todo!()
        },
           "row2" =>
        match field {
            'a' => self.row2 = Row{a: Box::new([num]), ..self.row2},
            'b' => self.row2 = Row{b: Box::new([num]), ..self.row2},
            'c' => self.row2 = Row{c: Box::new([num]), ..self.row2},
            _ => todo!()
        },
        "row3" =>
        match field {
            'a' => self.row3 = Row{a: Box::new([num]), ..self.row3},
            'b' => self.row3 = Row{b: Box::new([num]), ..self.row3},
            'c' => self.row3 = Row{c: Box::new([num]), ..self.row3},
            _ => todo!()
        },
        _ => todo!()
        }

    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_board();
        Ok(())
    }
}
pub enum Player {
    PlayerOne(Xs),
    PlayerTwo(Circles) 
}
pub struct Xs;
impl Xs {
    pub const MOVE: usize = 0;
}
pub struct Circles;
impl Circles {
    pub const MOVE: usize = 1;
}
