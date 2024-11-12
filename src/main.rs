use rand::Rng;
use crate::board::board::Board;
use crate::board::movee::Move;
use crate::board::piece::Piece;
use crate::engine::eval::Eval;
use crate::engine::search::Engine;
use crate::engine::training;
use crate::engine::training::save_db;
use crate::server::socket;
use crate::server::socket::serve;

mod board;
mod engine;
mod server;


fn test_pos1() {
    let moves = "B3E3,H8H4,H3HX,IXHX,H1G3,HXH5,A1A3,B8E8,I1I3,BXC8,B1C3,AXBX,G4G5,H5G5,I3H3,H4C4,G1I3,C4C1,D1E2,G5G4,E2F3,BXB2,H3HX,B2C2,C3D5";
    let mut board = Board::new();
    let (mg_pst, eg_pst) = Eval::load_pst("./required/pst2.txt");
    board.load_pst(mg_pst, eg_pst);
    let moves: Vec<&str> = moves.split(",").collect::<Vec<&str>>();
    let moves = moves.iter().map(|st| Move::from_string(&st.to_string())).collect::<Option<Vec<Move>>>();
    let mut moves = moves.unwrap();

    for mov in moves.iter_mut() {
        board.try_move(mov);
    }

    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 16, 20000000);
}

fn test_pos2() {
    let moves = "H3HX,B8E8,B3H3,AXA8,A1A3,BXC8,A3F3,DXE9,F3F7,H8H7,F7G7,H7H1,HXH1,IXI8,H3H6,C7C6,H6G6,EXDX,G6GX,DXD9,H1H9,E9F8,I1I3,C6C5,I3D3,C8D6,GXG9,D9DX,D3D6,E8D8,H9HX,DXD9,G9H9,A8B8,G7G9,FXE9";
    let mut board = Board::new();
    let mut board = Board::new();
    let (mg_pst, eg_pst) = Eval::load_pst("./required/pst2.txt");
    board.load_pst(mg_pst, eg_pst);
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
    let (mg_pst, eg_pst) = Eval::load_pst("./required/pst2.txt");
    board.load_pst(mg_pst, eg_pst);
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
    let (mg_pst, eg_pst) = Eval::load_pst("./required/pst2.txt");
    board.load_pst(mg_pst, eg_pst);
    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 15, 4000000);
}

fn test_pos4() {
    // level 7 leads to -m2, B8B9
    let moves = "B3BX,H8E8,H3B3,IXI8,I1I3,HXG8,I3D3,FXE9,D3D7,B8B7,D7C7,B7B1,BXB1,AXA8,B3E3,I8H8,H1I3,E8E4,F1E2,GXE8,B1B7,H8H6,A1B1,H6A6,B7B8,A8B8,B1B8,EXFX";
    let mut board = Board::new();
    let (mg_pst, eg_pst) = Eval::load_pst("./required/pst2.txt");
    board.load_pst(mg_pst, eg_pst);
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

fn test_pos5() {
    let moves = "H3H7,G7G6,B3E3,HXG8,B1C3,BXC8,A1B1,G8F6,C4C5,IXI9,E3G3,AXBX,G4G5,G6G5,G3GX,FXE9,H7C7,E7E6,B1B6,I9IX,GXG6,C8E7,G6E6,B8G8,B6BX,G8G1,E1E2,G1I1,BXCX,H8E8,E6E8,F6E8,C7B7,E9FX,B7BX,E7F5,CXC9,DXE9,C9CX,E9DX,CXC8,EXE9,C8A8,E8C7,A8H8,F5D4,E2E1,D4C2,E1E2,I1F1,C1E3,IXGX,H8H7,C7D9,BXAX,F1F8,H7H9,F8F9,C3D5,G5F5,H1I3,C2D4,E2D2,GXG3,D2D3,D4B3,AXA9,D9B8,A9F9,B8A6,D5C3,A6C5,D3D2,C5E4,F9F7,E9EX,H9D9,E4C3,D9DX";
    let mut board = Board::new();
    let (mg_pst, eg_pst) = Eval::load_pst("./required/pst2.txt");
    board.load_pst(mg_pst, eg_pst);
    let moves: Vec<&str> = moves.split(",").collect::<Vec<&str>>();
    let moves = moves.iter().map(|st| Move::from_string(&st.to_string())).collect::<Option<Vec<Move>>>();
    let mut moves = moves.unwrap();

    for mov in moves.iter_mut() {
        if !board.try_move(mov) {
            panic!("oh no");
        }
    }

    println!("{}", board.display());
    let mut engine = Engine::new();
    engine.search(&mut board, 16, 20000000);
}

fn start_ws() {
    serve();
}

fn test(msg: &mut Vec<i32>) {
    // pass
    msg.push(12);
}

fn show(msg: &Vec<i32>) {
    println!("{}", msg.len());
}

fn main() {
    // let db = training::create_db("/media/terry/Games/projects/2024/mlprojects/xiangqi-rs/data/output2");
    // save_db("/media/terry/Games/projects/2024/mlprojects/xiangqi-rs/data/parsed.txt", &db);
    // let mut rng = rand::thread_rng();
    // println!("{}", rng.random::<f64>());
    // let db = training::read_db("/media/terry/Games/projects/2024/mlprojects/xiangqi-rs/data/parsed.txt");
    // training::tune_pst(&db);
    // 
    
    // let num: u32 = 12;
    // let mut board = Board::new();
    // let moves = "C2=5,n8+7,N2+3,p7+1,P7+1,n2+3,N8+7,n7+8,R1+1,b3+5,R1=4,a4+5,C8=9,p9+1,R9=8,c2=1,N7+6,r1=4,R4+3,r9+3,C5=6,r4=1,N6+7,p5+1,C9=7,r9=4,N7-6,p7+1,R4=3,r4=6,P7+1,r1=2,R8+9,n3-2,B3+5,c1+4,P7=6,n8+6,R3=2,c8=7,P6=5,c1-1,R2-3,r6=2,P3+1,c7+5,C6=3,r2=4,N6+8,r4+3,R2=9,p1+1,-P+1,n2+4,R9+2,r4=1,N8-9,n4+3,C3=4,c1=2,N9+8,c2+4,N8-7,n3+4,A4+5,n6+5,N7-5,n4+5,+P=4,n5-4,C7+2,n4+3,C4=1,c2-4,C7-1,c2=7,C1+3,p1+1,B7+5,c7+1,A5+6,p1=2,-A+5,p2+1,C7=5,n3-4,C1+4,p2=3,B5+7,n4-3,P5+1,p3=4,C5+1";
    // let moves = moves.split(",").collect::<Vec<&str>>();
    // for mov in moves {
    //     println!("{}", mov);
    //     let mut actual = board.parse_move(mov.to_string()).expect("Invalid move");
    //     board.mov(&mut actual);
    //     println!("Move {}\n{}", actual.display(), board.display());
    // }
    // 

    start_ws();
    // test_pos1();
    // test_basic();
    // test_pos3();
    // test_pos4();
    // test_pos5();

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
