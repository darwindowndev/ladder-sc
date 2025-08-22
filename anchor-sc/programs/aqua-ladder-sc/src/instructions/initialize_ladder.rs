use anchor_lang::prelude::*;

use crate::{
    addresses::ADMIN_PUBKEY, constants::DISCRIMINATOR, pda::LADDER_SEED,
    state::ladder_information::LadderInformation,
};

#[derive(Accounts)]
pub struct InitializeLadder<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = ADMIN_PUBKEY)]
    pub admin: Signer<'info>,

    #[account(
        init,
        space = DISCRIMINATOR + LadderInformation::INIT_SPACE,
        payer = payer,
        seeds = [LADDER_SEED],
        bump,
    )]
    pub ladder_information: Account<'info, LadderInformation>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_ladder(ctx: Context<InitializeLadder>) -> Result<()> {
    msg!("Ladder initialization: {:?}", ctx.program_id);
    ctx.accounts.ladder_information.is_initialized = true;
    ctx.accounts.ladder_information.liquidity_lamports = 0;
    ctx.accounts.ladder_information.result_seed = 0;
    ctx.accounts.ladder_information.is_ladder_finished = false;
    ctx.accounts.ladder_information.initialized_at = Clock::get()?.unix_timestamp as u64;
    ctx.accounts.ladder_information.closed_at = 0;
    Ok(())
}
