use crate::square::ChessSquare as Square;
use crate::square::Square as ChessSquare;
use crate::moves::Move;
use crate::moves::get_moves;



#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct Castling {
    king_side: bool,
    queen_side: bool
}

pub struct Position {
    pub rows: [[char; 8]; 8],
    pub to_move: Color,
    pub castling: [Castling; 2],
    pub en_passant: Option<Square>,
    pub halfmove_clock: Option<u32>,
    pub fullmove_number: Option<u32>,
}

impl Position {
    pub fn from_string<S: ToString>(pos: S) -> Option<Position> {
        let mut rows = [['x'; 8]; 8];
        let mut row_index = 0;
        let mut column_index: u32 = 0;
        let mut whitespaces = 0;
        let mut to_move = Color::White;
        let mut castling = [ Castling {
            king_side: false,
            queen_side: false 
        }; 2];
        let mut en_passant_letter = ' ';
        let mut en_passant_integer = 0;
        let mut halfmove_clock: Option<u32> = None;
        let mut tmp_halfmove_clock = "".to_string();
        let mut fullmove_number: Option<u32> = None;
        let mut tmp_fullmove_number = "".to_string();
        for char in pos.to_string().chars() {
            if whitespaces == 0 {
                match char {
                    '/' => {
                        column_index += 1;
                        row_index = 0;
                    }
                    '0'..='8' => {
                        // for number in 0..char.to_digit(10).unwrap() {
                        //    board[column_index as usize][(row_index+char
                        //        .to_digit(10).unwrap() as u32) as usize] = "x";
                        // }
                        row_index += char.to_digit(10).unwrap() as u32;
                    }
                    'a'..'z' => {
                        match char {
                            'p' => {
                                rows[column_index as usize][row_index as usize] = 'p';
                            }
                            'n' => {
                                rows[column_index as usize][row_index as usize] = 'n';
                            }
                            'b' => {
                                rows[column_index as usize][row_index as usize] = 'b';
                            }
                            'r' => {
                                rows[column_index as usize][row_index as usize] = 'r';
                            }
                            'q' => {
                                rows[column_index as usize][row_index as usize] = 'q';
                            }
                            'k' => {
                                rows[column_index as usize][row_index as usize] = 'k';
                            }
                            _ => {
                                let square = Square::from_xy(
                                    column_index as usize as u8,
                                    row_index as usize as u8
                                );
                                panic!(
                                    "invalid piece for white on {}{}",
                                    square.character, square.integer
                                )
                            }
                        }
                        row_index += 1;
                    }
                    'A'..'Z' => {
                        match char {
                            'P' => {
                                rows[column_index as usize][row_index as usize] = 'P';
                            }
                            'N' => {
                                rows[column_index as usize][row_index as usize] = 'N';
                            }
                            'B' => {
                                rows[column_index as usize][row_index as usize] = 'B';
                            }
                            'R' => {
                                rows[column_index as usize][row_index as usize] = 'R';
                            }
                            'Q' => {
                                rows[column_index as usize][row_index as usize] = 'Q';
                            }
                            'K' => {
                                rows[column_index as usize][row_index as usize] = 'K';
                            }
                            _ => {
                                ()
                            }
                        }
                        row_index += 1;
                    }
                    ' ' => {
                        whitespaces += 1;
                    }
                    _ => {
                        panic!("invalid character in position string: '{}'", char);
                    }
                }
            } else if whitespaces == 1 {
                match char {
                    'w' => to_move = Color::White,
                    'b' => to_move = Color::Black,
                     _ => panic!("invalid color: {}", char)
                }
                whitespaces += 1;
            } else if whitespaces == 2 {
                whitespaces += 1;
            } else if whitespaces == 3 {
                match char {
                    'k' => {
                        castling[0].king_side = true;
                    }
                    'q' => {
                        castling[0].queen_side = true;
                    }
                    'K' => {
                        castling[1].king_side = true;
                    }
                    'Q' => {
                        castling[1].queen_side = true;
                    }
                    '-' => {
                        whitespaces += 1;
                    }
                    ' ' => {
                        whitespaces += 1;
                    }
                    _ => {
                        panic!("invalid castling rights: {}", char);
                    }
                }
 
            } else if whitespaces == 4 {
                match char {
                    ' ' => {
                        whitespaces += 1;
                    }
                    'a' => {
                        en_passant_letter = 'a';
                    }
                    'b' => {
                        en_passant_letter = 'b';
                    }
                    'c' => {
                        en_passant_letter = 'c';
                    }
                    'd' => {
                        en_passant_letter = 'd';
                    }
                    'e' => {
                        en_passant_letter = 'e';
                    }
                    'f' => {
                        en_passant_letter = 'f';
                    }
                    'g' => {
                        en_passant_letter = 'g';
                    }
                    'h' => {
                        en_passant_letter = 'h';
                    }
                    '3' => {
                        en_passant_integer = 3;
                    }
                    '6' => {
                        en_passant_integer = 6;
                    }
                    '-' => {
                        whitespaces += 1;
                    }
                    _ => {
                        println!("{}", char);
                        if en_passant_letter == ' ' {
                            panic!("invalid en passant letter: {}", char);
                        } else {
                            panic!("invalid en passant square: {}{}",
                                en_passant_letter, char
                            );
                        }
                    }
                }
            } else if whitespaces == 5 {
                whitespaces += 1;
            } else if whitespaces == 6 {
                match char {
                    '0'..='9' => {
                        if ! tmp_halfmove_clock.is_empty() {
                            halfmove_clock = Some(format!(
                                "{}{}", tmp_halfmove_clock, char)
                                    .parse::<u32>().unwrap()
                            );
                            whitespaces += 1;
                        }
                        tmp_halfmove_clock.push(char);
                    }
                    ' ' => {
                        if tmp_halfmove_clock.len() == 1 {
                            halfmove_clock = Some(
                                tmp_halfmove_clock
                                    .parse::<u32>()
                                    .unwrap()
                            );
                        }
                        whitespaces += 1;
                    }
                    _ => {
                        panic!("invalid halfmove clock: {}", char);
                    }
                }
            } else if whitespaces == 7 {
                match char {
                    '0'..='9' => {
                        if ! tmp_fullmove_number.is_empty() {
                            fullmove_number = Some(format!(
                                "{}{}", tmp_fullmove_number, char)
                                    .parse::<u32>().unwrap()
                            );
                            whitespaces += 1;
                        }
                        tmp_fullmove_number.push(char);
                    }
                    ' ' => {
                        if tmp_fullmove_number.len() == 1 {
                            fullmove_number = Some(
                                tmp_fullmove_number
                                    .parse::<u32>()
                                    .unwrap()
                            );
                        }
                        whitespaces += 1;
                    }
                    _ => {
                        panic!("invalid halfmove clock: {}", char);
                    }
                }
            }
        }
        if en_passant_integer == 0 && en_passant_letter == ' ' {
            return Some(Position {
                rows,
                to_move,
                castling,
                en_passant: None,
                halfmove_clock,
                fullmove_number
            })
        } else {
            return Some(Position {
                rows,
                to_move,
                castling,
                en_passant: Some(Square {
                    character: en_passant_letter,
                    integer: en_passant_integer
                }),
                halfmove_clock,
                fullmove_number
            })
        }
    }

    pub fn to_string(&self) -> String {
        let mut position_string = String::new();
        for row in self.rows.iter() {
            let mut empty_squares = 0;
            for square in row.iter() {
                if *square == 'x' {
                    empty_squares += 1;
                } else {
                    if empty_squares > 0 {
                        position_string.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    position_string.push(*square);
                }
            }
            if empty_squares > 0 {
                position_string.push_str(&empty_squares.to_string());
            }
            position_string.push('/');
        }
        position_string
    }

    pub fn print_board(&self) {
        println!();
        for row in self.rows.iter() {
            for column in row.iter() {
                print!("{} ", column);
            }
            println!();
        }
        println!();
    }

    pub fn print(&self) {
        self.print_board();
        println!("to move: {:?}", self.to_move);
        println!("castling white: Queenside = {:?} | Kingside = {:?}",
            self.castling[0].queen_side,
            self.castling[0].king_side
        );
        println!("castling black: Queenside = {:?} | Kingside = {:?}",
            self.castling[1].queen_side,
            self.castling[1].king_side
        );
        if self.en_passant != None {
            println!("en passant square: {}{}",
                self.en_passant.as_ref().unwrap().character,
                self.en_passant.as_ref().unwrap().integer
            );
        }
        if self.halfmove_clock != None {
            println!("halfmove clock: {:?}", self.halfmove_clock.unwrap());
        }
        if self.fullmove_number != None {
            println!("fullmove number: {:?}", self.fullmove_number.unwrap());
        }
        println!();
    }

    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut x_counter = 0;
        let mut y_counter = 0;
        for row in self.rows {
            for piece in row {
                if piece != 'x' {
                    for piece_move in get_moves(
                        piece, 
                        ChessSquare {
                            x: x_counter,
                            y: y_counter,
                        }
                    ) {
                        moves.push(piece_move);
                        // TODO: make checks for in check and pieces in the way
                    }
                }
                x_counter += 1;
            }
            y_counter += 1;
            x_counter = 0;
        }
        moves
    }

    // fn get_valid(){}
}
