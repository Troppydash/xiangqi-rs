use std::cmp::max;

pub struct SearchParameters;

impl SearchParameters {
    pub const Window: i32 = 15;
    pub const Checkmate: f32 = 9000.0;
    pub const Win: f32 = 10000.0;
    pub const Size: u64 = 64;
    pub const Buckets: i32 = 2;
    pub const AlphaFlag: i8 = 1;
    pub const BetaFlag: i8 = 2;
    pub const ExactFlag: i8 = 3;

    pub const StaticNullMovePruningBaseMargin: i32 = 10;
    pub const NMRDepthLimit: i32 = 4;

    pub const SingularExtensionDepthLimit: i32 = 4;
    pub const SingularMoveMargin: f32 = 20.0;
    pub const SingularMoveExtension: i32 = 1;

    pub const LMRLegalMovesLimit: i32 = 4;
    pub const LMRDepthLimit: i32 = 3;
    pub const LateMovePruningMargins: [i32; 6] = [0,8,12,16,20,24];

    pub const FutilityPruningDepthLimit: i32 = 8;
    pub const FutilityMargins: [i32;9] = [0, 20, 25, 30, 35, 40, 45, 50, 55];

    pub const IIDDepthReduction: i32 = 2;
    pub const IIDDepthLimit: i32 = 2;

    pub const MaxDepth: i32 = 100;

    pub const MaxKillers: i32 = 2;
    pub const FirstKillerMoveScore: i32 = 10;
    pub const SecondKillerMoveScore: i32 = 20;
    pub const CounterMoveBonus: i32 = 5;
    pub const MvvLvaOffset: i32 = 60000 - 256;
    pub const MaxHistoryScore: i32 = Self::MvvLvaOffset - 30;
    pub const PVMoveScore: i32 = 65;


    pub fn LMR(depth: i32, cnt: i32) -> i32 {
        (max(2, depth / 4) + cnt / 12)
    }
}