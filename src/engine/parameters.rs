use std::cmp::max;

pub struct SearchParameters;

impl SearchParameters {
    pub const Window: i32 = 30;
    pub const Checkmate: f32 = 9000.0;
    pub const Win: f32 = 10000.0;

    pub const Size: u64 = 64;
    pub const Buckets: i32 = 2;
    pub const AlphaFlag: i8 = 1;
    pub const BetaFlag: i8 = 2;
    pub const ExactFlag: i8 = 3;

    pub const StaticNullMovePruningBaseMargin: i32 = 60;
    pub const NMRDepthLimit: i32 = 2;
    pub const SingularExtensionDepthLimit: i32 = 4;
    pub const LMRLegalMovesLimit: i32 = 4;
    pub const LMRDepthLimit: i32 = 3;
    pub const MaxDepth: i32 = 30;
    
    pub const MvvLvaOffset: f32 = 60000.0 - 256.0;
    pub const MaxHistoryScore: f32 = Self::MvvLvaOffset - 30.0;
    pub const PVMoveScore: f32 = 65.0;
    

    pub fn LMR(depth: i32, cnt: i32) -> i32 {
        (max(2, depth / 4) + cnt / 12) as i32
    }
}