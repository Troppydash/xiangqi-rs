pub struct Piece;

impl Piece {
    pub const SPACE: i8 = 0;
    pub const ADVISOR: i8 = 1;
    pub const CANNON: i8 = 2;
    pub const CHARIOT: i8 = 3;
    pub const ELEPHANT: i8 = 4;
    pub const GENERAL: i8 = 5;
    pub const HORSE: i8 = 6;
    pub const SOLDIER: i8 = 7;
    
    pub fn display(piece: i8) -> String {
        let symbols = [' ', 'A', 'C', 'R', 'E', 'G', 'H', 'S'];
        let mut ch = symbols[piece.abs() as usize];
        if piece < 0 {
            ch = ch.to_ascii_lowercase();
        }

        ch.to_string()
    }
}