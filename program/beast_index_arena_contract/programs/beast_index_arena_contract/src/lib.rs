use anchor_lang::prelude::*;

declare_id!("28VkZmQABZWqq3gmossB41hYF9846gG2TWMyk4u6jTd4");

#[program]
pub mod beast_index_arena_contract {
    use super::*;

    pub fn initialize_battle(
        ctx: Context<InitializeBattle>,
        battle_id: u64,
        hp: u16,
        atk: u16,
        def: u16,
    ) -> Result<()> {
        let battle = &mut ctx.accounts.battle_state;
        battle.battle_id = battle_id;
        battle.authourity = ctx.accounts.authourity.key();

        battle.creature_a_hp = hp;
        battle.creature_a_atk = atk;
        battle.creature_a_def = def;
        battle.creature_a_max_hp = hp;

        battle.creature_b_hp = hp;
        battle.creature_b_atk = atk;
        battle.creature_b_def = def;
        battle.creature_b_max_hp = hp;

        battle.is_battle_over = false;
        battle.winner = None;
        battle.current_turn = 0;
        battle.bump = ctx.bumps.battle_state;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(battle_id: u64)]
pub struct InitializeBattle<'info> {
    #[account(
        init,
        payer= authourity,
        space= BattleState::LEN,
        seeds= [b"battle", battle_id.to_le_bytes().as_ref()],
        bump
    )]
    pub battle_state: Account<'info, BattleState>,

    #[account(mut)]
    pub authourity: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BattleState {
    pub battle_id: u64,
    pub authourity: Pubkey,

    pub creature_a_hp: u16,
    pub creature_a_max_hp: u16,
    pub creature_a_atk: u16,
    pub creature_a_def: u16,

    pub creature_b_hp: u16,
    pub creature_b_max_hp: u16,
    pub creature_b_atk: u16,
    pub creature_b_def: u16,

    pub is_battle_over: bool,
    pub winner: Option<u8>,
    pub current_turn: u64,

    pub bump: u8,
}

impl BattleState {
    pub const LEN: usize = 8 + 8 + 32 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 1 + 1 + 1 + 8 + 1 + 100;
}
