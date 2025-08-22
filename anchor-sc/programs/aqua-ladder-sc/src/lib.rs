use anchor_lang::prelude::*;

use instructions::{initialize_ladder::*, participate_in_ladder::*, resolve_ladder::*};

pub mod addresses;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod pda;
pub mod state;

declare_id!("C8C53TRUjHe6S29v76Y2srvCoBMR7VMJkWMwtNFJxNCX");

#[program]
pub mod aqua_ladder_sc {

    use super::*;

    pub fn initialize_ladder(ctx: Context<InitializeLadder>) -> Result<()> {
        handle_initialize_ladder(ctx)
    }

    pub fn participate_in_ladder(ctx: Context<ParticipateInLadder>, amount: u64) -> Result<()> {
        handle_participate_in_ladder(ctx, amount)
    }

    pub fn resolve_ladder(ctx: Context<ResolveLadder>) -> Result<()> {
        handle_resolve_ladder(ctx)
    }
}
