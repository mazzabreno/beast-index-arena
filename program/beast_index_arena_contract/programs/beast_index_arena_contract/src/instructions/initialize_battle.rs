use anchor_lang::prelude::*;
use crate::state::BattleState;

pub fn initialize_battle(
    ctx: Context<InitializeBattle>,
    battle_id: u64,
    hp: u16,
    atk: u16,
    def: u16,
    spd: u16,
    turn_interval: i64,
    max_duration: i64,
) -> Result<()> {
    let battle = &mut ctx.accounts.battle_state;
    let clock = Clock::get()?;
    battle.battle_id = battle_id;
    battle.authority = ctx.accounts.authority.key();

    battle.creature_hp = [hp, hp, hp, hp];
    battle.creature_atk = [atk, atk, atk, atk];
    battle.creature_def = [def, def, def, def];
    battle.creature_max_hp = [hp, hp, hp, hp];
    battle.creature_spd = [spd, spd, spd, spd];
    battle.is_alive = [true, true, true, true];

    battle.is_battle_over = false;
    battle.winner = None;
    battle.current_turn = 0;

    battle.start_time = clock.unix_timestamp;
    battle.last_turn_time = clock.unix_timestamp;
    battle.turn_interval = turn_interval;
    battle.max_duration = max_duration;

    battle.bump = ctx.bumps.battle_state;

    Ok(())
}

#[derive(Accounts)]
#[instruction(battle_id: u64)]
pub struct InitializeBattle<'info> {
    #[account(
        init,
        payer = authority,
        space = BattleState::LEN,
        seeds = [b"battle", battle_id.to_le_bytes().as_ref()],
        bump
    )]
    pub battle_state: Account<'info, BattleState>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
