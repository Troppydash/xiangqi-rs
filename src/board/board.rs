use std::cmp::{max, min};
use std::collections::HashMap;
use rand::Rng;
use crate::board::condition::Condition;
use crate::board::condition::Condition::{NONE, RED};
use crate::board::movee::Move;
use crate::board::piece::Piece;

#[derive(Clone)]
pub struct Board {
    // states
    pub state: Vec<Vec<i8>>,
    pub player: Condition,

    // caches
    horizontal: Vec<(i8, i8)>,
    horse: Vec<(i8, i8)>,
    diagonal: Vec<(i8, i8)>,

    rng: Vec<Vec<Vec<u64>>>,
    rng_black: u64,
    hh: u64,

    cache_moves: Vec<Move>,
    cache_ok: bool,

    ply: i32,
    last_capture: i32,
    history: HashMap<u64, u8>,
}

impl Board {
    pub const ROWS: usize = 10;
    pub const COLS: usize = 9;

    pub fn new() -> Self {
        let board = vec![
            vec![-3, -6, -4, -1, -5, -1, -4, -6, -3],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, -2, 0, 0, 0, 0, 0, -2, 0],
            vec![-7, 0, -7, 0, -7, 0, -7, 0, -7],
            vec![0, 0, 0, 2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![7, 0, 7, 0, 7, 0, 7, 0, 7],
            vec![0, 2, 0, 0, 0, 0, 0, 2, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![3, 6, 4, 1, 5, 1, 4, 6, 3],
        ];

        let horizontal = vec![
            (-1, 0),
            (1, 0),
            (0, 1),
            (0, -1)
        ];
        let diagonal = vec![
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1)
        ];
        let mut horse = Vec::new();
        for drow in [-2i8, -1, 1, 2] {
            for dcol in [-2i8, -1, 1, 2] {
                if drow.abs() + dcol.abs() == 3 {
                    horse.push((drow, dcol));
                }
            }
        }


        let mut rng = rand::thread_rng();
        let rng_black = rng.random::<u64>();
        let mut rngs = vec![];
        for _ in 0..14 {
            let mut v1 = vec![];
            for _ in 0..10 {
                let mut v2 = vec![];
                for _ in 0..9 {
                    v2.push(rng.random());
                }
                v1.push(v2);
            }
            rngs.push(v1);
        }

        let mut item = Self {
            state: board,
            player: Condition::RED,
            horizontal,
            horse,
            diagonal,
            rng: rngs,
            rng_black,
            hh: 0,
            ply: 0,
            last_capture: 0,
            history: Default::default(),
            cache_moves: vec![],
            cache_ok: false,
        };
        item.get_hash();
        item
    }

    pub fn get_hash_cell(&self, row: i8, col: i8) -> u64 {
        let row = row as usize;
        let col = col as usize;

        let cell = self.state[row][col];
        if cell == 0 {
            return 0;
        }

        if cell > 0 {
            self.rng[(cell - 1) as usize][row][col]
        } else {
            self.rng[(-cell - 1 + 7) as usize][row][col]
        }
    }

    pub fn get_hash(&mut self) -> u64 {
        if self.hh == 0 {
            for row in 0..Self::ROWS {
                for col in 0..Self::COLS {
                    self.hh ^= self.get_hash_cell(row as i8, col as i8);
                }
            }
        }

        if self.player == Condition::BLACK {
            self.hh ^ self.rng_black
        } else {
            self.hh
        }
    }

    pub fn get_moves(&mut self, captures: bool) -> Vec<Move> {
        if !captures && self.cache_ok {
            return self.cache_moves.clone();
        }
        
        let mut buffer = self.get_all_moves();

        // find own general and other
        let sign = if self.player == Condition::RED { 1 } else { -1 };
        let mut grow = -1;
        let mut gcol = -1;
        let mut otherrow = -1;
        let mut othercol = -1;
        for row in 0..Self::ROWS {
            for col in 0..Self::COLS {
                if self.state[row][col] == sign * Piece::GENERAL {
                    grow = row as i8;
                    gcol = col as i8;
                } else if self.state[row][col] == -sign * Piece::GENERAL {
                    otherrow = row as i8;
                    othercol = col as i8;
                }
            }
        }

        if grow == -1 || otherrow == -1 {
            println!("uh oh 1");
        }

        self.player = self.player.inverse();
        let mut potential = self.get_potentials(grow, gcol);
        self.player = self.player.inverse();


        let mut updated_buffer = vec![];

        for mov in buffer.iter_mut() {
            if captures && mov.captured == 0 {
                continue;
            }

            if !self.is_valid_move(&mov) {
                println!("{}", self.display());
                println!("Move {}", mov.display());
                // println!("oh no 2");
                continue;
            }

            self.mov(mov);
            if !self.will_check(&mov, &mut potential, grow, gcol, otherrow, othercol) {
                updated_buffer.push(mov.clone());
            }
            self.unmov(mov);
        }

        if !captures {
            self.cache_ok = true;
            self.cache_moves = updated_buffer.clone();
        }
        
        updated_buffer
    }


    /// Checks if the current king is in check
    pub fn is_check(&mut self) -> bool {
        let sign = if self.player == Condition::RED { 1 } else { -1 };

        // find own general
        let mut grow = -1;
        let mut gcol = -1;
        for row in 0..Self::ROWS {
            for col in 0..Self::COLS {
                if self.state[row][col] == sign * Piece::GENERAL {
                    grow = row as i8;
                    gcol = col as i8;
                    break;
                }
            }
        }

        if grow == -1 {
            println!("uh oh");
        }

        // find moves of other team
        self.player = self.player.inverse();
        for mov in self.get_all_moves() {
            if mov.endx == gcol && mov.endy == grow {
                self.player = self.player.inverse();
                return true;
            }
        }

        self.player = self.player.inverse();
        return false;
    }


    pub fn mov(&mut self, mov: &mut Move) {
        self.cache_ok = false;

        // handle null
        if mov.is_null() {
            self.player = self.player.inverse();
            self.ply += 1;
            return;
        }

        self.hh &= self.get_hash_cell(mov.endy, mov.endx);
        self.hh &= self.get_hash_cell(mov.starty, mov.startx);

        mov.last_capture = self.last_capture;
        if self.state[mov.endy as usize][mov.endx as usize] != Piece::SPACE {
            self.last_capture = self.ply;
        }

        let ch = self.state[mov.starty as usize][mov.startx as usize];
        self.state[mov.starty as usize][mov.startx as usize] = Piece::SPACE;
        self.state[mov.endy as usize][mov.endx as usize] = ch;

        self.hh ^= self.get_hash_cell(mov.endy, mov.endx);

        self.player = self.player.inverse();

        self.ply += 1;

        // TODO: history
    }

    pub fn unmov(&mut self, mov: &mut Move) {
        self.cache_ok = false;

        // handle null
        if mov.is_null() {
            self.player = self.player.inverse();
            self.ply -= 1;
            return;
        }

        self.hh ^= self.get_hash_cell(mov.endy, mov.endx);

        if mov.captured != Piece::SPACE {
            self.last_capture = mov.last_capture;
        }

        self.state[mov.starty as usize][mov.startx as usize] = self.state[mov.endy as usize][mov.endx as usize];
        self.state[mov.endy as usize][mov.endx as usize] = mov.captured;

        self.hh ^= self.get_hash_cell(mov.starty, mov.startx);
        self.hh ^= self.get_hash_cell(mov.endy, mov.endx);

        self.player = self.player.inverse();
        self.ply -= 1;

        // TODO: history
    }

    pub fn condition(&mut self) -> Condition {
        let moves = self.get_moves(false);

        // 30 move rule
        if self.ply - self.last_capture >= 60 {
            return Condition::DRAW;
        }

        // TODO: 3 fold rep

        if moves.len() == 0 {
            return self.player.inverse();
        }
        Condition::NONE
    }


    pub fn display(&self) -> String {
        let cols: Vec<char> = "ABCDEFGHIJK".chars().collect();
        let rows: Vec<char> = "X987654321".chars().collect();

        let mut last = vec![];
        let mut out = vec![];
        let mut first = true;
        for row in 0..Self::ROWS {
            let mut line = vec![rows[row].to_string() + " "];
            let mut divider = vec![" ".repeat(line[0].len())];
            if first {
                last = vec![" ".repeat(line[0].len())];
            }

            for col in 0..Self::COLS {
                let symbol = format!(" {} ", Piece::display(self.state[row][col]));
                divider.push("-".repeat(symbol.len()));
                line.push(symbol);

                if first {
                    last.push(format!(" {} ", cols[col]));
                }
            }

            line.push("".to_string());
            divider.push("".to_string());

            let line = line.join("|");
            let divider = divider.join("+");

            if first {
                out.push(divider.clone());
                first = false;
            }

            out.push(line);
            out.push(divider);
        }

        out.push(last.join(" "));

        out.join("\n")
    }
}

/// MOVES ///
impl Board {
    fn get_cell_player(&self, row: i8, col: i8) -> Condition {
        let value = self.state[row as usize][col as usize];
        if value == 0 {
            return Condition::NONE;
        }

        if value > 0 {
            return Condition::RED;
        }
        return Condition::BLACK;
    }

    fn is_inbound(&self, row: i8, col: i8) -> bool {
        !(row < 0 || row >= Self::ROWS as i8 || col < 0 || col >= Self::COLS as i8)
    }

    fn is_valid_move(&self, mov: &Move) -> bool {
        if !self.is_inbound(mov.starty, mov.startx)
            || !self.is_inbound(mov.endy, mov.endx) {
            return false;
        }

        if self.get_cell_player(mov.starty, mov.startx) != self.player {
            return false;
        }

        if self.get_cell_player(mov.starty, mov.startx) == self.get_cell_player(mov.endy, mov.endx) {
            return false;
        }

        return true;
    }


    fn get_all_moves(&mut self) -> Vec<Move> {
        let mut mov_buffer = vec![];

        let sign = if self.player == RED { 1 } else { -1 };
        for row in 0..Self::ROWS {
            for col in 0..Self::COLS {
                if self.state[row][col] == Piece::SPACE {
                    continue;
                }

                let irow = row as i8;
                let icol = col as i8;

                match self.state[row][col] * sign {
                    Piece::SOLDIER => {
                        self.soldier_moves(irow, icol, &mut mov_buffer);
                    }
                    Piece::CANNON => {
                        self.cannon_moves(irow, icol, &mut mov_buffer, 0, 0);
                    }
                    Piece::CHARIOT => {
                        self.chariot_moves(irow, icol, &mut mov_buffer, 0, 0);
                    }
                    Piece::ADVISOR => {
                        self.advisor_moves(irow, icol, &mut mov_buffer);
                    }
                    Piece::ELEPHANT => {
                        self.elephant_moves(irow, icol, &mut mov_buffer);
                    }
                    Piece::GENERAL => {
                        self.general_moves(irow, icol, &mut mov_buffer);
                    }
                    Piece::HORSE => {
                        self.horse_moves(irow, icol, &mut mov_buffer, 0, 0);
                    }
                    _ => {}
                }
            }
        }

        for mov in &mut mov_buffer {
            mov.captured = self.state[mov.endy as usize][mov.endx as usize];
        }

        return mov_buffer;
    }


    /// Checks if the last move resulted in check
    fn will_check(&mut self,
                  mov: &Move, potential: &Vec<(i8, i8)>,
                  mut grow: i8, mut gcol: i8,
                  otherrow: i8, othercol: i8) -> bool {

        // because rust is shit
        let mut potential = potential;


        // recompute for new general position if moved
        let temp;  // need this to stay
        if mov.startx == gcol && mov.starty == grow {
            gcol = mov.endx;
            grow = mov.endy;

            temp = self.get_potentials(grow, gcol);
            potential = &temp;
        }

        // check line
        if gcol == othercol {
            let mut ok = false;
            for row in (min(grow, otherrow) + 1)..(max(grow, otherrow)) {
                if self.state[row as usize][gcol as usize] != Piece::SPACE {
                    ok = true;
                    break;
                }
            }
            if !ok {
                return true;
            }
        }

        let sign = if self.player == Condition::RED { 1 } else { -1 };


        for (row, col) in potential {
            let row = *row;
            let col = *col;

            if self.state[row as usize][col as usize] == Piece::SPACE {
                continue;
            }

            let mut check_buffer = vec![];
            match sign * self.state[row as usize][col as usize] {
                Piece::SOLDIER => {
                    self.soldier_moves(row, col, &mut check_buffer);
                }
                Piece::CANNON => {
                    self.cannon_moves(row, col, &mut check_buffer, (gcol - col).signum(), (grow-row).signum());
                }
                Piece::CHARIOT => {
                    self.chariot_moves(row, col, &mut check_buffer, (gcol - col).signum(), (grow-row).signum());
                }
                Piece::HORSE => {
                    self.horse_moves(row, col, &mut check_buffer, gcol - col, grow - row);
                }
                _ => {}
            }

            for mov in check_buffer {
                if mov.endx == gcol && mov.endy == grow {
                    return true;
                }
            }
        }

        return false;
    }

    fn get_potentials(&self, grow: i8, gcol: i8) -> Vec<(i8, i8)> {
        let sign = if self.player == Condition::RED { 1 } else { -1 };
        let mut attacks = vec![];

        for row in 0..Self::ROWS {
            for col in 0..Self::COLS {
                if self.state[row][col] == Piece::SPACE {
                    continue;
                }

                let irow = row as i8;
                let icol = col as i8;

                if match self.state[row][col] * sign {
                    Piece::CANNON => {
                        self.cannon_potential(irow, icol, grow, gcol)
                    }
                    Piece::SOLDIER => {
                        self.soldier_potential(irow, icol, grow, gcol)
                    }
                    Piece::CHARIOT => {
                        self.chariot_potential(irow, icol, grow, gcol)
                    }
                    Piece::HORSE => {
                        self.horse_potential(irow, icol, grow, gcol)
                    }
                    _ => false
                } {
                    attacks.push((irow, icol));
                }
            }
        }

        return attacks;
    }


    fn soldier_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
        let sign = if self.player == Condition::RED { 1 } else { -1 };
        let direction = -sign;

        let directions = if (self.player == Condition::RED && row <= 4) || (self.player == Condition::BLACK && row >= 5) {
            vec![(direction, 0), (0, 1), (0, -1)]
        } else {
            vec![(direction, 0)]
        };


        for (drow, dcol) in directions {
            let mov = Move::new(row, col, row + drow, col + dcol);
            if self.is_valid_move(&mov) {
                moves.push(mov);
            }
        }
    }

    fn soldier_potential(&self, row: i8, col: i8, grow: i8, gcol: i8) -> bool {
        if !((self.player == Condition::RED && row <= 4) || (self.player == Condition::BLACK && row >= 5)) {
            return false;
        }

        let sign = if self.player == Condition::RED { 1 } else { -1 };
        let direction = -sign;

        let directions = vec![(direction, 0), (0, 1), (0, -1)];
        for (drow, dcol) in directions {
            let mov = Move::new(row, col, row + drow, col + dcol);
            if grow == mov.endy && gcol == mov.endx {
                return true;
            }
        }

        return false;
    }

    fn cannon_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>,  dcol: i8, drow: i8) {
        let search = if !(dcol == 0 && drow == 0) {
            &vec![(drow, dcol)]
        } else {
            &self.horizontal
        };
        
        for (drow, dcol) in search {
            let mut jumped = false;
            for steps in 1..11 {
                let target_row = row + drow * steps;
                let target_col = col + dcol * steps;

                if !self.is_inbound(target_row, target_col) {
                    break;
                }

                let target = self.get_cell_player(target_row, target_col);
                if !jumped {
                    if target == Condition::NONE {
                        moves.push(Move::new(row, col, target_row, target_col));
                    } else {
                        jumped = true;
                    }
                } else {
                    if target == Condition::NONE {
                        continue;
                    } else if target != self.player {
                        moves.push(Move::new(row, col, target_row, target_col));
                        break;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn cannon_potential(&self, row: i8, col: i8, grow: i8, gcol: i8) -> bool {
        row == grow || col == gcol
    }

    fn chariot_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>, dcol: i8, drow: i8) {
        let search = if !(dcol == 0 && drow == 0) {
            &vec![(drow, dcol)]
        } else {
            &self.horizontal
        };
        
        for (drow, dcol) in search {
            for steps in 1..11 {
                let target_row = row + drow * steps;
                let target_col = col + dcol * steps;

                if !self.is_inbound(target_row, target_col) {
                    break;
                }


                let target = self.get_cell_player(target_row, target_col);
                if target == Condition::NONE {
                    moves.push(Move::new(row, col, target_row, target_col));
                } else if target != self.player {
                    moves.push(Move::new(row, col, target_row, target_col));
                    break;
                } else {
                    break;
                }
            }
        }
    }

    fn chariot_potential(&self, row: i8, col: i8, grow: i8, gcol: i8) -> bool {
        row == grow || col == gcol
    }

    fn advisor_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
        for (drow, dcol) in &self.diagonal {
            let mov = Move::new(row, col, row + drow, col + dcol);
            if !(mov.endx >= 3 && mov.endx <= 5) {
                continue;
            }

            if self.player == RED {
                if !(mov.endy >= 7 && mov.endy <= 9) {
                    continue;
                }
            } else {
                if !(mov.endy >= 0 && mov.endy <= 2) {
                    continue;
                }
            }

            if self.is_valid_move(&mov) {
                moves.push(mov);
            }
        }
    }

    fn elephant_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
        for (drow, dcol) in &self.diagonal {
            if self.player == RED {
                if !(5 <= row + 2 * drow && row + 2 * drow <= 9) {
                    continue;
                }
            } else {
                if !(0 <= row + 2 * drow && row + 2 * drow <= 4) {
                    continue;
                }
            }

            if !self.is_inbound(row + 2 * drow, col + 2 * dcol) {
                continue;
            }

            if self.get_cell_player(row + drow, col + dcol) != NONE {
                continue;
            }

            let mov = Move::new(row, col, row + 2 * drow, col + 2 * dcol);
            if self.is_valid_move(&mov) {
                moves.push(mov)
            }
        }
    }

    fn general_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
        for (drow, dcol) in &self.horizontal {
            let mov = Move::new(row, col, row + drow, col + dcol);
            if !(mov.endx >= 3 && mov.endx <= 5) {
                continue;
            }

            if self.player == RED {
                if !(mov.endy >= 7 && mov.endy <= 9) {
                    continue;
                }
            } else {
                if !(mov.endy >= 0 && mov.endy <= 2) {
                    continue;
                }
            }

            if self.is_valid_move(&mov) {
                moves.push(mov);
            }
        }
    }

    fn horse_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>, dcol: i8, drow: i8) {
        let search= if !(dcol == 0 && drow == 0) {
            &vec![(drow, dcol)]
        }else {
            &self.horse
        };
        
        for (drow, dcol) in search {
            let mov = Move::new(row, col, row + drow, col + dcol);
            if !self.is_valid_move(&mov) {
                continue;
            }

            if drow.abs() == 2 {
                if self.get_cell_player(row + drow / 2, col) != NONE {
                    continue;
                }
            } else {
                if self.get_cell_player(row, col + dcol / 2) != NONE {
                    continue;
                }
            }

            moves.push(mov);
        }
    }

    fn horse_potential(&self, row: i8, col: i8, grow: i8, gcol: i8) -> bool {
        for (drow, dcol) in &self.horse {
            let mov = Move::new(row, col, row + drow, col + dcol);
            if mov.endx == gcol && mov.endy == grow {
                return true;
            }
        }
        return false;
    }
}