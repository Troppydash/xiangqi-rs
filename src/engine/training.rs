use std::fs;
use rand::Rng;
use crate::board::board::Board;
use crate::board::condition::Condition;
use crate::board::condition::Condition::{BLACK, NONE, RED};
use crate::board::movee::Move;
use crate::board::piece::Piece;
use crate::engine::eval::Eval;
use crate::engine::search::Engine;


pub struct Game {
    score: f64,
    moves: Vec<Move>,
}

fn sigmoid(s: f64, k: f64) -> f64 {
    1.0 / (1.0 + f64::powf(10.0, -k * s / 400.0))
}

pub fn create_db(folder: &str) -> Vec<Game> {
    let paths = fs::read_dir(folder).unwrap();
    let mut games = vec![];
    for p in paths {
        print!("\r{}", games.len());
        let content = fs::read_to_string(p.unwrap().path()).unwrap();

        // parse content
        // id \n red win? \n moves
        let rows: Vec<&str> = content.split("\n").collect();
        let mut winner = NONE;
        let score = match rows[1] {
            "WIN" => {
                winner = RED;
                1.0
            }
            "DRAW" => {
                winner = NONE;
                0.5
            }
            "LOSS" => {
                winner = BLACK;
                0.0
            }
            "?" => {
                println!("skipping id {}", rows[0]);
                continue;
            }
            _ => {
                println!("{}", rows[1]);
                panic!("uh oh");
            }
        };
        let moves: Vec<&str> = rows[2].split(",").collect();


        let mut ok = true;
        let mut parsed_moves = vec![];
        let mut board = Board::new();
        for mov in moves {
            let mut mv = board.parse_move(mov.to_string());
            if mv.is_none() {
                if board.player != winner {
                    // assume it is a loss for them and they threw
                    break;
                }

                // only care about non length errors
                if !(mov.len() != 4 && mov.len() != 5) {
                    println!("move parse error {}\n    id {}\n    {}\n{}", mov, rows[0], rows[2], board.display());
                }
                ok = false;
                break;
            }

            let mut mv = mv.unwrap();

            if board.condition() == Condition::DRAW {
                println!("draw error {}\n    id {}", mov, rows[0]);
                ok = false;
                break;
            }

            if !board.try_move(&mut mv) {
                if board.player != winner {
                    // assume it is a loss for them and they threw
                    break;
                }

                println!("move execute error {}\n    id {}\n    {}\n{}", mov, rows[0], rows[2], board.display());

                ok = false;
                break;
            }

            parsed_moves.push(mv.clone());
        }

        if !ok {
            continue;
        }

        games.push(Game { score, moves: parsed_moves });
    }

    games
}

pub fn read_db(file: &str) -> Vec<Game> {
    let text = fs::read_to_string(file).unwrap();
    let mut games = vec![];

    for line in text.split("\n") {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let cols = line.split("|").collect::<Vec<&str>>();
        let score = cols[0].parse::<f64>().unwrap();
        let moves: Vec<Move> = cols[1].split(",").filter_map(|t| {
            let val = Move::from_string(t);
            // if val.is_none() {
            //     println!("Problem {} {}", t, line);
            // }

            val
        }).collect();
        games.push(Game { score, moves });
    }


    println!("loaded {} entries", games.len());

    games
}

pub fn save_db(file: &str, db: &Vec<Game>) {
    let mut text = "".to_string();
    for g in db.iter() {
        let combined = g.moves.iter().map(|m| m.display()).collect::<Vec<String>>().join(",");
        text += &format!("{}|{}\n", g.score, combined);
    }

    fs::write(file, text).expect("Unable to write file");
}

pub fn find_k(db: &Vec<Game>) {
    let mut engine = Engine::new();

    println!("Counting positions...");
    let mut total = 0;
    for game in db.iter() {
        let mut board = Board::new();
        for mov in game.moves.iter() {
            let mut mov = mov.clone();
            if !board.try_move(&mut mov) {
                panic!("uh oh");
            }
            total += 1;
        }
    }
    println!("Found {} positions", total);

    let mut rng = rand::thread_rng();
    let mut k = 4.0;
    let mut best_k = k;
    let mut best_score = 1e9;
    loop {
        println!("Trying {}, Best {} {}", k, best_k, best_score);
        // score = 1/n sum (score_i - sigmoid(qi))^2
        let mut score = 0.0;
        let mut counts = 0;
        for game in db.iter().take(90000 / 1) {
            let mut board = Board::new();
            for mov in game.moves.iter() {
                if counts % 10000 == 0 {
                    print!("\rCount {} / {}", counts, total);
                }

                let mut mov = mov.clone();
                if !board.try_move(&mut mov) {
                    panic!("uh oh");
                }

                // compute qi
                // let qi = engine.qsearch(&mut board, -1e9 as i32, 1e9 as i32, &mut vec![], 0, 0) as f32;
                let mut qi = engine.evaluate(&mut board, 0) as f64;
                // qi is from the current player's perspective, we want the red's perspective
                if board.player == BLACK {
                    qi = -qi;
                }
                score += ((game.score - sigmoid(qi, k)) * (game.score - sigmoid(qi, k))) / total as f64;
                counts += 1;
            }
        }

        println!("\nscore {}", score);
        if score < best_score {
            best_score = score;
            best_k = k;
        } else {
            k = best_k;
        }

        // change k
        k = k + ((rng.random::<f64>()) - 0.5) * 2.0;
        k = k.clamp(0.01, 1000.0);
    }
}

pub fn tune_pst(db: &Vec<Game>) {
    let mut engine = Engine::new();
    let k = 4.314075670609904;
    println!("Counting positions...");
    let mut total = 0;
    for game in db.iter() {
        let mut board = Board::new();
        for mov in game.moves.iter() {
            let mut mov = mov.clone();
            if !board.try_move(&mut mov) {
                panic!("uh oh");
            }
            total += 1;
        }
    }
    println!("Found {} positions", total);

    let mut rng = rand::thread_rng();
    let (mut mg_pst, mut eg_pst) = Eval::create_pst();
    let mut best_state = (mg_pst.clone(), eg_pst.clone());
    let mut best_score = 1e9;

    let mut mg_rng_piece = rng.gen_range(0..7);
    let mut mg_rng_row = rng.gen_range(0..10);
    let mut mg_rng_col = rng.gen_range(0..9);
    let mut mg_rng_dir = 0;

    let mut eg_rng_piece = rng.gen_range(0..7);
    let mut eg_rng_row = rng.gen_range(0..10);
    let mut eg_rng_col = rng.gen_range(0..9);
    let mut eg_rng_dir = 0;

    loop {
        println!("Best {}\nMG\n{}EG\n{}", best_score, Eval::display_pst(&best_state.0), Eval::display_pst(&best_state.1));
        // score = 1/n sum (score_i - sigmoid(qi))^2
        let mut score = 0.0;
        let mut counts = 0;
        for game in db.iter().take(90000 / 5) {
            let mut board = Board::new();
            board.load_pst(mg_pst.clone(), eg_pst.clone());

            for mov in game.moves.iter() {
                if counts % 10000 == 0 {
                    print!("\rCount {} / {}", counts, total);
                }

                let mut mov = mov.clone();
                if !board.try_move(&mut mov) {
                    panic!("uh oh");
                }

                // compute qi
                let mut qi = engine.evaluate(&mut board, 0) as f64;
                // qi is from the current player's perspective, we want the red's perspective
                if board.player == BLACK {
                    qi = -qi;
                }
                score += ((game.score - sigmoid(qi, k)) * (game.score - sigmoid(qi, k))) / total as f64;
                counts += 1;
            }
        }

        println!("\nscore {}", score);
        if score < best_score {
            best_score = score;
            best_state = (mg_pst.clone(), eg_pst.clone());
            
            let text = format!("{}\n{}", Eval::display_pst(&best_state.0), Eval::display_pst(&best_state.1));
            fs::write("/media/terry/Games/projects/2024/mlprojects/xiangqi-rs/data/boards.txt", text).unwrap();
        } else {
            mg_pst[mg_rng_piece][mg_rng_row][mg_rng_col] -= mg_rng_dir;
            if mg_rng_col != 4 {
                mg_pst[mg_rng_piece][mg_rng_row][8 - mg_rng_col] -= mg_rng_dir;
            }
            eg_pst[eg_rng_piece][eg_rng_row][eg_rng_col] -= eg_rng_dir;
            if eg_rng_col != 4 {
                eg_pst[eg_rng_piece][eg_rng_row][8 - eg_rng_col] -= eg_rng_dir;
            }
        }

        // change pst
        mg_rng_piece = rng.gen_range(0..7);
        loop {
            mg_rng_row = rng.gen_range(0..10);
            mg_rng_col = rng.gen_range(0..9);
            mg_rng_dir = if rng.gen_range(0..2) == 0 { -1 } else { 1 };

            if mg_rng_piece as i8 == Piece::GENERAL - 1 || mg_rng_piece as i8 == Piece::ADVISOR - 1 {
                if !(7 <= mg_rng_row && mg_rng_row <= 9 && 3 <= mg_rng_col && mg_rng_col <= 5) {
                    continue;
                }
            }
            if mg_rng_piece as i8 == Piece::ELEPHANT - 1 {
                if !(5 <= mg_rng_row && mg_rng_row <= 9) {
                    continue;
                }
            }

            if mg_rng_piece as i8 == Piece::SOLDIER - 1 {
                if mg_rng_row >= 7 {
                    continue;
                }
            }

            break;
        }

        mg_pst[mg_rng_piece][mg_rng_row][mg_rng_col] += mg_rng_dir;
        if mg_rng_col != 4 {
            mg_pst[mg_rng_piece][mg_rng_row][8 - mg_rng_col] += mg_rng_dir;
        }


        eg_rng_piece = rng.gen_range(0..7);
        loop {
            eg_rng_row = rng.gen_range(0..10);
            eg_rng_col = rng.gen_range(0..9);
            eg_rng_dir = if rng.gen_range(0..2) == 0 { -1 } else { 1 };

            if eg_rng_piece as i8 == Piece::GENERAL - 1 || eg_rng_piece as i8 == Piece::ADVISOR - 1{
                if !(7 <= eg_rng_row && eg_rng_row <= 9 && 3 <= eg_rng_col && eg_rng_col <= 5) {
                    continue;
                }
            }
            if eg_rng_piece as i8 == Piece::ELEPHANT - 1 {
                if !(5 <= eg_rng_row && eg_rng_row <= 9) {
                    continue;
                }
            }

            if eg_rng_piece as i8 == Piece::SOLDIER - 1 {
                if eg_rng_row >= 7 {
                    continue;
                }
            }

            break;
        }
        eg_pst[eg_rng_piece][eg_rng_row][eg_rng_col] += eg_rng_dir;
        if eg_rng_col != 4 {
            eg_pst[eg_rng_piece][eg_rng_row][8 - eg_rng_col] += eg_rng_dir;
        }
    }
}

//
// pub fn train(folder: String) {
//     let paths = fs::read_dir(folder).unwrap();
// 
//     // read files
//     let mut count = 0;
//     for p in paths {
//         let content = fs::read_to_string(p.unwrap().path()).unwrap();
//         println!("{}", content);
//         // parse content
//         // id \n red win? \n moves
// 
//         // csv format
//         // moves, {0,0.5,1}
// 
//         let rows: Vec<&str> = content.split("\n").collect();
//         let score = match rows[0] {
//             "WIN" => 1.0,
//             "DRAW" => 0.5,
//             "LOSS" => 0.0,
//             _ => panic!("uh oh")
//         };
//         
//         // score = 1/n sum (score_i - sigmoid(qi))^2
//         
//         // first iteration get K
//         
//         
//         count += 1;
//     }
//     println!("{}", count);
// }