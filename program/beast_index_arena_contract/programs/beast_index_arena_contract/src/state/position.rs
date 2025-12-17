use anchor_lang::prelude::*;

#[account]
pub struct UserPosition {
    pub user: Pubkey,
    pub battle_id: u64,
    pub creature_index: u8,
    pub amount: u64,
    pub claimed: bool,
    pub bump: u8,
}

impl UserPosition {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8 + 1 + 1 + 50;
}
