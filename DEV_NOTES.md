public testing: C9t6ou6s6SHZrZFy6AwiWoL2rihxFzYYoGiWPyKjRenR

INVARIANTS:

InitializeLadder:

- Initiatior caller must be the admin wallet addresses::ADMIN_PUBKEY

ParticipateInLadder:

- LadderInformation::is_initialized = true
- LadderInformation::is_ladder_finished = false
- User participation should be greater than constants::MINIMUM_PARTICIPATION_SOL
- User participation (previous ones + current one) should be less than constants::MAXIMUM_PARTICIPATION_SOL
- Destination for transfer must be addresses::MULTISIG_PUBKEY

ResolveLadder:

- Only callable from the admin wallet addresses::ADMIN_PUBKEY
