use crate::board::board::Board;
use crate::board::condition::Condition::{BLACK, DRAW, RED};
use crate::board::movee::Move;
use crate::board::piece::Piece;
use crate::engine::parameters::SearchParameters;
use crate::engine::tt::TT;

pub struct Engine {
    tt: TT,
    history: Vec<Vec<Vec<i32>>>,

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
            searches: 0,
        }
    }

    fn classic_predict(&self, game: &mut Board, ply: i32) -> f32 {
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

        let move_advantage = 1.0;
        score + move_advantage
    }

    fn score_moves(&self, game: &mut Board, moves: &mut Vec<Move>) {
        // TODO: add this, but also fix draw and history quiet move stores
        // sort by history, decreasing
        moves.sort_unstable_by_key(|mov| self.get_history(game, mov));
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
        self.score_moves(game, &mut moves);


        for mov in moves.iter_mut() {
            let mut child_pv_line = vec![];

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
        self.history
            [game.player as usize]
            [(mov.starty * 9 + mov.startx) as usize]
            [(mov.endy * 9 + mov.endx) as usize] += depth * depth;
    }

    fn decrement_history(&mut self, game: &Board, mov: &Move) {
        if self.get_history(game, mov) > 0 {
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

    fn negamax(&mut self, game: &mut Board,
               mut depth: i32, ply: i32, mut alpha: f32, beta: f32,
               pv_line: &mut Vec<Move>,
               do_null: bool, prev_move: &Move, skip_move: &Move, is_extended: bool,
    ) -> f32 {
        self.searches += 1;

        // conditions check
        let cond = game.condition();
        if cond == game.player {
            return SearchParameters::Win - ply as f32;
        } else if cond == game.player.inverse() {
            return -SearchParameters::Win + ply as f32;
        } else if cond == DRAW {
            return 0.0;
        }

        let in_check = game.is_check();
        let is_root = ply == 0;
        let is_pv_node = beta - alpha != 1.0;
        
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
        if do_null && !in_check && !is_pv_node && depth >= SearchParameters::NMRDepthLimit {
            let mut child_pv_line = vec![];

            game.mov(&mut Move::null());
            let R = 3 + depth / 6;
            let score = -self.negamax(game, depth - 1 - R, ply + 1, -beta, -beta + 1.0, &mut child_pv_line, false, &Move::null(), &Move::null(), is_extended);
            game.unmov(&mut Move::null());

            if score >= beta && score.abs() < SearchParameters::Checkmate {
                return beta;
            }
        }

        let mut moves = game.get_moves(false);
        self.score_moves(game, &mut moves);

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


            let mut score;

            if legal_moves == 1 {
                let next_depth = depth - 1;

                // singular extension: todo

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
            
            // did not converge
            if score <= alpha || score >= beta {
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