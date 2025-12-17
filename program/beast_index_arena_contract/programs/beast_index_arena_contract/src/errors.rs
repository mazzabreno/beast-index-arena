use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Battle is already over")]
    BattleAlreadyOver,

    #[msg("Turn interval not met - wait longer between turns")]
    TurnIntervalNotMet,

    #[msg("Battle has exceeded maximum duration")]
    BattleDurationExceeded,
    #[msg("Invalid creature index (must be 0-3)")]
    InvalidCreatureIndex,

    #[msg("Bet amount too small (minimum 0.01 SOL)")]
    BetTooSmall,

    #[msg("Cannot bet on a dead creature")]
    CreatureIsDead,
    #[msg("Battle is not over yet")]
    BattleNotOver,

    #[msg("Battle has no winner (draw or timeout)")]
    NoWinner,

    #[msg("You didn't bet on the winning creature")]
    NotAWinner,

    #[msg("You already claimed your winnings")]
    AlreadyClaimed,

    #[msg("Calculation overflow")]
    CalculationOverflow,

    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Insufficient shares to sell")]
    InsufficientShares,
}
