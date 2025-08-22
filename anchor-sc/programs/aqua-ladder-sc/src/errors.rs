use anchor_lang::prelude::*;

#[error_code]
pub enum LadderErrorCode {
    #[msg("Amount must be greater than the minimum participation")]
    LessThanMinimumParticipation,
    #[msg("Amount exceeds the maximum participation")]
    ExceedsMaximumParticipation,
    #[msg("Ladder is not initialized")]
    NotInitialized,
    #[msg("Ladder is already finished")]
    AlreadyFinished,
    #[msg("Ladder is not finished")]
    NotFinished,
}
