use std::fmt::format;
use std::fs;
use crate::board::board::Board;
use crate::board::condition::Condition::{BLACK, RED};
use crate::board::piece::Piece;

pub struct Eval {
    pub tempo_score: i32,
    pub mobility_mg: [i32; 7],
    pub mobility_eg: [i32; 7],
}

impl Eval {
    // phase constants
    const SoliderPhase: i32 = 0;
    const HorsePhase: i32 = 3;
    const ChariotPhase: i32 = 4;
    const CannonPhase: i32 = 3;
    const ElephantPhase: i32 = 1;
    const AdvisorPhase: i32 = 1;
    const TotalPhase: i32 = Self::SoliderPhase * 10 + Self::HorsePhase * 4 + Self::ChariotPhase * 4 + Self::ElephantPhase * 4 + Self::AdvisorPhase * 4 + Self::CannonPhase * 4;

    // base values, unused
    const BasePieceScore: [i32; 7] = [20, 50, 100, 20, 10, 35, 10];

    
    pub fn new() -> Self {
        Self {
            tempo_score: 0,
            mobility_mg: [0;7],
            mobility_eg: [0;7],
        }
    }

    pub fn evaluate(&self, board: &mut Board) -> i32 {
        let mut mg_eval = board.mg_score[board.player as usize] - board.mg_score[board.player.inverse() as usize];
        let mut eg_eval = board.eg_score[board.player as usize] - board.eg_score[board.player.inverse() as usize];
        
        // also evaluate mobility
        for row in 0..10 {
            for col in 0..9 {
                if board.state[row][col] == Piece::SPACE {
                    continue;
                }
                
                let mut piece = board.state[row][col];
                let mut sign = if piece > 0 && board.player == RED || piece < 0 && board.player == BLACK { 1 } else {-1};
                if sign != 1 {
                    board.player = board.player.inverse();
                }                
                
                let row = row as i8;
                let col = col as i8;
                let piece = piece.abs();
                let mut moves = vec![];
                match piece {
                    Piece::SOLDIER => {
                        board.soldier_moves(row, col, &mut moves);
                        mg_eval += sign*self.mobility_mg[(piece-1) as usize] * (moves.len()) as i32;
                        eg_eval += sign*self.mobility_eg[(piece-1) as usize] * (moves.len()) as i32;
                    },
                    Piece::CANNON => {
                        board.cannon_moves(row, col, &mut moves, 0, 0);
                        mg_eval += sign*self.mobility_mg[(piece-1) as usize] * (moves.len()  as i32 -7);
                        eg_eval += sign*self.mobility_eg[(piece-1) as usize] * (moves.len()  as i32 -7);
                    },
                    Piece::CHARIOT => {
                        board.chariot_moves(row, col, &mut moves, 0, 0);
                        mg_eval += sign*self.mobility_mg[(piece-1) as usize] * (moves.len()  as i32-7);
                        eg_eval += sign*self.mobility_eg[(piece-1) as usize] * (moves.len()  as i32 -7);
                    },
                    Piece::HORSE => {
                        board.horse_moves(row, col, &mut moves, 0,0 );
                        mg_eval += sign*self.mobility_mg[(piece-1) as usize] * (moves.len() as i32-2);
                        eg_eval += sign*self.mobility_eg[(piece-1) as usize] * (moves.len() as i32-2);
                    },
                    _ => {}
                }
                
                if sign != 1 {
                    board.player = board.player.inverse();
                }
            }
        }
        
        // add tempo
        mg_eval += self.tempo_score;
        
        // linear interpolate between mg and eg eval
        // https://www.chessprogramming.org/Tapered_Eval
        let phase = Self::compute_phase(board);
        return ((mg_eval * (256 - phase)) + (eg_eval * phase)) / 256;
    }


    fn compute_phase(board: &mut Board) -> i32 {
        // https://www.chessprogramming.org/Tapered_Eval
        let mut phase = Self::TotalPhase;
        let lookup = [0, Self::AdvisorPhase, Self::CannonPhase, Self::ChariotPhase, Self::ElephantPhase, 0, Self::HorsePhase, Self::SoliderPhase];
        for row in board.state.iter() {
            for cell in row.iter() {
                let piece = cell.abs() as usize;
                phase -= lookup[piece];
            }
        }

        return (phase * 256 + (Self::TotalPhase / 2)) / Self::TotalPhase;
    }
}

impl Eval {
    pub fn load_pst(file: &str) -> (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<i32>>>) {
        let text = fs::read_to_string(file).unwrap();
        
        let rows: Vec<&str> = text.split("\n").collect();
        
        let (mut mg_pst, mut eg_pst) = Eval::create_pst();
        
        let pieces = 7;
        
        // 1..11 for mg
        // 14..24 for eg
        for row in 1..11 {
            let values: Vec<&str> = rows[row].split(",").collect();
            for piece in 0..pieces {
                for col in 0..9 {
                    mg_pst[piece][row-1][col] = values[9*piece + col].trim().parse::<i32>().unwrap();
                }
            }    
        }

        for row in 14..24 {
            let values: Vec<&str> = rows[row].split(",").collect();
            for piece in 0..pieces {
                for col in 0..9 {
                    eg_pst[piece][row-14][col] = values[9*piece + col].trim().parse::<i32>().unwrap();
                }
            }
        }
        

        (mg_pst, eg_pst)
    }
    
    pub fn create_pst() -> (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<i32>>>) {
        // 7 pieces, 10 by 9 board, [piece, row, col]
        let mut mgpst = vec![vec![vec![0; 9]; 10]; 7];
        for (piece, val) in Self::BasePieceScore.iter().enumerate() {
            for row in 0..10 {
                for col in 0..9 {
                    mgpst[piece][row][col] = *val;
                }
            }
        }
        
        let mut egpst = mgpst.clone();
        (mgpst, egpst)
    }

    pub fn display_pst(pst: &Vec<Vec<Vec<i32>>>) -> String {
        let title = ["Advisor", "Cannon", "Chariot", "Elephant", "General", "Horse", "Soldier"];
        let mut text = vec![];

        for (piece, board) in pst.iter().enumerate() {
            let mut piece_text = String::new();
            piece_text += title[piece];
            piece_text += "\n";

            for row in board.iter() {
                for cell in row.iter() {
                    piece_text += &format!("{},", cell);
                }
                piece_text += "\n";
            }
            text.push(piece_text.split("\n").map(|t| t.to_string()).collect::<Vec<String>>());
        }

        // vertically stack piece_text
        let max_width = 40;
        let mut output = String::new();
        let height = text[0].len();
        for i in 0..height {
            for k in text.iter() {
                output += &format!("{: <width$}", k[i], width=max_width);
            }
            output += "\n";
        }

        output
    }

 
}