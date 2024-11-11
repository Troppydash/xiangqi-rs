use std::fmt::format;
use std::fs;
use crate::board::board::Board;
use crate::board::piece::Piece;

pub struct Eval;

impl Eval {
    // phase constants
    const SoliderPhase: i32 = 0;
    const HorsePhase: i32 = 3;
    const ChariotPhase: i32 = 4;
    const CannonPhase: i32 = 3;
    const ElephantPhase: i32 = 1;
    const AdvisorPhase: i32 = 1;
    const TotalPhase: i32 = Self::SoliderPhase * 10 + Self::HorsePhase * 4 + Self::ChariotPhase * 4 + Self::ElephantPhase * 4 + Self::AdvisorPhase * 4 + Self::CannonPhase * 4;

    // piece square tables

    // base values
    const BasePieceScore: [i32; 7] = [20, 50, 100, 20, 10, 35, 10];


    pub fn evaluate(board: &mut Board) -> i32 {
        let mg_eval = board.mg_score[board.player as usize] - board.mg_score[board.player.inverse() as usize];
        let eg_eval = board.eg_score[board.player as usize] - board.eg_score[board.player.inverse() as usize];

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