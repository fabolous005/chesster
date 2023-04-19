pub struct ChessSquare {
    pub character: char,
    pub integer: u8
}

impl ChessSquare {
    pub fn from_xy(x: u8, y: u8) -> ChessSquare {
        let mut character: char = 'x';
        match x {
            0 => character = 'a',
            1 => character = 'b',
            2 => character = 'c',
            3 => character = 'd',
            4 => character = 'e',
            5 => character = 'f',
            6 => character = 'g',
            7 => character = 'h',
            _ => panic!("invalid x coordinate for square\nx coordinate: {}", x)
        }
        if y > 8 {
            panic!("invalid y coordinate for square\ny coordinate: {}", y);
        }
        ChessSquare { character, integer: y + 1 }
    }
}


pub struct Square {
    pub x: u8,
    pub y: u8
}

impl Square {
    pub fn from_xy(x: u8, y: u8) -> Square {
        Square { x, y }
    }
}
