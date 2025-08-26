use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

pub const DISCRIMINATOR: usize = 8;

// ladder constants
pub const LADDER_INITIAL_CAP: u64 = 2222 * LAMPORTS_PER_SOL;

// participation constraints
pub const MINIMUM_PARTICIPATION_SOL: u64 = LAMPORTS_PER_SOL / 10; // 0.1 SOL
pub const MAXIMUM_PARTICIPATION_SOL: u64 = 20 * LAMPORTS_PER_SOL; // 20 SOL
