use crate::moves::Move;
use crate::position::{Position, self};

pub struct Line {
    move_: Move,
    value: i32,
    line: Option<Box<Vec<Line>>>
}

impl Line {
    pub fn deeper(&mut self, position: Position) -> &mut Self {
        let line = self.deepest();
        for move_ in position.get_moves() {
            let mut new_position = position;
            new_position.make_move(&move_);
            line.deeper(new_position);
            let self.line = Some(Box::new(
                Line {
                    move_,
                    value: 0,
                    line: None
                }));
            self
        }
        line
    }

    pub fn deepest(&mut self) -> &mut Self {
        if let Some(ref mut line) = self.line {
            line.last_mut().unwrap().deepest();
        }
        self
    }
}

pub fn get_lines(position: &mut Position, depth: i32) -> Vec<Line> {
    let mut lines = Vec::new();
    let moves = position.get_moves();
    for move_ in moves {
        let mut new_position = position;
        get_lines(new_position.make_move(&move_), 0);
        
    }
    lines
}