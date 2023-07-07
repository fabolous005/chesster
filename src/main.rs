#![feature(exclusive_range_pattern)]
#![allow(dead_code)]

mod square;
mod position;
mod moves;
mod pieces;
mod evaluate;

use std::collections::HashMap;

use crate::position::Position;
use crate::evaluate::evaluate::{analyze_move, find_best, analyze_moves};
use crate::moves::Move;

use std::thread;
use std::sync::mpsc;


fn main() {
    /*
    let example_positions = vec![
        "r2q1b2/p1k5/1p1p4/2pQ4/4P3/2B5/P1PP2PP/R3K1NR b KQ - 1 21",
        "6k1/1p2p1bp/1R3pp1/8/4B3/8/5PPP/2Br2K1 w - -",
        "r1b1kb1r/pp6/3p1p2/2pPp2p/q3N3/8/P1PB1PPP/R2QKB1R w KQkq - 2 13",
        "r1b1kb1r/pp3pp1/2p2n1p/q3N3/N2pP3/1B1P4/PPP2PPP/R2Q1RK1 b kq - 0 11",
        "1k1r2nr/p4pbp/2Qp1p2/8/N3P3/1B3N2/P4PPP/R3K2R w KQ - 1 17",
        "3r2nr/p1k2pbp/Q2p4/3B4/N3p3/5N2/P4PPP/1R2K2R w K - 2 20"
    ];
    */

    let (sender_main, receiver_main) = mpsc::channel();
    let (sender_sub, receiver_sub) = mpsc::channel();


    let exmaple_position = "r2q1b2/p1k5/1p1p4/2pQ4/4P3/2B5/P1PP2PP/R3K1NR b KQ - 1 21";

    
    let position = Position::from_string(exmaple_position).unwrap();
    let moves = position.get_moves();
    println!("{:#?}", moves);

    let mut moves_value = Vec::new();
    for move_ in moves.clone() {
        moves_value.push(analyze_move(move_, position.clone()));
    }
    let map = moves.into_iter().zip(moves_value.into_iter()).collect::<HashMap<_, _>>();
    for move_ in map.keys() {
        analyze_moves(position.clone().make_move(move_).get_moves().as_ref(), position.clone());
    }

    let best_move = find_best(map);
    println!("{:#?}", best_move);
    
    thread::spawn(move || {
        loop{
            let result: HashMap<Move, i32> = get_lines();
            sender_main.send(result).unwrap();
            let position: Position = receiver_sub.recv().unwrap();
        }
    });

    let result = receiver_main.recv().unwrap();


    /*
    for pos in positions {
        pos.print();
        let moves = pos.get_moves();
        println!("{:#?}", moves);
        println!("{}", moves.len());
    }
    */



    /*
    let test_pos_b = Position::from_string(
        "r2q1b2/p1k5/1p1p4/2pQ4/4P3/2B5/P1PP2PP/R3K1NR b KQ - 1 21"
        // "8/PP6/8/8/8/8/8/8 w - - 0 1"
        // Pawn1 4 + 4 = 8
        // Pawn2 4 + 4 + 4 = 12
        // = 20 "valid" moves
    ).unwrap();
    test_pos_b.print();
    let test_pos = test_pos_b.get_moves();
    println!("{:#?}", test_pos);
    println!("{}", test_pos.len());
    */
}
