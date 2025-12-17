use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub current_battle_id: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

impl GlobalState {
    pub const LEN: usize = 8 + 8 + 32 + 1 + 50;
}
