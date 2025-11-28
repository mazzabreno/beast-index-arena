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
        battle.authority = ctx.accounts.authority.key();

        battle.creature_hp = [hp, hp, hp, hp];
        battle.creature_atk = [atk, atk, atk, atk];
        battle.creature_def = [def, def, def, def];
        battle.creature_max_hp = [hp, hp, hp, hp];
        battle.is_alive = [true, true, true, true];

        battle.is_battle_over = false;
        battle.winner = None;
        battle.current_turn = 0;
        battle.bump = ctx.bumps.battle_state;

        Ok(())
    }

    pub fn execute_turn(ctx: Context<ExecuteTurn>) -> Result<()> {
        let battle = &mut ctx.accounts.battle_state;
        let clock = &ctx.accounts.clock;

        require!(!battle.is_battle_over, GameError::BattleAlreadyOver);

        for attacker_idx in 0..4 {
            if !battle.is_alive[attacker_idx] {
                continue;
            }
            let random_seed = get_random_seed(clock, attacker_idx as u64);
            let target_idx = match pick_random_target(attacker_idx, &battle.is_alive, random_seed) {
                Some(idx) => idx,
                None => {
                    msg!("Creature {} has no valid targets", attacker_idx);
                    continue;
                }
            };

            // if !battle.is_alive[target_idx] || target_idx == attacker_idx {
            //     continue;
            // }

            let damage = battle.creature_atk[attacker_idx]
                .saturating_sub(battle.creature_def[target_idx])
                .max(1);

            battle.creature_hp[target_idx] = battle.creature_hp[target_idx].saturating_sub(damage);

            if battle.creature_hp[target_idx] == 0 {
                battle.is_alive[target_idx] = false;
                msg!("Creature {} died!", target_idx);
            }
        }
        let alive_creatures: Vec<usize> = battle
            .is_alive
            .iter()
            .enumerate()
            .filter(|(_, &alive)| alive)
            .map(|(idx, _)| idx)
            .collect();

        if alive_creatures.len() == 1 {
            battle.is_battle_over = true;
            battle.winner = Some(alive_creatures[0] as u8);
            msg!("Creature {} WINS!", alive_creatures[0]);
        } else if alive_creatures.len() == 0 {
            battle.is_battle_over = true;
            battle.winner = None;
            msg!("All creatures died! It's a draw!");
        }

        battle.current_turn += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(battle_id: u64)]
pub struct InitializeBattle<'info> {
    #[account(
        init,
        payer= authority,
        space= BattleState::LEN,
        seeds= [b"battle", battle_id.to_le_bytes().as_ref()],
        bump
    )]
    pub battle_state: Account<'info, BattleState>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTurn<'info> {
    #[account(
        mut,
        seeds = [b"battle", battle_state.battle_id.to_le_bytes().as_ref()],
        bump  = battle_state.bump,
    )]
    pub battle_state: Account<'info, BattleState>,
    pub executer: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct BattleState {
    pub battle_id: u64,
    pub authority: Pubkey,

    pub creature_hp: [u16; 4],
    pub creature_max_hp: [u16; 4],
    pub creature_atk: [u16; 4],
    pub creature_def: [u16; 4],
    pub is_alive: [bool; 4],

    pub is_battle_over: bool,
    pub winner: Option<u8>,
    pub current_turn: u64,

    pub bump: u8,
}

impl BattleState {
    pub const LEN: usize =
        8 + 8 + 32 + (2 * 4) + (2 * 4) + (2 * 4) + (2 * 4) + (1 * 4) + 1 + 2 + 8 + 1 + 100;
}

fn get_random_seed(clock: &Clock, salt: u64) -> u64 {
    let slot = clock.slot;
    let timestamp = clock.unix_timestamp as u64;

    // Simple but effective randomness for MVP
    slot.wrapping_mul(timestamp).wrapping_add(salt)
}

fn pick_random_target(
    attacker_idx: usize,
    is_alive: &[bool; 4],
    random_seed: u64,
) -> Option<usize> {
    let mut valid_targets = Vec::new();
    for i in 0..4 {
        if i == attacker_idx {
            continue;
        }
        if !is_alive[i] {
            continue;
        }
        valid_targets.push(i);
    }
    if valid_targets.is_empty() {
        return None;
    }

    let random_index = (random_seed as usize) % valid_targets.len();

    Some(valid_targets[random_index])
}

#[error_code]
pub enum GameError {
    #[msg("Battle is already over")]
    BattleAlreadyOver,
}
