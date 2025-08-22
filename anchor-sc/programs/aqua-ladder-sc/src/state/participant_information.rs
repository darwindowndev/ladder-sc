use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ParticipantInformation {
    pub participation_lamports: u64, // how much the user has participated
}
