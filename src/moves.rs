use crate::square::Square;
use crate::pieces::pawn::*;
use crate::pieces::knight::KnightMoveOptions;
use crate::pieces::bishop::BishopMoveOptions;
use crate::pieces::rook::RookMoveOptions;
use crate::pieces::queen::QueenMoveOptions;
use crate::pieces::king::KingMoveOptions;

use std::hash::{Hash, Hasher};




#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    pub fn to_string(&self) -> String {
        match self {
            PromotionPieces::Queen => "Q".to_string(),
            PromotionPieces::Rook => "R".to_string(),
            PromotionPieces::Bishop => "B".to_string(),
            PromotionPieces::Knight => "N".to_string(),
        }
        // WARN: This does not handle Black or white promotion
    }
}


#[derive(Debug, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PromotionPieces>,
}


        impl PartialEq for Move {
            fn eq(&self, other: &Self) -> bool {
                self.from == other.from && self.to == other.to
            }
        }

        impl Eq for Move {}

        impl Hash for Move {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.from.hash(state);
                self.to.hash(state);
            }
        }

pub fn get_moves_white(piece: char, pos: Square) -> Vec<Move> {
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
        'N' => {
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
        'B' => {
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
        'R' => {
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
        'Q' => {
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
        'K' => {
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
        _ => ()
    }
    moves
}

pub fn get_moves_black(piece: char, pos: Square) -> Vec<Move> {
    let mut moves = Vec::new();
    match piece {
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
        'n' => {
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
        'b' => {
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
        'r' => {
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
        'q' => {
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
        'k' => {
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
        _ => ()
    }
    moves
}
