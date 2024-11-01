use crate::board::movee::Move;
use crate::engine::parameters::SearchParameters;

struct TTEntry {
    hash: u64,
    depth: i32,
    score: f32,
    best: Option<Move>,
    flag: i8,
}

impl TTEntry {
    pub fn new() -> Self {
        Self {
            depth: 0,
            hash: 0,
            score: 0.0,
            best: None,
            flag: 0,
        }
    }

    pub fn get(&self, hash: u64, ply: i32, depth: i32, alpha: f32, beta: f32) -> (Option<Move>, f32, bool) {
        let mut adjusted = 0.0;
        let mut should_use = false;
        let mut best = &None;

        if self.hash == hash {
            best = &self.best;
            adjusted = self.score;

            if self.depth >= depth {
                let mut score = self.score;
                if score > SearchParameters::Checkmate {
                    score -= ply as f32;
                }

                if score < -SearchParameters::Checkmate {
                    score += ply as f32;
                }

                if self.flag == SearchParameters::ExactFlag {
                    adjusted = score;
                    should_use = true;
                }

                if self.flag == SearchParameters::AlphaFlag && score <= alpha {
                    adjusted = alpha;
                    should_use = true;
                }

                if self.flag == SearchParameters::BetaFlag && score >= beta {
                    adjusted = beta;
                    should_use = true;
                }
            }
        }

        (best.clone(), adjusted, should_use)
    }

    pub fn set(&mut self, hash: u64, mut score: f32, best: &Option<Move>, ply: i32, depth: i32, flag: i8) {
        self.hash = hash;
        self.depth = depth;
        self.best = best.clone();
        self.flag = flag;

        if score > SearchParameters::Checkmate {
            score -= ply as f32;
        }

        if score < -SearchParameters::Checkmate {
            score += ply as f32;
        }

        self.score = score;
    }
}


pub struct TT {
    size: u64,
    entries: Vec<TTEntry>,
}

impl TT {
    pub fn new() -> Self {
        let size = SearchParameters::Size * 1024 * 1024;
        let mut entries = vec![];
        for _ in 0..size {
            entries.push(TTEntry::new());
        }


        Self {
            size,
            entries,
        }
    }

    pub fn probe(&mut self, hash: u64) -> &mut TTEntry {
        let index = hash % self.size;
        if index + 1 == self.size {
            return &mut self.entries[index as usize];
        }

        if self.entries[index as usize].hash == hash {
            return &mut self.entries[index as usize];
        }

        return &mut self.entries[(index + 1) as usize];
    }

    pub fn store(&mut self, hash: u64, depth: i32) -> &mut TTEntry {
        let index = hash % self.size;
        if index + 1 == self.size {
            return &mut self.entries[index as usize];
        }

        if self.entries[index as usize].depth <= depth {
            return &mut self.entries[index as usize];
        }

        return &mut self.entries[(index + 1) as usize];
    }
}