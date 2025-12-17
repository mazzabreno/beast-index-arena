use anchor_lang::prelude::*;
use crate::state::GlobalState;

pub fn update_current_battle(ctx: Context<UpdateCurrentBattle>, battle_id: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.current_battle_id = battle_id;
    msg!("Current battle ID updated to: {}", battle_id);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateCurrentBattle<'info> {
    #[account(
        mut,
        seeds = [b"global"],
        bump = global_state.bump,
        has_one = authority,
    )]
    pub global_state: Account<'info, GlobalState>,

    pub authority: Signer<'info>,
}
