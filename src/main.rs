use crate::board::board::Board;
use crate::board::movee::Move;
use crate::board::piece::Piece;

mod board;

fn main() {
    let mut board = Board::new();
    println!("{}", board.display());
    
    let mut total = 0;
    for _ in 0..1000000 {
        let moves = board.get_moves(false);
        total += moves.len();
    }
    
  
    // let mov = Move::new(0, 0, 0, 1);
    // let mov2 = Move::new(0, 0, 0, 1);
    // println!("Move {}, Hash {}", mov.display(), mov.equals(&mov2));
    // println!("Piece {}", Piece::display(-Piece::ADVISOR));
}
