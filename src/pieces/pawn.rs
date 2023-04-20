use crate::pieces::extra::Change;


pub enum PawnMoveOptionsBlack {
    Normal(Change),
    Double(Change),
    CaptureL(Change),
    CaptureR(Change),
    EnPassant(Change),
}

impl PawnMoveOptionsBlack {
    pub fn get() -> Vec<PawnMoveOptionsBlack> {
        let mut moves = Vec::new();
        moves.push(PawnMoveOptionsBlack::Normal(Change { x: 0, y: 1 }));
        moves.push(PawnMoveOptionsBlack::Double(Change { x: 0, y: 2 }));
        moves.push(PawnMoveOptionsBlack::CaptureR(Change { x: 1, y: 1 }));
        moves.push(PawnMoveOptionsBlack::CaptureL(Change { x: -1, y: 1 }));
        moves.push(PawnMoveOptionsBlack::EnPassant(Change { x: 1, y: 1 }));
        moves
    }
}



pub enum PawnMoveOptionsWhite {
    Normal(Change),
    Double(Change),
    CaptureL(Change),
    CaptureR(Change),
    EnPassant(Change),
}

impl PawnMoveOptionsWhite {
    pub fn get() -> Vec<PawnMoveOptionsWhite> {
        let mut moves = Vec::new();
        moves.push(PawnMoveOptionsWhite::Normal(Change { x: 0, y: -1 }));
        moves.push(PawnMoveOptionsWhite::Double(Change { x: 0, y: -2 }));
        moves.push(PawnMoveOptionsWhite::CaptureR(Change { x: 1, y: -1 }));
        moves.push(PawnMoveOptionsWhite::CaptureL(Change { x: -1, y: -1 }));
        moves.push(PawnMoveOptionsWhite::EnPassant(Change { x: 1, y: -1 }));
        moves
    }

}