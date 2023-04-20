use crate::pieces::extra::Change;


pub enum BishopMoveOptions {
    Normal(Change),
}

impl BishopMoveOptions {
    pub fn get() -> Vec<BishopMoveOptions> {
        let mut moves = Vec::new();
        moves.push(BishopMoveOptions::Normal(Change { x: 1, y: 1 }));
        moves.push(BishopMoveOptions::Normal(Change { x: 1, y: -1 }));
        moves.push(BishopMoveOptions::Normal(Change { x: -1, y: -1 }));
        moves.push(BishopMoveOptions::Normal(Change { x: -1, y: 1 }));
        moves
    }
}
