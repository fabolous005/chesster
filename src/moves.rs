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
        'P' => {
            for pos_option in PawnMoveOptionsWhite::get() {
                let mut to = Square::from_xy(0, 0);
                match pos_option {
                    PawnMoveOptionsWhite::Normal(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsWhite::Double(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsWhite::CaptureR(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsWhite::CaptureL(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsWhite::EnPassant(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                }
                if to.stage1_check() {
                    if to.is_promoting_white() {
                        for promotion_piece in PromotionPieces::get() {
                            moves.push(Move { from: pos, to, promotion: Some(promotion_piece) });
                        }
                    } else {
                        moves.push(Move { from: pos, to, promotion: None });
                    }
                }
            }
        },
        'p' => {
            for pos_option in PawnMoveOptionsBlack::get() {
                let mut to = Square::from_xy(0, 0);
                match pos_option {
                    PawnMoveOptionsBlack::Normal(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsBlack::Double(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsBlack::CaptureR(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsBlack::CaptureL(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    PawnMoveOptionsBlack::EnPassant(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                }
                if to.stage1_check() {
                    if to.is_promoting_black() {
                        for promotion_piece in PromotionPieces::get() {
                            moves.push(Move { from: pos, to, promotion: Some(promotion_piece) });
                        }
                    } else {
                        moves.push(Move { from: pos, to, promotion: None });
                    }
                }
            }
        },
        'N' | 'n' => {
            for pos_option in KnightMoveOptions::get() {
                let mut to = Square::from_xy(0, 0);
                match pos_option {
                    KnightMoveOptions::Normal(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                }
                if to.stage1_check() {
                    moves.push(Move { from: pos, to, promotion: None });
                }
            }
        },
        'B' | 'b' => {
            for pos_option in BishopMoveOptions::get() {
                let mut to = Square::from_xy(0, 0);
                match pos_option {
                    BishopMoveOptions::Normal(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                }
                if to.stage1_check() {
                    moves.push(Move { from: pos, to, promotion: None });
                }
            }
        },
        'R' | 'r' => {
            for pos_option in RookMoveOptions::get() {
                let mut to = Square::from_xy(0, 0);
                match pos_option {
                    RookMoveOptions::Normal(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                }
                if to.stage1_check() {
                    moves.push(Move { from: pos, to, promotion: None });
                }
            }
        },
        'Q' | 'q' => {
            for pos_option in QueenMoveOptions::get() {
                let mut to = Square::from_xy(0, 0);
                match pos_option {
                    QueenMoveOptions::Normal(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                }
                if to.stage1_check() {
                    moves.push(Move { from: pos, to, promotion: None });
                }
            }
        },
        'K' | 'k' => {
            for pos_option in KingMoveOptions::get() {
                let mut to = Square::from_xy(0, 0);
                match pos_option {
                    KingMoveOptions::Normal(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                    KingMoveOptions::Castle(change) => {
                        to = Square::from_xy(
                            (pos.x as i8 +
                            change.x) as u8,
                            (pos.y as i8 +
                            change.y) as u8
                        );
                    },
                }
                if to.stage1_check() {
                    moves.push(Move { from: pos, to, promotion: None });
                }
            }
        },
        _ => println!("pice not yet implemented: {}", piece)
    }
    moves
}
