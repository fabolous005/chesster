use crate::position::Position;
use crate::moves::Move;
use crate::evaluate::material::material;

pub fn evaluate(position: Position) -> i32 {
	todo!();
	// this function should get all available moves and evaluate each move and the state of the current position
	// including castling rights, material, piece square tables, and pawn structure
}


pub fn evaluate_move(move_: Move) -> i32 {
	let mut value: i32 = 0;
	let mut buf = 0;

	let buf = material(move_);
	let value = value + buf;
	value

}
