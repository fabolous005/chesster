use crate::square::Square;


pub struct Change {
    x: i8,
    y: i8,
}

#[derive(Debug)]
pub enum PromotionPieces {
    Queen,
    Rook,
    Bishop,
    Knight,
}

impl PromotionPieces {
    pub fn get() -> Vec<PromotionPieces> {
        let mut pieces = Vec::new();
        pieces.push(PromotionPieces::Queen);
        pieces.push(PromotionPieces::Rook);
        pieces.push(PromotionPieces::Bishop);
        pieces.push(PromotionPieces::Knight);
        pieces
    }
}

pub enum PawnMoveOptions {
    Normal(Change),
    Double(Change),
    CaptureL(Change),
    CaptureR(Change),
    EnPassant(Change),
}


impl PawnMoveOptions {
    pub fn get() -> Vec<PawnMoveOptions> {
        let mut moves = Vec::new();
        moves.push(PawnMoveOptions::Normal(Change { x: 0, y: 1 }));
        moves.push(PawnMoveOptions::Double(Change { x: 0, y: 2 }));
        moves.push(PawnMoveOptions::CaptureR(Change { x: 1, y: 1 }));
        moves.push(PawnMoveOptions::CaptureL(Change { x: -1, y: 1 }));
        moves.push(PawnMoveOptions::EnPassant(Change { x: 1, y: 1 }));
        moves
    }
}


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



#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PromotionPieces>,
}


pub fn get_moves(piece: char, pos: Square) -> Vec<Move> {
    let mut moves = Vec::new();
    match piece {
        // TODO: add checks for out ot board, and for pieces in the way, and for check pinning
        'P' | 'p' => {
            for pos_option in PawnMoveOptions::get() {
                match pos_option {
                    PawnMoveOptions::Normal(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                    PawnMoveOptions::Double(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                    PawnMoveOptions::CaptureR(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                    PawnMoveOptions::CaptureL(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                    PawnMoveOptions::EnPassant(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                }
            }
            if moves.last().unwrap().to.y == 0 | 8 {
                for promote_option in PromotionPieces::get() {
                    moves.push(Move { from: pos, to: moves.last().unwrap().to, promotion: Some(promote_option) });
                }
            }
        },
        'N' | 'n' => {
            for pos_option in KnightMoveOptions::get() {
                match pos_option {
                    KnightMoveOptions::Normal(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                }
            }
        },
        'B' | 'b' => {
            for pos_option in BishopMoveOptions::get() {
                match pos_option {
                    BishopMoveOptions::Normal(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                }
            }
        },
        'R' | 'r' => {
            for pos_option in RookMoveOptions::get() {
                match pos_option {
                    RookMoveOptions::Normal(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                }
            }
        },
        'Q' | 'q' => {
            for pos_option in QueenMoveOptions::get() {
                match pos_option {
                    QueenMoveOptions::Normal(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                }
            }
        },
        'K' | 'k' => {
            for pos_option in KingMoveOptions::get() {
                match pos_option {
                    KingMoveOptions::Normal(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                    KingMoveOptions::Castle(change) => {
                        let to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                        moves.push(Move { from: pos, to, promotion: None });
                    },
                }
            }
        },
        _ => println!("pice not yet implemented: {}", piece)
    }
    moves
}
