use crate::square::Square;
use std::collections::HashMap;
use crate::square::Square;

pub struct Move {
    pub from: Option<String>,
    pub to: String,
}


enum PromotionPieces {
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
    Double(Chnage),
    DoubleCapture(Change),
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


pub struct PawnMoves {
    moves: Vec<HashMap<String, Change>>
}

impl PawnMoves {
    pub fn new() -> PawnMoves {
        let mut moves = Vec::new();
        let mut normal = HashMap::new();
        normal.insert("normal".to_string(), Change { x: 0, y: 1 });
        moves.push(normal);
        let mut double = HashMap::new();
        double.insert("double".to_string(), Change { x: 0, y: 2 });
        moves.push(double);
        let mut capture = HashMap::new();
        capture.insert("capture".to_string(), Change { x: 1, y: 1 });
        moves.push(capture);
        let mut en_passant = HashMap::new();
        en_passant.insert("en_passant".to_string(), Change { x: 1, y: 1 });
        moves.push(en_passant);
        let mut promotion = HashMap::new();
        promotion.insert("promotion".to_string(), Change { x: 0, y: 1 });
        moves.push(promotion);
        let mut promotion_capture = HashMap::new();
        promotion_capture.insert("promotion_capture".to_string(), Change { x: 1, y: 1 });
        moves.push(promotion_capture);
        let mut double_capture = HashMap::new();
        double_capture.insert("double_capture".to_string(), Change { x: 1, y: 2 });
        moves.push(double_capture);
        PawnMoves { moves }
    }
}


pub struct PawnMove {
    pub from: Option<String>,
    pub to: String,
    pub promotion: Option<PromotionPieces>,
}


impl Move {
    pub fn get_moves(pos: Position) -> Vec<Move> {
        Vec::new()
    }
}

impl Move for PawnMove {
    fn get(pawn: Square) -> Vec<PawnMove> {
        let moves = Vec::new();
        for pawn_option in PawnMoveOptions::get() {
            match pawn_option {
                PawnMoveOptions::Normal(change) => {
                    let to = Square::from_xy(pawn.x + change.x, pawn.y + change.y);
                    moves.push(PawnMove { from: None, to: to.to_string(), promotion: None });
                },
                PawnMoveOptions::Double(change) => {
                    let to = Square::from_xy(pawn.x + change.x, pawn.y + change.y);
                    moves.push(PawnMove { from: None, to: to.to_string(), promotion: None });
                },
                PawnMoveOptions::Capture(change) => {
                    let to = Square::from_xy(pawn.x + change.x, pawn.y + change.y);
                    moves.push(PawnMove { from: None, to: to.to_string(), promotion: None });
                },
                PawnMoveOptions::EnPassant(change) => {
                    let to = Square::from_xy(pawn.x + change.x, pawn.y + change.y);
                    moves.push(PawnMove { from: None, to: to.to_string(), promotion: None });
                },
                PawnMoveOptions::Promotion(change) => {
                    let to = Square::from_xy(pawn.x + change.x, pawn.y + change.y);
                    moves.push(PawnMove { from: None, to: to.to_string(), promotion: None });
                },
                PawnMoveOptions::PromotionCapture(change) => {
                    let to = Square::from_xy(pawn.x + change.x, pawn.y + change.y);
                    moves.push(PawnMove { from: None, to: to.to_string(), promotion: None });
                },
                PawnMoveOptions::DoubleCapture(change) => {
                    let to = Square::from_xy(pawn.x + change.x, pawn.y + change.y);
                    moves.push(PawnMove { from: None, to: to.to_string(), promotion: None });
                },
                _ => {}
            }
        }

        moves
    }

    fn get_moves(pawns: Vec<Square>) -> Vec<PawnMove> {
        let moves = Vec::new();
        for pawn in pawns {
            moves.push(PawnMove::get(pawn));
        }
        moves
    }
}