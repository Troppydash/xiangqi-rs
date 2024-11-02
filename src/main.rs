use crate::board::board::Board;
use crate::board::movee::Move;
use crate::board::piece::Piece;
use crate::engine::search::Engine;
use crate::server::socket;
use crate::server::socket::serve;

mod board;
mod engine;
mod server;


fn main() {
    // let mut board = Board::new();
    // let moves = "B3E3,H8H4,H3HX,IXHX,H1G3,HXH5,A1A3,B8E8,I1I3,BXC8,B1C3,AXBX,G4G5,H5G5,I3H3,H4C4,G1I3,C4C1,D1E2,G5G4,E2F3,BXB2,H3HX,B2C2,C3D5";
    // let moves: Vec<&str> = moves.split(",").collect::<Vec<&str>>();
    // let moves = moves.iter().map(|st| Move::from_string(&st.to_string())).collect::<Option<Vec<Move>>>();
    // let mut moves = moves.unwrap();
    // 
    // for mov in moves.iter_mut() {
    //     board.try_move(mov);
    // }
    // 
    // println!("{}", board.display());
    // 
    // 
    serve();
    
    return;
    
    
    
    // let mut board = Board::new();
    // println!("{}", board.display());
    // 
    // let mut engine = Engine::new();
    // engine.search(&mut board, 10, 1000000);
    // // 
    // let mut total = 0;
    // // for _ in 0..10000000 {
    //     let moves = board.get_moves(false);
    //     total += moves.len();
    //     
    //     if total / 44 % 1000 == 0 {
    //         print!("\r{}", total/44);
    //     }
    // // }
    
  
    // let mov = Move::new(0, 0, 0, 1);
    // let mov2 = Move::new(0, 0, 0, 1);
    // println!("Move {}, Hash {}", mov.display(), mov.equals(&mov2));
    // println!("Piece {}", Piece::display(-Piece::ADVISOR));
}
