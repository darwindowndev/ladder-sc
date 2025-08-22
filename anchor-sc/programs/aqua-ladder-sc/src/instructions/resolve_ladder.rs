use crate::{
    addresses::ADMIN_PUBKEY, pda::LADDER_SEED, state::ladder_information::LadderInformation,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ResolveLadder<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = ADMIN_PUBKEY)]
    pub admin: Signer<'info>,

    #[account(mut, seeds = [LADDER_SEED], bump)]
    pub ladder_information: Account<'info, LadderInformation>,

    pub system_program: Program<'info, System>,
}

pub fn handle_resolve_ladder(ctx: Context<ResolveLadder>) -> Result<()> {
    msg!("Ladder resolution: {:?}", ctx.program_id);
    let clock = Clock::get()?;
    let block_timestamp = clock.unix_timestamp;
    let slot = clock.slot;
    // combine block timestamp and slot (block number) for more randomization.
    let combined_seed = (block_timestamp as u64)
        .wrapping_add(slot)
        .wrapping_mul(6364136223846793005)
        .rotate_left(17)
        ^ slot.rotate_right(13);
    msg!("Randomization factor: {}", combined_seed);
    msg!("Current block timestamp: {}", block_timestamp);
    msg!("Current slot (block number): {}", slot);

    ctx.accounts.ladder_information.result_seed = combined_seed;
    ctx.accounts.ladder_information.is_ladder_finished = true;
    ctx.accounts.ladder_information.closed_at = block_timestamp as u64;

    Ok(())
}
