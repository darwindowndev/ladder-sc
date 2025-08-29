use crate::{
    addresses::MULTISIG_PUBKEY,
    constants::{DISCRIMINATOR, MAXIMUM_PARTICIPATION_SOL, MINIMUM_PARTICIPATION_SOL},
    errors::LadderErrorCode,
    pda::{LADDER_SEED, PARTICIPANT_SEED},
    state::{
        ladder_information::LadderInformation, participant_information::ParticipantInformation,
    },
};
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct ParticipateInLadder<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        space = DISCRIMINATOR + ParticipantInformation::INIT_SPACE,
        payer = payer,
        seeds = [PARTICIPANT_SEED, payer.key().as_ref()],
        bump,
    )]
    pub participant_information: Account<'info, ParticipantInformation>,

    #[account(
        mut,
        seeds = [LADDER_SEED],
        bump,
    )]
    pub ladder_information: Account<'info, LadderInformation>,

    #[account(mut, address = MULTISIG_PUBKEY)]
    pub multisig: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_participate_in_ladder(ctx: Context<ParticipateInLadder>, amount: u64) -> Result<()> {
    msg!("Participation in ladder: {:?}", ctx.program_id);

    // check if the ladder is initialized
    if !ctx.accounts.ladder_information.is_initialized {
        msg!("Ladder is not initialized");
        return Err(LadderErrorCode::NotInitialized.into());
    }

    // check if the ladder is finished
    if ctx.accounts.ladder_information.is_ladder_finished {
        msg!("Ladder is already finished");
        return Err(LadderErrorCode::AlreadyFinished.into());
    }

    // amount must be greater than the minimum
    if amount < MINIMUM_PARTICIPATION_SOL {
        msg!("Amount must be greater than the minimum participation");
        return Err(LadderErrorCode::LessThanMinimumParticipation.into());
    }

    let mut adjusted_amount = amount;
    // amount must be less than the maximum
    let active_participation = ctx.accounts.participant_information.participation_lamports;

    if active_participation + amount > MAXIMUM_PARTICIPATION_SOL {
        // adjusting the amount to the maximum participation instead of rejecting
        adjusted_amount = MAXIMUM_PARTICIPATION_SOL.saturating_sub(active_participation);

        if adjusted_amount == 0 {
            msg!("Maximum participation limit already reached");
            return Err(LadderErrorCode::ExceedsMaximumParticipation.into());
        }
    }

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.multisig.to_account_info(),
            },
        ),
        adjusted_amount,
    )?;

    // increasing the participant participation
    ctx.accounts.participant_information.participation_lamports = ctx
        .accounts
        .participant_information
        .participation_lamports
        .checked_add(adjusted_amount)
        .unwrap();

    // increasing the liquidity in the ladder information
    ctx.accounts.ladder_information.liquidity_lamports = ctx
        .accounts
        .ladder_information
        .liquidity_lamports
        .checked_add(adjusted_amount)
        .unwrap();

    Ok(())
}
