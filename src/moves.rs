use crate::square::Square;
use crate::pieces::pawn::*;
use crate::pieces::knight::KnightMoveOptions;
use crate::pieces::bishop::BishopMoveOptions;
use crate::pieces::rook::RookMoveOptions;
use crate::pieces::queen::QueenMoveOptions;
use crate::pieces::king::KingMoveOptions;




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


#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PromotionPieces>,
}


pub fn get_moves(piece: char, pos: Square) -> Vec<Move> {
    let mut moves = Vec::new();
    match piece {
        'P' => {
            for pos_option in PawnMoveOptionsWhite::get() {
                let to: Square;
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
                        if ! pos.is_en_passant_white() {
                            to = Square::from_xy( 9, 9);
                        } else {
                            to = Square::from_xy(
                                (pos.x as i8 +
                                change.x) as u8,
                                (pos.y as i8 +
                                change.y) as u8
                            );
                        }
                    },
                }
                if to.stage1_check() {
                    if to.is_promoting_white() {
                        for promotion_piece in PromotionPieces::get() {
                            moves.push(Move { 
                                from: pos,
                                to,
                                promotion: Some(promotion_piece) 
                            });
                        }
                    } else {
                        moves.push(Move { from: pos, to, promotion: None });
                    }
                }
            }
        },
        'p' => {
            for pos_option in PawnMoveOptionsBlack::get() {
                let to: Square;
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
                        if ! pos.is_en_passant_black() {
                            to = Square::from_xy( 9, 9);
                        } else {
                            to = Square::from_xy(
                                (pos.x as i8 +
                                change.x) as u8,
                                (pos.y as i8 +
                                change.y) as u8
                            );
                        }
                    },
                }
                if to.stage1_check() {
                    if to.is_promoting_black() {
                        for promotion_piece in PromotionPieces::get() {
                            moves.push(Move {
                                from: pos,
                                to,
                                promotion: Some(promotion_piece)
                            });
                        }
                    } else {
                        moves.push(Move { from: pos, to, promotion: None });
                    }
                }
            }
        },
        'N' | 'n' => {
            for pos_option in KnightMoveOptions::get() {
                let to: Square;
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
            for mut range in 1..8 {
                for pos_option in BishopMoveOptions::get() {
                    let to: Square;
                    match pos_option {
                        BishopMoveOptions::Normal(change) => {
                            to = Square::from_xy(
                                (pos.x as i8 +
                                (change.x *
                                range)) as u8,
                                (pos.y as i8 +
                                (change.y *
                                range)) as u8
                            );
                        },
                    }
                    if to.stage1_check() {
                        moves.push(Move { from: pos, to, promotion: None });
                    }
                }
                range += 1;
            }
        },
        'R' | 'r' => {
            for mut range in 1..8 {
                for pos_option in RookMoveOptions::get() {
                    let to: Square;
                    match pos_option {
                        RookMoveOptions::Normal(change) => {
                            to = Square::from_xy(
                                (pos.x as i8 +
                                (change.x *
                                range)) as u8,
                                (pos.y as i8 +
                                (change.y *
                                range)) as u8
                            );
                        },
                    }
                    if to.stage1_check() {
                        moves.push(Move { from: pos, to, promotion: None });
                    }
                }
                range += 1;
            }
        },
        'Q' | 'q' => {
            for mut range in 1..8 {
                for pos_option in QueenMoveOptions::get() {
                    let to: Square;
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
                range += 1;
            }
        },
        'K' | 'k' => {
            for pos_option in KingMoveOptions::get() {
                let to: Square;
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
