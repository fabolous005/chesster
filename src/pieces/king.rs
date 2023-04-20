use crate::pieces::extra::Change;


pub enum KingMoveOptions {
    Normal(Change),
    Castle(Change),
}

impl KingMoveOptions {
    pub fn get() -> Vec<KingMoveOptions> {
        let mut moves = Vec::new();
        moves.push(KingMoveOptions::Normal(Change { x: 1, y: 1 }));
        moves.push(KingMoveOptions::Normal(Change { x: 1, y: -1 }));
        moves.push(KingMoveOptions::Normal(Change { x: -1, y: -1 }));
        moves.push(KingMoveOptions::Normal(Change { x: -1, y: 1 }));
        moves.push(KingMoveOptions::Normal(Change { x: 1, y: 0 }));
        moves.push(KingMoveOptions::Normal(Change { x: 0, y: 1 }));
        moves.push(KingMoveOptions::Normal(Change { x: -1, y: 0 }));
        moves.push(KingMoveOptions::Normal(Change { x: 0, y: -1 }));
        moves.push(KingMoveOptions::Castle(Change { x: 2, y: 0 }));
        moves.push(KingMoveOptions::Castle(Change { x: -2, y: 0 }));
        moves
    }
}
