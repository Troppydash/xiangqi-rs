use crate::board::board::Board;
use crate::board::movee::Move;
use crate::board::piece::Piece;

mod board;
mod engine;

fn main() {
    let mut board = Board::new();
    println!("{}", board.display());
    
    let mut total = 0;
    for _ in 0..10000000 {
        let moves = board.get_moves(false);
        total += moves.len();
        
        if total / 44 % 1000 == 0 {
            print!("\r{}", total/44);
        }
    }
    
  
    // let mov = Move::new(0, 0, 0, 1);
    // let mov2 = Move::new(0, 0, 0, 1);
    // println!("Move {}, Hash {}", mov.display(), mov.equals(&mov2));
    // println!("Piece {}", Piece::display(-Piece::ADVISOR));
}
