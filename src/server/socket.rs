use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::{accept, WebSocket};
use serde::{Deserialize, Serialize};
use futures::executor::ThreadPool;
use futures::task::SpawnExt;
use crate::board::board::Board;
use crate::board::movee::Move;
use crate::engine::eval::Eval;
use crate::engine::search::Engine;

#[derive(Serialize, Deserialize)]
struct Instruct {
    // can be: analyze
    method: String,

    // analyze
    moves: Vec<String>,
    limit: i32,
}

#[derive(Serialize, Deserialize)]
struct Response {
    // can be: analyze
    method: String,
    
    // analyze
    best_move: String,
    score: i32
}

fn analyze_board(websocket: &mut WebSocket<TcpStream>, instruct: &Instruct) {
    // parse moves
    let moves = instruct.moves.iter().map(|s| Move::from_string(&s)).collect::<Option<Vec<Move>>>();
    if let None = moves {
        websocket.send("failed to parse move list".into()).unwrap();
        return;
    }

    // execute moves
    let mut moves = moves.unwrap();
    let mut board = Board::new();
    let (mg_pst, eg_pst) = Eval::load_pst("./required/pst.txt");
    board.load_pst(mg_pst, eg_pst);
    
    for mov in moves.iter_mut() {
        if !board.try_move(mov) {
            websocket.send("failed to execute move list".into()).unwrap();
            return;
        }
    }

    println!("{}", board.display());
    println!("{}", moves.iter().map(Move::display).collect::<Vec<String>>().join(","));

    let mut engine = Engine::new();

    // run analysis
    let (best_move, score) = engine.search(&mut board, 50, instruct.limit);
    let response = Response {
        method: "analyze".to_string(),
        score: score,
        best_move: best_move.display(),
    };
    
    websocket.send(serde_json::to_string(&response).unwrap().into()).unwrap();
}

fn handle_connection(mut websocket: WebSocket<TcpStream>) {

    loop {
        let msg = websocket.read();
        if let Err(_) = msg {
            // connection closed
            break;
        }
        let msg = msg.unwrap();

        if msg.is_binary() || msg.is_text() {
            // parse connection string
            let text = msg.to_text().unwrap();
            let result = serde_json::from_str(text);
            if let Err(_) = result {
                websocket.send("cannot parse json".into()).unwrap();
                continue;
            }

            let instruct: Instruct = result.unwrap();
            match instruct.method.as_str() {
                "analyze" => {
                    analyze_board(&mut websocket, &instruct);

                    // TODO: test pondering
                    
                }

                _ => {}
            }
        }
    }
}


pub fn serve() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() == 2  {
        args[1].to_string()
    } else {
        "3030".to_string()
    };
    
    println!("websocket started on port {}", port);

    let server = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    let pool = ThreadPool::builder()
        .pool_size(10)
        .create().expect("failed to create thread pool");

    for stream in server.incoming() {
        pool.spawn(async {
            // handle
            let websocket = accept(stream.unwrap()).unwrap();
            handle_connection(websocket);
        }).unwrap();
    }
}
