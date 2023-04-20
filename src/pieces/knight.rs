use crate::pieces::extra::Change;


pub enum KnightMoveOptions {
    Normal(Change),
}

impl KnightMoveOptions {
    pub fn get() -> Vec<KnightMoveOptions> {
        let mut moves = Vec::new();
        moves.push(KnightMoveOptions::Normal(Change { x: 1, y: 2 }));
        moves.push(KnightMoveOptions::Normal(Change { x: 2, y: 1 }));
        moves.push(KnightMoveOptions::Normal(Change { x: 2, y: -1 }));
        moves.push(KnightMoveOptions::Normal(Change { x: 1, y: -2 }));
        moves.push(KnightMoveOptions::Normal(Change { x: -1, y: -2 }));
        moves.push(KnightMoveOptions::Normal(Change { x: -2, y: -1 }));
        moves.push(KnightMoveOptions::Normal(Change { x: -2, y: 1 }));
        moves.push(KnightMoveOptions::Normal(Change { x: -1, y: 2 }));
        moves
    }
}
