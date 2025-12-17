use anchor_lang::prelude::*;

#[account]
pub struct MarketState {
    pub battle_id: u64,

    pub creature_0_pool: u64,
    pub creature_1_pool: u64,
    pub creature_2_pool: u64,
    pub creature_3_pool: u64,

    pub total_pool: u64,
    pub is_settled: bool,

    pub creature_0_shares: u64,
    pub creature_1_shares: u64,
    pub creature_2_shares: u64,
    pub creature_3_shares: u64,

    pub k_constant: u128,

    pub bump: u8,
}

impl MarketState {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 8 + 16 + 1 + 100;
}
