use anchor_lang::prelude::*;
use crate::state::BattleState;
use crate::errors::GameError;
use crate::utils::{get_random_seed, pick_random_target, pick_random_ability, calculate_damage};

pub fn execute_turn(ctx: Context<ExecuteTurn>) -> Result<()> {
    let battle = &mut ctx.accounts.battle_state;
    let clock = &ctx.accounts.clock;

    require!(!battle.is_battle_over, GameError::BattleAlreadyOver);

    let time_since_last_turn = clock.unix_timestamp - battle.last_turn_time;
    require!(
        time_since_last_turn >= battle.turn_interval,
        GameError::TurnIntervalNotMet
    );

    let battle_duration = clock.unix_timestamp - battle.start_time;
    if battle_duration > battle.max_duration {
        battle.is_battle_over = true;
        battle.winner = None;
        msg!("Battle timed out after {} seconds!", battle_duration);
        return Ok(());
    }

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
        let random_seed = get_random_seed(
            clock,
            battle.current_turn.wrapping_mul(10).wrapping_add(attacker_idx as u64),
        );
        let target_idx = match pick_random_target(attacker_idx, &battle.is_alive, random_seed) {
            Some(idx) => idx,
            None => {
                msg!("Creature {} has no valid targets", attacker_idx);
                continue;
            }
        };

        let ability_seed = get_random_seed(
            clock,
            battle.current_turn.wrapping_mul(100).wrapping_add(attacker_idx as u64),
        );
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
            msg!(" Creature {} died!", target_idx);
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

    battle.last_turn_time = clock.unix_timestamp;

    battle.current_turn += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteTurn<'info> {
    #[account(
        mut,
        seeds = [b"battle", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = battle_state.bump,
    )]
    pub battle_state: Account<'info, BattleState>,
    pub executer: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}
