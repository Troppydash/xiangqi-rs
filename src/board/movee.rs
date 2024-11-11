use std::ops::Index;
use crate::board::piece::Piece;

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct Move {
    pub startx: i8,
    pub starty: i8,
    pub endx: i8,
    pub endy: i8,
    pub captured: i8,
    /// cache of last capture ply, set when moving
    pub last_capture: i32,
}

impl Move {
    pub fn new(starty: i8, startx: i8, endy: i8, endx: i8) -> Self {
        Self { startx, starty, endx, endy, captured: 0, last_capture: 0 }
    }
    
    pub fn from_coords(start: (usize, usize), to: (usize, usize)) -> Self {
        Move::new(start.0 as i8, start.1 as i8, to.0 as i8, to.1 as i8)
    }

    pub fn null() -> Self {
        Self {
            startx: -1,
            starty: -1,
            endx: -1,
            endy: -1,
            captured: -1,
            last_capture: -1
        }
    }

    pub fn from_string(text: &str) -> Option<Move> {
        let cols: Vec<char> = "ABCDEFGHIJK".chars().collect();
        let rows: Vec<char> = "X987654321".chars().collect();

        let text: Vec<char> = text.chars().collect();
        if text.len() != 4 {
            return None;
        }


        let startx = cols.iter().position(|v| *v == text[0])? as i8;
        let starty = rows.iter().position(|v| *v == text[1])? as i8;
        let endx = cols.iter().position(|v| *v == text[2])? as i8;
        let endy = rows.iter().position(|v| *v == text[3])? as i8;
        Some(Move::new(starty, startx, endy, endx))
    }

    pub fn is_null(&self) -> bool {
        self.startx == -1
    }

    pub fn is_quiet(&self) -> bool {
        self.captured == Piece::SPACE
    }

    pub fn start_sq(&self) -> usize {
        (self.starty * 9 + self.startx) as usize
    }

    pub fn end_sq(&self) -> usize {
        (self.endy * 9 + self.endx) as usize
    }

    pub fn display(&self) -> String {
        let cols: Vec<char> = "ABCDEFGHIJK".chars().collect();
        let rows: Vec<char> = "X987654321".chars().collect();

        format!("{}{}{}{}", cols[self.startx as usize], rows[self.starty as usize], cols[self.endx as usize], rows[self.endy as usize])
    }

    pub fn flip_coord(coord: &(usize, usize)) -> (usize, usize) {
        (9 - coord.0, 8 - coord.1)
    }

    pub fn equals(&self, other: &Move) -> bool {
        self.compute_hash() == other.compute_hash()
    }

    pub fn compute_hash(&self) -> u64 {
        if self.is_null() {
            return 0;
        }

        let prime = 37;
        vec![self.startx, self.starty, self.endx, self.endy].iter().fold(0, |acc, &val| acc * prime + (val as u64) + 1)
    }
}