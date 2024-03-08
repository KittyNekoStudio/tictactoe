pub struct Row;
impl Row {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const C: usize = 2;
}

pub struct Column;
impl Column {
    pub const A: usize = 0;
    pub const B: usize = 1;
    pub const C: usize = 2;
}

pub struct Board;
impl Board {
    pub const ROW1: usize = Row::A;
    pub const ROW2: usize = Row::B;
    pub const ROW3: usize = Row::C;
    pub const COLUMN1: usize = Column::A;
    pub const COLUMN2: usize = Column::B;
    pub const COLUMN3: usize = Column::C;

}
pub struct Circle;
impl Circle {
    pub const MOVE: usize = 0;
}

pub struct Xs;
impl Xs {
    pub const MOVE: usize = 1;
}