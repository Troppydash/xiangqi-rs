use std::cmp::max;

pub struct SearchParameters;

impl SearchParameters {
    pub const Window: i32 = 30;
    pub const Checkmate: f32 = 9000.0;
    
    pub const Size: u64 = 64;
    pub const Buckets: i32 = 2;
    pub const AlphaFlag: i8 = 1;
    pub const BetaFlag: i8 = 1;
    pub const ExactFlag: i8 = 1;
    
    pub const StaticNullMovePruningBaseMargin: i32 = 60;
    pub const NMRDepthLimit: i32 = 2;
    pub const SingularExtensionDepthLimit: i32 = 4;
    pub const LMRLegalMovesLimit: i32 = 4;
    pub const LMRDepthLimit: i32 = 3;
    
    pub fn LMR() -> Vec<Vec<i32>> {
        let mut lmr = vec![vec![0; 100]; 1000];
        for depth in 3..1000 {
            for cnt in 3..100usize {
                lmr[depth][cnt] = (max(2, depth / 4) + cnt / 12) as i32;
            }
        }

        lmr
    }
}