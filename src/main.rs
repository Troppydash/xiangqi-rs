use crate::board::board::Board;
use crate::board::movee::Move;
use crate::board::piece::Piece;
use crate::engine::search::Engine;
use crate::server::socket;
use crate::server::socket::serve;

mod board;
mod engine;
mod server;


fn test_pos1() {
    let moves = "B3E3,H8H4,H3HX,IXHX,H1G3,HXH5,A1A3,B8E8,I1I3,BXC8,B1C3,AXBX,G4G5,H5G5,I3H3,H4C4,G1I3,C4C1,D1E2,G5G4,E2F3,BXB2,H3HX,B2C2,C3D5";
    let mut board = Board::new();
    let moves: Vec<&str> = moves.split(",").collect::<Vec<&str>>();
    let moves = moves.iter().map(|st| Move::from_string(&st.to_string())).collect::<Option<Vec<Move>>>();
    let mut moves = moves.unwrap();

    for mov in moves.iter_mut() {
        board.try_move(mov);
    }

    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 16, 1000000);
}

fn test_pos2() {
    let moves = "H3HX,B8E8,B3H3,AXA8,A1A3,BXC8,A3F3,DXE9,F3F7,H8H7,F7G7,H7H1,HXH1,IXI8,H3H6,C7C6,H6G6,EXDX,G6GX,DXD9,H1H9,E9F8,I1I3,C6C5,I3D3,C8D6,GXG9,D9DX,D3D6,E8D8,H9HX,DXD9,G9H9,A8B8,G7G9,FXE9";
    let mut board = Board::new();
    let moves: Vec<&str> = moves.split(",").collect::<Vec<&str>>();
    let moves = moves.iter().map(|st| Move::from_string(&st.to_string())).collect::<Option<Vec<Move>>>();
    let mut moves = moves.unwrap();

    for mov in moves.iter_mut() {
        board.try_move(mov);
    }

    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 13, 1000000000);
}

fn test_pos3() {
    let moves = "H3HX,IXHX,I1I3,H8E8,H1G3,HXH2,I3I2,H2I2,G3I2,B8B4,B3E3,B4E4,D1E2,E4I4,I2G3,I4I5,B1C3,BXC8,A1B1,AXBX,B1BX,C8BX,G4G5,BXC8,G3H5,I5G5,H5G7,E8E3,C1E3,G5G3,C3E4,G3G4,G7F9,DXE9,E4D6,C8DX,F9E7,DXE8,A4A5,E9DX,D6B5,E8G7,B5A7,G7E6,E7G6,I7I6,G6H8,I6I5,H8GX,G4G8,GXI9,G8E8,I9H7,E6G7,A7B5,C7C6,H7I5,C6C5,C4C5,G7E6,I5H7,EXE9,C5C6,E6F4,E1D1,E8E4,A5A6,E4E5,A6A7,CXE8,C6C7,E8C6,H7G9,E5D5,G9F7,E9D9,B5C3,D5D4,D1D2,DXE9,C3B5,D4D5,B5C3,D5D4,C7C8,D9DX,C3B5,D4D5,B5C3,D5D4,C3B5,D4D5,C8C9,C6E8,B5C7,E9F8";
    let mut board = Board::new();
    let moves: Vec<&str> = moves.split(",").collect::<Vec<&str>>();
    let moves = moves.iter().map(|st| Move::from_string(&st.to_string())).collect::<Option<Vec<Move>>>();
    let mut moves = moves.unwrap();

    for mov in moves.iter_mut() {
        board.try_move(mov);
    }

    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 15, 1000000);
}

fn test_basic() {
    let mut board = Board::new();
    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 15, 2000000);
}

fn test_pos4() {
    // level 7 leads to -m2, B8B9
    let moves = "B3BX,H8E8,H3B3,IXI8,I1I3,HXG8,I3D3,FXE9,D3D7,B8B7,D7C7,B7B1,BXB1,AXA8,B3E3,I8H8,H1I3,E8E4,F1E2,GXE8,B1B7,H8H6,A1B1,H6A6,B7B8,A8B8,B1B8,EXFX";
    let mut board = Board::new();
    let moves: Vec<&str> = moves.split(",").collect::<Vec<&str>>();
    let moves = moves.iter().map(|st| Move::from_string(&st.to_string())).collect::<Option<Vec<Move>>>();
    let mut moves = moves.unwrap();

    for mov in moves.iter_mut() {
        board.try_move(mov);
    }

    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 15, 1000000);
}

fn start_ws() {
    serve();
}

fn main() {
    start_ws();
    // test_pos1();
    // test_basic();
    // test_pos3();
    // test_pos4();

    // let mut board = Board::new();
    // println!("{}", board.display());
    //

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
