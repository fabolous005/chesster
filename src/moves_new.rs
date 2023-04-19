use crate::square::Square;

trait Move {
    fn get(pawn: Square) -> Vec<PawnMove>;
    fn get_moves(pawns: Vec<Square>) -> Vec<PawnMove>;
}




pub enum PromotionPieces {
    Queen,
    Rook,
    Bishop,
    Knight,
}


pub struct Change {
    x: i8,
    y: i8,
}

pub enum PawnMoveOptions {
    Normal(Change),
    Double(Change),
    Capture(Change),
    EnPassant(Change),
    Promotion(Change),
    PromotionCapture(Change),
}


impl PawnMoveOptions {
    pub fn get() -> Vec<PawnMoveOptions> {
        let mut moves = Vec::new();
        moves.push(PawnMoveOptions::Normal(Change { x: 0, y: 1 }));
        moves.push(PawnMoveOptions::Double(Change { x: 0, y: 2 }));
        moves.push(PawnMoveOptions::Capture(Change { x: 1, y: 1 }));
        moves.push(PawnMoveOptions::EnPassant(Change { x: 1, y: 1 }));
        moves.push(PawnMoveOptions::Promotion(Change { x: 0, y: 1 }));
        moves.push(PawnMoveOptions::PromotionCapture(Change { x: 1, y: 1 }));
        moves
    }
}

pub struct PawnMove {
    pub from: Option<Square>,
    pub to: Square,
    pub promotion: Option<PromotionPieces>,
}

impl Move for PawnMove {
    fn get(pawn: Square) -> Vec<PawnMove> {
        let mut moves = Vec::new();
        for pawn_option in PawnMoveOptions::get() {
            match pawn_option {
                PawnMoveOptions::Normal(change) => {
                    let to = Square::from_xy(pawn.x + change.x as u8, pawn.y + change.y as u8);
                    moves.push(PawnMove { from: None, to, promotion: None });
                },
                PawnMoveOptions::Double(change) => {
                    let to = Square::from_xy(pawn.x + change.x as u8, pawn.y + change.y as u8);
                    moves.push(PawnMove { from: None, to, promotion: None });
                },
                PawnMoveOptions::Capture(change) => {
                    let to = Square::from_xy(pawn.x + change.x as u8, pawn.y + change.y as u8);
                    moves.push(PawnMove { from: None, to, promotion: None });
                },
                PawnMoveOptions::EnPassant(change) => {
                    let to = Square::from_xy(pawn.x + change.x as u8, pawn.y + change.y as u8);
                    moves.push(PawnMove { from: None, to, promotion: None });
                },
                PawnMoveOptions::Promotion(change) => {
                    let to = Square::from_xy(pawn.x + change.x as u8, pawn.y + change.y as u8);
                    moves.push(PawnMove { from: None, to, promotion: None });
                },
                PawnMoveOptions::PromotionCapture(change) => {
                    let to = Square::from_xy(pawn.x + change.x as u8, pawn.y + change.y as u8);
                    moves.push(PawnMove { from: None, to, promotion: None });
                },
                _ => {}
            }
        }

        moves
    }

    fn get_moves(pawns: Vec<Square>) -> Vec<PawnMove> {
        let mut moves = Vec::new();
        for pawn in pawns {
            for pawn_move in PawnMove::get(pawn) {
                moves.push(pawn_move);
            }
        }
        moves
    }
}