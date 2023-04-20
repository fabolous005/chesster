use crate::pieces::extra::Change;


pub enum RookMoveOptions {
    Normal(Change),
}

impl RookMoveOptions {
    pub fn get() -> Vec<RookMoveOptions> {
        let mut moves = Vec::new();
        moves.push(RookMoveOptions::Normal(Change { x: 1, y: 0 }));
        moves.push(RookMoveOptions::Normal(Change { x: 0, y: 1 }));
        moves.push(RookMoveOptions::Normal(Change { x: -1, y: 0 }));
        moves.push(RookMoveOptions::Normal(Change { x: 0, y: -1 }));
        moves
    }
}
