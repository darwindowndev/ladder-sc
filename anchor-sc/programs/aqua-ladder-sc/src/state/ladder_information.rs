use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LadderInformation {
    pub is_initialized: bool,
    pub liquidity_lamports: u64, // amount that has been already stored

    pub is_ladder_finished: bool,
    pub result_seed: u64, // this seed will be used to resolve the ladder using a public algorithm

    pub initialized_at: u64, // the timestamp when the ladder is initialized
    pub closed_at: u64,      // the timestamp when the ladder is closed
}
