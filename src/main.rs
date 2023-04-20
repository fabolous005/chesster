#![feature(exclusive_range_pattern)]

mod square;
mod position;
mod moves;
use crate::position::Position;


fn main() {
    let example_positions = vec![
        "r2q1b2/p1k5/1p1p4/2pQ4/4P3/2B5/P1PP2PP/R3K1NR b KQ - 1 21",
        "6k1/1p2p1bp/1R3pp1/8/4B3/8/5PPP/2Br2K1 w - -",
        "r1b1kb1r/pp6/3p1p2/2pPp2p/q3N3/8/P1PB1PPP/R2QKB1R w KQkq - 2 13",
        "r1b1kb1r/pp3pp1/2p2n1p/q3N3/N2pP3/1B1P4/PPP2PPP/R2Q1RK1 b kq - 0 11",
        "1k1r2nr/p4pbp/2Qp1p2/8/N3P3/1B3N2/P4PPP/R3K2R w KQ - 1 17",
        "3r2nr/p1k2pbp/Q2p4/3B4/N3p3/5N2/P4PPP/1R2K2R w K - 2 20"
    ];
    let mut positions = Vec::new();
    for pos in example_positions {
        positions.push(Position::from_string(pos).unwrap());
    }

    let test_pos_b = Position::from_string(
//        "r2q1b2/p1k5/1p1p4/2pQ4/4P3/2B5/P1PP2PP/R3K1NR b KQ - 1 21"
        "8/PP6/8/8/8/8/8/8 w - - 0 1"
    ).unwrap();
    test_pos_b.print();
    let test_pos = test_pos_b.get_moves();
    println!("{:#?}", test_pos);
    println!("{}", test_pos.len());
}
