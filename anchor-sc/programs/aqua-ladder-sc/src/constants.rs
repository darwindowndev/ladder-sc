use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

pub const DISCRIMINATOR: usize = 8;

// participation constraints
pub const MINIMUM_PARTICIPATION_SOL: u64 = LAMPORTS_PER_SOL; // 1 SOL
pub const MAXIMUM_PARTICIPATION_SOL: u64 = 30 * LAMPORTS_PER_SOL; // 30 SOL
