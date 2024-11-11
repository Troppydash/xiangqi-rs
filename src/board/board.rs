use std::cmp::{max, min};
use std::collections::HashMap;
use fnv::FnvHashMap;
use rand::Rng;
use crate::board::condition::Condition;
use crate::board::condition::Condition::{BLACK, NONE, RED};
use crate::board::movee::Move;
use crate::board::piece::Piece;
use crate::engine::eval::Eval;

#[derive(Clone)]
pub struct Board {
    /// Board state
    pub state: Vec<Vec<i8>>,
    /// Current player
    pub player: Condition,

    // piece space tables, from red's perspective
    // [piece][row][col]
    mg_table: Vec<Vec<Vec<i32>>>,
    eg_table: Vec<Vec<Vec<i32>>>,

    // cached general position
    general: [i8; 4],

    // cached scores, red and black
    pub mg_score: [i32; 2],
    pub eg_score: [i32; 2],

    // caches
    horizontal: Vec<(i8, i8)>,
    horse: Vec<(i8, i8)>,
    diagonal: Vec<(i8, i8)>,

    // rng for hashes
    rng: Vec<Vec<Vec<u64>>>,
    rng_black: u64,
    hh: u64,

    // cached move computation
    cache_moves: Vec<Move>,
    cache_ok: bool,

    // drawing check
    ply: i32,
    last_capture: i32,  // last capture ply
    history: FnvHashMap<u64, i32>,
    exceeded: bool,  // is a draw
}

impl Board {
    pub const ROWS: usize = 10;
    pub const COLS: usize = 9;

    /// Creates a board
    pub fn new() -> Self {
        let board = vec![
            vec![-3, -6, -4, -1, -5, -1, -4, -6, -3],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, -2, 0, 0, 0, 0, 0, -2, 0],
            vec![-7, 0, -7, 0, -7, 0, -7, 0, -7],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
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

        let (mg, eg) = Eval::create_pst();
        
        
        let mut item = Self {
            state: board,
            player: Condition::RED,
            general: [9, 4, 0, 4],
            eg_table: mg,
            mg_table: eg,
            mg_score: [0, 0],
            eg_score: [0, 0],
            horizontal,
            horse,
            diagonal,
            rng: rngs,
            rng_black,
            hh: 0,
            ply: 0,
            last_capture: 0,
            history: FnvHashMap::default(),
            cache_moves: vec![],
            cache_ok: false,
            exceeded: false,
        };
        item.get_hash();
        item
    }

    
    pub fn load_pst(&mut self, mg: Vec<Vec<Vec<i32>>>, eg: Vec<Vec<Vec<i32>>>) {
        self.mg_table = mg;
        self.eg_table = eg;
    }
    
    /// Gets the hash for the specific cell
    fn get_hash_cell(&self, row: i8, col: i8) -> u64 {
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

    /// Gets the board hash
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

    /// Get a list of moves
    pub fn get_moves(&mut self, captures: bool) -> Vec<Move> {
        if !captures && self.cache_ok {
            return self.cache_moves.clone();
        }

        let mut buffer = if captures && self.cache_ok {
            self.cache_moves.clone()
        } else {
            self.get_all_moves()
        };

        // find own general and other
        let grow = self.general[2*self.player as usize];
        let gcol = self.general[2*self.player as usize + 1];
        let otherrow = self.general[2*self.player.inverse() as usize];
        let othercol = self.general[2*self.player.inverse() as usize + 1];


        self.next_turn();
        let mut potential = self.get_potentials(grow, gcol);
        self.next_turn();

        let mut updated_buffer = vec![];

        for mov in buffer.iter_mut() {
            if captures && mov.captured == 0 {
                continue;
            }

            // if !self.is_valid_move(&mov) {
            //     println!("{}", self.display());
            //     println!("Move {}", mov.display());
            //     println!("oh no 2");
            //     panic!("move is not legal in get_moves");
            // }

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

    /// Flips player turn
    fn next_turn(&mut self) {
        self.cache_ok = false;
        self.player = self.player.inverse();
    }


    /// Checks if the last move resulted a check
    pub fn last_check(&mut self) -> bool {
        // check if the last player will be captured
        self.next_turn();
        let result = self.is_check();
        self.next_turn();

        result
    }

    /// Checks if the current king is in check
    pub fn is_check(&mut self) -> bool {
        // find own general
        let grow = self.general[2*self.player as usize];
        let gcol = self.general[2*self.player as usize + 1];
      
        // find moves of other team
        self.next_turn();
        self.cache_ok = false;
        for mov in self.get_all_moves() {
            if mov.endx == gcol && mov.endy == grow {
                self.next_turn();
                return true;
            }
        }

        self.next_turn();
        return false;
    }

    
    /// Performs the move
    pub fn mov(&mut self, mov: &mut Move) {
        // check if capturing general
        if mov.captured.abs() == Piece::GENERAL {
            println!("capturing general?");
            panic!("trying to capture general");
        }

        // check if exceeded
        if self.exceeded {
            panic!("cannot move when drew");
        }

        // handle null
        if mov.is_null() {
            self.next_turn();
            self.ply += 1;
            return;
        }

        // remove hashes
        self.hh ^= self.get_hash_cell(mov.endy, mov.endx);
        self.hh ^= self.get_hash_cell(mov.starty, mov.startx);

        // handle last capture draws
        mov.last_capture = self.last_capture;
        if self.state[mov.endy as usize][mov.endx as usize] != Piece::SPACE {
            self.last_capture = self.ply;
        }

        // update pst scores
        let piece = self.state[mov.starty as usize][mov.startx as usize].abs();
        assert_ne!(piece, Piece::SPACE, "cannot move an empty space");
        
        // subtract prev
        let mut start = (mov.starty as usize, mov.startx as usize);
        if self.player == BLACK {
            start = Move::flip_coord(&start);
        }
        let mut end = (mov.endy as usize, mov.endx as usize);
        if self.player == BLACK {
            end = Move::flip_coord(&end);
        }
        
        self.mg_score[self.player as usize] -= self.mg_table[piece as usize - 1][start.0][start.1];
        self.eg_score[self.player as usize] -= self.eg_table[piece as usize - 1][start.0][start.1];
        // add new scores
        self.mg_score[self.player as usize] += self.mg_table[piece as usize - 1][end.0][end.1];
        self.eg_score[self.player as usize] += self.eg_table[piece as usize - 1][end.0][end.1];

        // if captures, remove other
        let otherpiece = self.state[mov.endy as usize][mov.endx as usize].abs();
        if otherpiece != Piece::SPACE {
            let other = self.player.inverse() as usize;
            let otherend = Move::flip_coord(&end);
            self.mg_score[other] -= self.mg_table[otherpiece as usize - 1][otherend.0][otherend.1];
            self.eg_score[other] -= self.eg_table[otherpiece as usize - 1][otherend.0][otherend.1];
        }

        // move general
        if self.state[mov.starty as usize][mov.startx as usize] == Piece::GENERAL {
            self.general[0] = mov.endy;
            self.general[1] = mov.endx;
        } else if self.state[mov.starty as usize][mov.startx as usize] == -Piece::GENERAL {
            self.general[2] = mov.endy;
            self.general[3] = mov.endx;
        }

        // perform move
        let ch = self.state[mov.starty as usize][mov.startx as usize];
        self.state[mov.starty as usize][mov.startx as usize] = Piece::SPACE;
        self.state[mov.endy as usize][mov.endx as usize] = ch;

        self.hh ^= self.get_hash_cell(mov.endy, mov.endx);

        self.next_turn();

        self.ply += 1;

        // push history
        let hh = self.hh;
        self.history.insert(hh, self.history.get(&hh).unwrap_or(&0) + 1);
        // for some reason
        if *self.history.get(&hh).unwrap() >= 3 {
            self.exceeded = true;
        }
    }

    /// Undo the move
    pub fn unmov(&mut self, mov: &mut Move) {
        // handle null
        if mov.is_null() {
            self.next_turn();
            self.ply -= 1;
            return;
        }

        // pop history
        let hh = self.hh;
        self.history.insert(hh, self.history.get(&hh).unwrap_or(&0) - 1);
        self.exceeded = false;
        if *self.history.get(&hh).unwrap() == 0 {
            self.history.remove(&hh);
        }

        // remove hash at new position
        self.hh ^= self.get_hash_cell(mov.endy, mov.endx);

        // handle last capture draw
        if mov.captured != Piece::SPACE {
            self.last_capture = mov.last_capture;
        }
        
        // remove end square score and readd back
        let piece = self.state[mov.endy as usize][mov.endx as usize].abs();
        let player = self.player.inverse();
        let mut start = (mov.starty as usize, mov.startx as usize);
        if player == BLACK {
            start = Move::flip_coord(&start);
        }
        let mut end = (mov.endy as usize, mov.endx as usize);
        if player == BLACK {
            end = Move::flip_coord(&end);
        }
        self.mg_score[player as usize] -= self.mg_table[piece as usize - 1][end.0][end.1];
        self.eg_score[player as usize] -= self.eg_table[piece as usize - 1][end.0][end.1];
        self.mg_score[player as usize] += self.mg_table[piece as usize - 1][start.0][start.1];
        self.eg_score[player as usize] += self.eg_table[piece as usize - 1][start.0][start.1];
        
        if mov.captured != Piece::SPACE {
            let otherpiece = mov.captured.abs();
            let otherend = Move::flip_coord(&end);
            self.mg_score[self.player as usize] += self.mg_table[otherpiece as usize - 1][otherend.0][otherend.1];
            self.eg_score[self.player as usize] += self.eg_table[otherpiece as usize - 1][otherend.0][otherend.1];
        }

        // move general
        if self.state[mov.endy as usize][mov.endx as usize] == Piece::GENERAL {
            self.general[0] = mov.starty;
            self.general[1] = mov.startx;
        } else if self.state[mov.endy as usize][mov.endx as usize] == -Piece::GENERAL {
            self.general[2] = mov.starty;
            self.general[3] = mov.startx;
        }

        // perform reverse
        self.state[mov.starty as usize][mov.startx as usize] = self.state[mov.endy as usize][mov.endx as usize];
        self.state[mov.endy as usize][mov.endx as usize] = mov.captured;

        self.hh ^= self.get_hash_cell(mov.starty, mov.startx);
        self.hh ^= self.get_hash_cell(mov.endy, mov.endx);

        self.next_turn();
        self.ply -= 1;
    }

    /// Returns the board summary state
    pub fn condition(&mut self) -> Condition {
        if self.is_draw() {
            return Condition::DRAW;
        }

        let moves = self.get_moves(false);
        if moves.len() == 0 {
            return self.player.inverse();
        }
        Condition::NONE
    }
    
    pub fn is_draw(&self) -> bool {
        // 30 move rule
        if self.ply - self.last_capture >= 60 {
            return true;
        }

        // 3 fold rep
        if self.exceeded {
            return true;
        }
        
        return false;
    }
    
    pub fn score_piece(&self, row: usize, col: usize) -> i32 {
        let piece = self.state[row][col];
        assert!(piece != Piece::SPACE);
        
        let mut coord = (row, col);
        if self.player == BLACK {
            coord = Move::flip_coord(&coord);
        }
        
        return self.mg_table[(piece.abs() - 1) as usize][coord.0][coord.1];
    }

    /// make a move, where the move is unverified
    pub fn try_move(&mut self, mut mov: &mut Move) -> bool {
        if !self.is_valid_move(&mov) {
            return false;
        }

        // load information
        mov.captured = self.state[mov.endy as usize][mov.endx as usize];
        
        if mov.captured.abs() == Piece::GENERAL {
            return false;
        }

        // try move
        self.mov(&mut mov);

        // failing will unmove
        if self.last_check() {
            self.unmov(&mut mov);
            return false;
        }

        true
    }

    /// Returns a string of the board
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


    pub fn parse_move(&self, text: String) -> Option<Move> {
        /// Ok here's the format
        /// {+-[1-5]}{KRHCPAE}{[1-9]}{+-=}{[1-9]}
        /// tandem, piece, file, towards, amount
        ///
        /// All direction is against the current player
        /// Tandem is + if front, - if back, 1-5 for pawns with 1 at the front
        /// Piece is piece name
        /// File is initial file with 1 at RIGHT and 9 at LEFT (none if +-)
        /// Towards is + if forward, - if backwards, = if sideways
        /// Amount is number of steps if +-, or file if =

        /// minimal error checking here

        if !(text.len() >= 4 && text.len() <= 5) {
            // println!( "incorrect move length, got {} and {}, \n{}", text.len(), text, self.display());
            return None;
        }
        

        let chars: Vec<char> = text.chars().collect();

        // the index in file for the piece
        let mut index = 0;
        // the offset in text indexing
        let mut offset = 0;
        let mult: i8 = if self.player == RED { 1 } else { -1 };

        //// Handle move from ////
        let mut start = (0, 0);
        let mut special = false;

        // handle tandem
        let piece;
        let tandem = chars[0];
        if tandem.is_digit(10) {
            // handle pawn tandem
            piece = Piece::SOLDIER;
            offset = 0;
            index = (chars[0] as usize - '0' as usize) - 1;
        } else if tandem == '+' {
            // forward tandem
            piece = Piece::from_char(chars[1])?;
            index = 0;
            offset = 1;
        } else if tandem == '-' {
            // backwards tandem
            piece = Piece::from_char(chars[1])?;
            index = 1;
            offset = 1;
        } else {
            // normal
            piece = Piece::from_char(chars[0])?;
            index = 0;
            offset = 0;
        }

        if tandem.is_digit(10) || tandem == '+' || tandem == '-' {
            // special case
            if chars.len() == 4 {
                offset = 0;
                special = true;
                // locate piece
                let mut failed = true;
                let mut order: Vec<usize> = (0..Self::ROWS).collect();
                if self.player == Condition::BLACK {
                    order.reverse();
                }
                for col in 0..9 {
                    if !failed {
                        break;
                    }
                    
                    // note that we need the column to actually have that many
                    let mut count = 0;
                    for row in order.iter() {
                        if self.state[*row][col] == mult * piece {
                            count += 1;
                        }
                    }
                    
                    if count < 2 {
                        continue;
                    }
                    
                    for row in order.iter() {
                        if self.state[*row][col] == mult * piece {
                            if index == 0 {
                                failed = false;
                                start = (*row, col);
                                break;
                            } else {
                                index -= 1;
                            }
                        }
                    }
                }
                assert!(!failed, "failed to find piece to move special +");
            }
        }

        if !special {
            // get piece column
            let file = chars[offset + 1] as usize - '0' as usize;
            // 1 -> 8, 2 -> 7
            let mut col = 9 - file;
            if self.player == BLACK {
                col = 8 - col;
            }

            // locate piece
            let mut failed = true;
            let mut order: Vec<usize> = (0..Self::ROWS).collect();
            if self.player == Condition::BLACK {
                order.reverse();
            }
            for row in order {
                if self.state[row][col] == mult * piece {
                    if index == 0 {
                        failed = false;
                        start = (row, col);
                        break;
                    } else {
                        index -= 1;
                    }
                }
            }
            
            if failed {
                println!("failed to find piece to move");
                return None;
            }
        }


        //// Handle move to ////

        // get other stats
        let towards = chars[offset + 2];
        let amount = chars[offset + 3] as usize - '0' as usize;

        let mut col = 9 - amount;
        if self.player == BLACK {
            col = 8 - col;
        }

        if towards == '=' {
            // horizontal moves are exact
            let to = (start.0, col);
            return Some(Move::from_coords(start, to));
        } else if towards == '+' {
            let row = (start.0 as i8 - mult * amount as i8) as usize;

            // horizontal can derive row
            if Piece::is_horizontal(piece) {
                let to = (row, start.1);
                return Some(Move::from_coords(start, to));
            }

            // need to find where this piece moved to
            for mov in self.get_all_moves() {
                if mov.startx == start.1 as i8 && mov.starty == start.0 as i8 {
                    if mov.endx == col as i8 && (mov.endy - start.0 as i8).signum() == -mult {
                        // this is the move
                        return Some(mov);
                    }
                }
            }
            println!("cannot find the move destination {}", text);
            return None;
        } else if towards == '-' {
            let row = (start.0 as i8 + mult * amount as i8) as usize;

            // horizontal can derive row
            if Piece::is_horizontal(piece) {
                let to = (row, start.1);
                return Some(Move::from_coords(start, to));
            }

            // need to find where this piece moved to
            for mov in self.get_all_moves() {
                if mov.startx == start.1 as i8 && mov.starty == start.0 as i8 {
                    if mov.endx == col as i8 && (mov.endy - start.0 as i8).signum() == mult {
                        // this is the move
                        return Some(mov);
                    }
                }
            }
            println!("cannot find the move destination {}\n{}", text, self.display());
            return None;
        }

        panic!("why did i reach here?");
    }
}

/// MOVES ///
impl Board {
    /// Gets the cell the player is on
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

    /// Is position inside grid
    fn is_inbound(&self, row: i8, col: i8) -> bool {
        !(row < 0 || row >= Self::ROWS as i8 || col < 0 || col >= Self::COLS as i8)
    }

    /// Is move ok
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

    /// Returns all possible (maybe invalid for checks) moves
    pub fn get_all_moves(&self) -> Vec<Move> {
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

        mov_buffer
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
                    self.cannon_moves(row, col, &mut check_buffer, (gcol - col).signum(), (grow - row).signum());
                }
                Piece::CHARIOT => {
                    self.chariot_moves(row, col, &mut check_buffer, (gcol - col).signum(), (grow - row).signum());
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

    /// Return potential attacker squares
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


    pub fn soldier_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
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
        
        return true;
    }

    pub fn cannon_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>, dcol: i8, drow: i8) {
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

    pub fn chariot_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>, dcol: i8, drow: i8) {
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

    pub fn advisor_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
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

    pub fn elephant_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
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

    pub fn general_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>) {
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

    pub fn horse_moves(&self, row: i8, col: i8, moves: &mut Vec<Move>, dcol: i8, drow: i8) {
        let search = if !(dcol == 0 && drow == 0) {
            &vec![(drow, dcol)]
        } else {
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
        return (row - grow).abs() * (col - gcol).abs() == 2;
    }
}