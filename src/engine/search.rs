use crate::engine::tt::TT;

struct Search {
    tt: TT,
    history: Vec<Vec<Vec<f32>>>
}

impl Search {
    pub fn new() -> Self {
        Self {
            tt: TT::new(),
            history: vec![vec![vec![0.0; 90]; 90]; 2]
        }
    }
    
    fn score_moves() {
        
    }
    
    fn qsearch() {
        
    }
    
    fn increment_history() {
        
    }
    
    fn decrement_history() {
        
    }
    
    fn get_history() {
        
    }
    
    fn negamax() {
        
    }
    
    fn search() {
        
    }
}