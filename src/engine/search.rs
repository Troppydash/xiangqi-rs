use std::os::linux::raw::stat;
use crate::board::board::Board;
use crate::board::condition::Condition::{BLACK, DRAW, RED};
use crate::board::movee::Move;
use crate::board::piece::Piece;
use crate::engine::parameters::SearchParameters;
use crate::engine::tt::TT;

pub struct Engine {
    tt: TT,
    history: Vec<Vec<Vec<i32>>>,
    killers: Vec<Vec<Move>>,
    counter: Vec<Vec<Vec<Move>>>,

    maxpositions: i32,

    // debug
    searches: i32,
}

impl Engine {
    // piece lookup
    const SCORES: [f32; 8] = [0.0, 2.0, 5.0, 10.0, 2.0, 1.0, 3.5, 1.0];

    // position lookup
    const MULTIPLIERS: [[[f32; 9]; 10]; 8] = [
        [
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ],
        // advisor
        [
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
        ],
        // cannon
        [
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, ],
            [0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
        ],
        // chariot
        [
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.4, 1.4, 1.4, 1.4, 1.4, 1.4, 1.4, 1.4, 1.4, ],
            [1.3, 1.3, 1.3, 1.3, 1.3, 1.3, 1.3, 1.3, 1.3, ],
            [1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2, ],
            [1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
        ],
        // elephant
        [
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [0.7, 1.0, 1.0, 1.0, 1.2, 1.0, 1.0, 1.0, 0.7, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
        ],
        // general
        [
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, -0.5, -0.5, -0.5, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, ],
        ],
        // horse
        [
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, ],
            [1.4, 1.4, 1.4, 1.4, 1.4, 1.4, 1.4, 1.4, 1.4, ],
            [1.3, 1.3, 1.3, 1.3, 1.3, 1.3, 1.3, 1.3, 1.3, ],
            [1.0, 1.2, 1.5, 1.2, 1.2, 1.2, 1.5, 1.2, 1.0, ],
            [1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, 1.1, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
        ],
        // soldier
        [
            [2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5],
            [2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5],
            [2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5],
            [2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5],
            [2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, ],
            [1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2, 1.2, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
            [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, ],
        ]
    ];


    pub fn new() -> Self {
        Self {
            tt: TT::new(),
            history: vec![vec![vec![0; 90]; 90]; 2],
            killers: vec![vec![Move::null(); SearchParameters::MaxKillers as usize]; SearchParameters::MaxDepth as usize],
            counter: vec![vec![vec![Move::null(); 90]; 90]; 2],
            maxpositions: 0,
            searches: 0,
        }
    }

    fn classic_predict(&self, game: &mut Board, ply: i32) -> f32 {
        // TODO: this is shit
        let mut score = 0.0;

        let cond = game.condition();
        if cond == game.player {
            return SearchParameters::Checkmate * 2.0 - ply as f32;
        } else if cond == game.player.inverse() {
            return -SearchParameters::Checkmate * 2.0 + ply as f32;
        } else if cond == DRAW {
            return 0.0;
        }

        let sign = if game.player == RED { 1 } else { -1 };

        for row in 0..Board::ROWS {
            let mut r = row;
            if game.player == BLACK {
                r = 9 - row;
            }

            for col in 0..Board::COLS {
                let cell = game.state[row][col];
                if cell == Piece::SPACE {
                    continue;
                }


                if cell.signum() == sign {
                    score += Self::SCORES[cell.abs() as usize]
                        * Self::MULTIPLIERS[cell.abs() as usize]
                        [r][col];
                } else {
                    score -= Self::SCORES[cell.abs() as usize]
                        * Self::MULTIPLIERS[cell.abs() as usize]
                        [9 - r][col];
                }
            }
        }

        let move_advantage = 0.5;
        score + move_advantage
    }

    fn score_moves(&self, game: &mut Board, moves: &mut Vec<Move>, ply: i32, pv_move: &Move, prev_move: &Move) {
        let ply = ply as usize;

        // TODO: add this, but also fix draw and history quiet move stores
        // sort by history, decreasing
        moves.sort_unstable_by_key(|mov| {
            let mut score = 0;

            let capture = mov.captured;
            if mov.equals(pv_move) {
                score += SearchParameters::MvvLvaOffset + SearchParameters::PVMoveScore;
            } else if capture != Piece::SPACE {
                score += SearchParameters::MvvLvaOffset + 5*Self::SCORES[capture.abs() as usize] as i32;
            } else if mov.equals(&self.killers[ply][0]) {
                score += SearchParameters::MvvLvaOffset - SearchParameters::FirstKillerMoveScore;
            } else if mov.equals(&self.killers[ply][1]) {
                score += SearchParameters::MvvLvaOffset - SearchParameters::SecondKillerMoveScore;
            } else {
                let history_score = self.get_history(game, mov);

                if !prev_move.is_null() {
                    let counter_move = &self.counter
                        [game.player as usize]
                        [prev_move.start_sq()]
                        [prev_move.end_sq()];

                    if mov.equals(counter_move) {
                        score += SearchParameters::CounterMoveBonus;
                    }
                }

                score += history_score;
            }

            return score;
        });
        moves.reverse();
    }

    fn qsearch(&mut self, game: &mut Board, mut alpha: f32, beta: f32, pv_line: &mut Vec<Move>, ply: i32, maxply: i32) -> f32 {
        self.searches += 1;

        // conditions check that are exact
        let cond = game.condition();
        if cond == game.player {
            return SearchParameters::Win - ply as f32;
        } else if cond == game.player.inverse() {
            return -SearchParameters::Win + ply as f32;
        } else if cond == DRAW {
            return 0.0;
        }

        if self.searches > self.maxpositions {
            return 0.0;
        }

        if maxply + ply >= SearchParameters::MaxDepth {
            return self.classic_predict(game, ply);
        }

        let mut best_score = self.classic_predict(game, ply);
        let in_check = ply <= 2 && game.is_check();

        if !in_check && best_score >= beta {
            return best_score;
        }

        if best_score > alpha {
            alpha = best_score;
        }

        let mut moves = game.get_moves(!in_check);
        self.score_moves(game, &mut moves, maxply, &Move::null(), &Move::null());


        for mov in moves.iter_mut() {
            let mut child_pv_line = vec![];

            // todo: static exchange

            game.mov(mov);
            let score = -self.qsearch(
                game, -beta, -alpha, &mut child_pv_line, ply + 1, maxply
            );
            game.unmov(mov);

            if score > best_score {
                best_score = score;
            }

            if score >= beta {
                break;
            }

            if score > alpha {
                alpha = score;
                pv_line.clear();
                pv_line.push(mov.clone());
                pv_line.append(&mut child_pv_line);
            }
        }

        best_score
    }

    fn increment_history(&mut self, game: &Board, mov: &Move, depth: i32) {
        if mov.is_quiet() {
            self.history
                [game.player as usize]
                [(mov.starty * 9 + mov.startx) as usize]
                [(mov.endy * 9 + mov.endx) as usize] += depth * depth;
        }

        if self.get_history(game, mov) >= SearchParameters::MaxHistoryScore {
            self.age_history(game);
        }
    }

    fn age_history(&mut self, game: &Board) {
        for a in 0..90 {
            for b in 0..90 {
                self.history
                [game.player as usize]
                [a][b] /= 2;
            }
        }
    }

    fn decrement_history(&mut self, game: &Board, mov: &Move) {
        if mov.is_quiet() && self.get_history(game, mov) > 0 {
            self.history
                [game.player as usize]
                [(mov.starty * 9 + mov.startx) as usize]
                [(mov.endy * 9 + mov.endx) as usize] -= 1;
        }
    }

    fn get_history(&self, game: &Board, mov: &Move) -> i32 {
        self.history
            [game.player as usize]
            [(mov.starty * 9 + mov.startx) as usize]
            [(mov.endy * 9 + mov.endx) as usize]
    }

    fn store_killer(&mut self, ply: i32, mov: &Move) {
        let ply = ply as usize;
        if mov.is_quiet() {
            if !mov.equals(&self.killers[ply][0]) {
                self.killers[ply][1] = self.killers[ply][0].clone();
                self.killers[ply][0] = mov.clone();
            }
        }
    }

    fn store_counter(&mut self, game: &Board, prev_move: &Move, curr_move: &Move) {
        if curr_move.is_quiet() && !prev_move.is_null() {
            self.counter
                [game.player as usize]
                [prev_move.start_sq()]
                [prev_move.end_sq()] = curr_move.clone();
        }
    }


    fn negamax(&mut self, game: &mut Board,
               mut depth: i32, ply: i32, mut alpha: f32, beta: f32,
               pv_line: &mut Vec<Move>,
               do_null: bool, prev_move: &Move, skip_move: &Move, is_extended: bool,
    ) -> f32 {
        self.searches += 1;

        if ply >= SearchParameters::MaxDepth {
            return self.classic_predict(game, ply);
        }

        // conditions check
        let cond = game.condition();
        if cond == game.player {
            return SearchParameters::Win - ply as f32;
        } else if cond == game.player.inverse() {
            return -SearchParameters::Win + ply as f32;
        } else if cond == DRAW {
            return 0.0;
        }

        // fail-safe in case we fuck something up
        if self.searches > self.maxpositions {
            return 0.0;
        }

        let in_check = game.is_check();
        let is_root = ply == 0;
        let is_pv_node = beta - alpha != 1.0;
        let mut can_futility_prune = false;

        // check extension
        if in_check {
            depth += 1;
        }

        if depth <= 0 {
            return self.qsearch(game, alpha, beta, pv_line, ply, ply);
        }

        // tt probing
        let mut tt_move = Move::null();
        let entry = self.tt.probe(game.get_hash());
        let (mov, tt_score, should_use) = entry.get(game.get_hash(), ply, depth, alpha, beta);
        if mov.is_some() {
            tt_move = mov.unwrap();
        }
        let tt_hit = entry.hash == game.get_hash();
        let can_sve = entry.flag == SearchParameters::ExactFlag || entry.flag == SearchParameters::BetaFlag;  // need to be here for rust is a crybaby
        let caniid = entry.flag == SearchParameters::BetaFlag;

        // use tt score
        if should_use && !is_root && !skip_move.equals(&tt_move) {
            return tt_score;
        }

        // static null move pruning
        if !in_check && !is_pv_node && beta.abs() < SearchParameters::Checkmate {
            let stat = self.classic_predict(game, ply);
            let margin = (SearchParameters::StaticNullMovePruningBaseMargin * depth) as f32;
            if stat - margin >= beta {
                return stat - margin;
            }
        }

        // null move pruning
        if do_null 
            && !in_check 
            && !is_pv_node 
            && depth >= SearchParameters::NMRDepthLimit
            // && false
            // todo: only do if has major pieces
        {
            let mut child_pv_line = vec![];

            game.mov(&mut Move::null());
            let R = 1 + depth / 6;
            let score = -self.negamax(game, depth - 1 - R, ply + 1, -beta, -beta + 1.0, &mut child_pv_line, false, &Move::null(), &Move::null(), is_extended);
            game.unmov(&mut Move::null());

            if score >= beta && score.abs() < SearchParameters::Checkmate {
                return beta;
            }
        }

        // razoring
        if depth <= 2 && !is_pv_node && !in_check {
            let static_score = self.classic_predict(game, ply);
            if (static_score + (SearchParameters::FutilityMargins[depth as usize]*3) as f32) < alpha {
                let score = self.qsearch(game, alpha, beta, &mut vec![], ply, 0);
                if score < alpha {
                    return alpha;
                }
            }
        }

        // futility pruning
        if depth <= SearchParameters::FutilityPruningDepthLimit
            && !is_pv_node
            && !in_check
            && alpha < SearchParameters::Checkmate
            && beta < SearchParameters::Checkmate {
            let static_score = self.classic_predict(game, ply);
            let margin = SearchParameters::FutilityMargins[depth as usize] as f32;
            can_futility_prune = static_score + margin <= alpha;
        }

        // internal iterative deepening
        if depth >= SearchParameters::IIDDepthLimit
            && (is_pv_node || caniid)
            && tt_move.equals(&Move::null()) {
            let mut child_pv_line = vec![];
            self.negamax(game, depth-SearchParameters::IIDDepthReduction-1, ply+1, -beta, -alpha, &mut child_pv_line, true, &Move::null(), &Move::null(), is_extended);
            if child_pv_line.len() > 0 {
                tt_move = child_pv_line[0].clone();
            }
        }

        let mut moves = game.get_moves(false);
        self.score_moves(game, &mut moves, ply, &tt_move, prev_move);

        let mut legal_moves = 0;
        let mut tt_flag = SearchParameters::AlphaFlag;
        let mut best_score = -1e9;
        let mut best_move = &Move::null();

        for mov in moves.iter_mut() {
            if mov.equals(skip_move) {
                continue;
            }

            let mut child_pv_line = vec![];

            game.mov(mov);
            legal_moves += 1;

            // late move pruning
            if depth <= 5
                && !is_pv_node
                && !in_check
                && legal_moves > SearchParameters::LateMovePruningMargins[depth as usize] {
                let tactical = game.is_check() || !mov.is_quiet();
                if !tactical {
                    game.unmov(mov);
                    continue;
                }
            }

            // futility prune
            if can_futility_prune
                && legal_moves > 1
                && !game.is_check()
                && mov.is_quiet() {
                game.unmov(mov);
                continue;
            }

            let mut score;

            if legal_moves == 1 {
                let mut next_depth = depth - 1;

                // singular extension
                if !is_extended
                    && depth >= SearchParameters::SingularExtensionDepthLimit
                    && tt_move.equals(mov)
                    && is_pv_node
                    && tt_hit
                    && can_sve {

                    game.unmov(mov);

                    let score_to_beat = tt_score - SearchParameters::SingularMoveMargin;
                    let R = 1 + depth / 6;

                    let next_best_score = self.negamax(game, depth - 1 - R, ply+1, score_to_beat, score_to_beat+1.0, &mut vec![], true, prev_move, mov, true);
                    if next_best_score <= score_to_beat {
                        next_depth += SearchParameters::SingularMoveExtension;
                    }

                    game.mov(mov);
                }

                score = -self.negamax(game, next_depth, ply + 1, -beta, -alpha, &mut child_pv_line, true, mov, &Move::null(), is_extended);
            } else {
                // late move reduction
                let tactical = in_check && mov.captured != 0;
                let mut reduction = 0;
                if !is_pv_node && legal_moves >= SearchParameters::LMRLegalMovesLimit
                    && depth >= SearchParameters::LMRDepthLimit && !tactical {
                    reduction = SearchParameters::LMR(depth, legal_moves);
                }

                score = -self.negamax(game, depth - 1 - reduction, ply + 1, -(alpha + 1.0), -alpha, &mut child_pv_line, true, mov, &Move::null(), is_extended);

                if score > alpha && reduction > 0 {
                    score = -self.negamax(game, depth - 1, ply + 1, -(alpha + 1.0), -alpha, &mut child_pv_line, true, mov, &Move::null(), is_extended);
                    if score > alpha {
                        score = -self.negamax(game, depth - 1, ply + 1, -beta, -alpha, &mut child_pv_line, true, mov, &Move::null(), is_extended);
                    }
                } else if alpha < score && score < beta {
                    score = -self.negamax(game, depth - 1, ply + 1, -beta, -alpha, &mut child_pv_line, true, mov, &Move::null(), is_extended);
                }
            }

            game.unmov(mov);

            if score > best_score {
                best_score = score;
                best_move = mov;
            }

            if score >= beta {
                tt_flag = SearchParameters::BetaFlag;
                self.increment_history(game, mov, depth);
                self.store_killer(ply, mov);
                self.store_counter(game, prev_move, mov);
                break;
            } else {
                self.decrement_history(game, mov);
            }

            if score > alpha {
                alpha = score;
                tt_flag = SearchParameters::ExactFlag;
                pv_line.clear();
                pv_line.push(mov.clone());
                pv_line.append(&mut child_pv_line);
                self.increment_history(game, mov, depth);
            } else {
                self.decrement_history(game, mov);
            }
        }

        // store tt
        let entry = self.tt.store(game.get_hash(), depth);
        entry.set(game.get_hash(), best_score, best_move, ply, depth, tt_flag);


        best_score
    }

    pub fn search(&mut self, game: &mut Board, maxdepth: i32, maxpositions: i32) -> (Move, f32) {
        self.searches = 0;
        self.maxpositions = maxpositions;


        let mut best_move = Move::null();
        let mut alpha = -1e9;
        let mut beta = -1e9;
        let mut score = 0.0;

        let mut level = 1;
        while level <= maxdepth {
            let mut pv_line = vec![];

            let before = game.get_hash();
            score = self.negamax(game, level, 0, alpha, beta, &mut pv_line, true, &Move::null(), &Move::null(), false);
            assert_eq!(before, game.get_hash(), "checking if the hash before and after negamax is equal");

            if self.searches > self.maxpositions {
                if best_move.is_null() && level == 1 {
                    best_move = pv_line[0].clone();
                }
                break;
            }

            // did not converge
            if score <= alpha || score >= beta {
                println!("restart");
                alpha = -1e9;
                beta = 1e9;
                continue;
            }

            alpha = score - SearchParameters::Window as f32;
            beta = score + SearchParameters::Window as f32;

            best_move = pv_line[0].clone();
            let score_text = if score > SearchParameters::Checkmate {
                format!("+M{}", SearchParameters::Win - score)
            } else if score < -SearchParameters::Checkmate {
                format!("-M{}", score + SearchParameters::Win)
            } else {
                format!("{}", score)
            };

            println!("Searched {}, Depth {}, PV {}, Score {}", self.searches, level, best_move.display(), score_text);

            // check for position limit and checkmates
            if self.searches > maxpositions || score.abs() > SearchParameters::Checkmate - 100.0 {
                break;
            }

            level += 1;
        }

        (best_move, score)
    }
}