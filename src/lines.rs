use crate::moves::Move;
use crate::position::Position;

pub struct Line {
    move_: Move,
    values: i32,
    line: Option<Box<Line>>
}

impl Line {
    pub fn get_lines(position: Position) -> Vec<Line> {
        todo!()
    }
}