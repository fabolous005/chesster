use std::vec;

use crate::moves::Move;
use crate::position::{Position};

pub struct Line {
    move_: Move,
    value: i32,
    abondened: bool,
    line: Option<Box<Vec<Line>>>
}

impl Line {
    pub fn deeper(&mut self, position: Position) -> &mut Self {
        let line = self.deepest();
        for move_ in position.get_moves() {
            let mut new_position = position;
            new_position.make_move(&move_);
            let new_line = line.deeper(new_position);
            new_line.line = Some(Box::new(
                vec![
                    Line {
                        move_,
                        value: 0,
                        abondened: false,
                        line: None
                    }
                ]
            ));
            // self
        }
        line
    }

    pub fn deepest(&mut self) -> &mut Self {
        if let Some(ref mut line) = self.line {
            line.last_mut().unwrap().deepest();
        }
        self
    }

    pub fn get_lines(position: &Position, depth: i32) -> Vec<Line> {
        let mut lines = Vec::new();
        let moves = position.get_moves();
        for move_ in moves {
            let new_position: &mut Position = &mut position.clone();
            new_position.make_move(&move_);
            lines.push(Line { move_, value: 0, abondened: false, line: None })
        }
        lines
    }

    pub fn get_line(position: &Position) -> Vec<Line> {
        let mut lines = Vec::new();
        let moves = position.get_moves();
        for move_ in moves {
            let new_position: &mut Position = &mut position.clone();
            new_position.make_move(&move_);
            lines.push(Line { move_, value: 0, abondened: false, line: None })
        }
        lines
    }

}


