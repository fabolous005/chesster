use crate::pieces::extra::Change;


pub enum QueenMoveOptions {
    Normal(Change),
}


impl QueenMoveOptions {
    pub fn get() -> Vec<QueenMoveOptions> {
        let mut moves = Vec::new();
        moves.push(QueenMoveOptions::Normal(Change { x: 1, y: 1 }));
        moves.push(QueenMoveOptions::Normal(Change { x: 1, y: -1 }));
        moves.push(QueenMoveOptions::Normal(Change { x: -1, y: -1 }));
        moves.push(QueenMoveOptions::Normal(Change { x: -1, y: 1 }));
        moves.push(QueenMoveOptions::Normal(Change { x: 1, y: 0 }));
        moves.push(QueenMoveOptions::Normal(Change { x: 0, y: 1 }));
        moves.push(QueenMoveOptions::Normal(Change { x: -1, y: 0 }));
        moves.push(QueenMoveOptions::Normal(Change { x: 0, y: -1 }));
        moves
    }
}
