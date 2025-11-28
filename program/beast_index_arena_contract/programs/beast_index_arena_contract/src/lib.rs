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
        spd: u16,
    ) -> Result<()> {
        let battle = &mut ctx.accounts.battle_state;
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
        battle.bump = ctx.bumps.battle_state;

        Ok(())
    }

    pub fn execute_turn(ctx: Context<ExecuteTurn>) -> Result<()> {
        let battle = &mut ctx.accounts.battle_state;
        let clock = &ctx.accounts.clock;

        require!(!battle.is_battle_over, GameError::BattleAlreadyOver);

        let mut creature_order: Vec<(usize, u16)> = Vec::new();
        for i in 0..4 {
            if battle.is_alive[i] {
                creature_order.push((i, battle.creature_spd[i]));
            }
        }
        creature_order.sort_by(|a, b| b.1.cmp(&a.1));

        for (attacker_idx, _speed) in creature_order {
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

            // let damage = battle.creature_atk[attacker_idx]
            //     .saturating_sub(battle.creature_def[target_idx])
            //     .max(1);
            let ability_seed = get_random_seed(clock, (attacker_idx + 100) as u64);
            let ability = pick_random_ability(ability_seed);

            let damage = calculate_damage(
                battle.creature_atk[attacker_idx],
                battle.creature_def[target_idx],
                ability,
            );

            battle.creature_hp[target_idx] = battle.creature_hp[target_idx].saturating_sub(damage);

            msg!(
                "   Creature {} uses {:?}! Attacks Creature {} for {} damage! HP: {}",
                attacker_idx,
                ability,
                target_idx,
                damage,
                battle.creature_hp[target_idx]
            );

            if battle.creature_hp[target_idx] == 0 {
                battle.is_alive[target_idx] = false;
                msg!("   💀 Creature {} died!", target_idx);
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum Ability {
    BasicHit,
    HeavyStrike,
    QuickJab,
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
    pub creature_spd: [u16; 4],
    pub is_alive: [bool; 4],

    pub is_battle_over: bool,
    pub winner: Option<u8>,
    pub current_turn: u64,

    pub bump: u8,
}

impl BattleState {
    pub const LEN: usize = 8
        + 8
        + 32
        + (2 * 4)
        + (2 * 4)
        + (2 * 4)
        + (2 * 4)
        + (2 * 4)
        + (1 * 4)
        + 1
        + 2
        + 8
        + 1
        + 100;
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

fn pick_random_ability(random_seed: u64) -> Ability {
    let choice = random_seed % 3;
    match choice {
        0 => Ability::BasicHit,
        1 => Ability::HeavyStrike,
        2 => Ability::QuickJab,
        _ => Ability::BasicHit,
    }
}

fn calculate_damage(atk: u16, def: u16, ability: Ability) -> u16 {
    let base_damage = atk.saturating_sub(def);
    let modified_damage = match ability {
        Ability::BasicHit => base_damage,
        Ability::HeavyStrike => base_damage * 3 / 2,
        Ability::QuickJab => base_damage * 3 / 4,
    };
    modified_damage.max(1)
}

#[error_code]
pub enum GameError {
    #[msg("Battle is already over")]
    BattleAlreadyOver,
}
