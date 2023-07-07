use crate::position::Position;
use crate::moves::Move;
use crate::evaluate::material::material;
use crate::evaluate::squares::bishop::evaluate_bishop;

pub fn evaluate(position: &Position) -> i32 {
	let mut value: i32 = 0;
	let value = evaluate_bishop();
	// TODO: this function should get all available moves and evaluate each move and the state of the current position
	// including castling rights, material, piece square tables, and pawn structure
	value
}


pub fn evaluate_move(move_: Move) -> i32 {
	let mut value: i32 = 0;
	let mut buf = 0;

	let value = value + buf;
	value

}

pub fn analyze_moves(moves: &[Move], position: Position) -> Vec<i32> {
    let mut evaluations = Vec::with_capacity(moves.len());
	for move_ in moves {
        let mut new_position = position;
        new_position.make_move(move_);
        evaluations.push(evaluate(&new_position));
    }
    evaluations
}
