use anchor_lang::prelude::*;
use crate::state::GlobalState;

pub fn initialize_global(ctx: Context<InitializeGlobal>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.current_battle_id = 0;
    global_state.authority = ctx.accounts.authority.key();
    global_state.bump = ctx.bumps.global_state;
    msg!("Global state initialized");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGlobal<'info> {
    #[account(
        init,
        payer = authority,
        space = GlobalState::LEN,
        seeds = [b"global"],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
