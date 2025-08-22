use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

pub const DISCRIMINATOR: usize = 8;

// ladder constants
pub const LADDER_INITIAL_CAP: u64 = 1111 * LAMPORTS_PER_SOL;
pub struct LadderLevels;
impl LadderLevels {
    // ladder levels sum should be equal to LADDER_INITIAL_CAP
    pub const LEVEL_1: u64 = 30 * LAMPORTS_PER_SOL;
    pub const LEVEL_2: u64 = 35 * LAMPORTS_PER_SOL;
    pub const LEVEL_3: u64 = 50 * LAMPORTS_PER_SOL;
    pub const LEVEL_4: u64 = 137 * LAMPORTS_PER_SOL;
    pub const LEVEL_5: u64 = 190 * LAMPORTS_PER_SOL;
    pub const LEVEL_6: u64 = 220 * LAMPORTS_PER_SOL;
    pub const LEVEL_7: u64 = 236 * LAMPORTS_PER_SOL;
    pub const LEVEL_8: u64 = 130 * LAMPORTS_PER_SOL;
    pub const LEVEL_9: u64 = 53 * LAMPORTS_PER_SOL;
    pub const LEVEL_10: u64 = 30 * LAMPORTS_PER_SOL;
}

// participation constraints
pub const MINIMUM_PARTICIPATION_SOL: u64 = LAMPORTS_PER_SOL / 10; // 0.1 SOL
pub const MAXIMUM_PARTICIPATION_SOL: u64 = 15 * LAMPORTS_PER_SOL; // 15 SOL
