use crate::moves::Move;
use crate::position::Position;

pub struct Line {
    move_: Move,
    values: i32,
    line: Option<Box<Line>>
}

impl Line {
    pub fn get_lines(position: Position, depth: i32) -> Vec<Line> {
        let mut lines = Vec::new();
        let moves = position.get_moves();
        for move_ in moves {
            let mut new_position = position;
            new_position.make_move(&move_);
            todo!()
        }
        lines
    }
}