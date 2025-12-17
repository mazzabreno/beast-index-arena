use anchor_lang::prelude::*;
use crate::state::Ability;
use crate::errors::GameError;

pub fn get_random_seed(clock: &Clock, salt: u64) -> u64 {
    let slot = clock.slot;
    let timestamp = clock.unix_timestamp as u64;

    let mixed1 = slot.wrapping_mul(0x9e3779b97f4a7c15);
    let mixed2 = timestamp.wrapping_mul(0x517cc1b727220a95);
    let mixed3 = salt.wrapping_mul(0x85ebca77c2b2ae63);

    let result = mixed1 ^ mixed2 ^ mixed3;
    result.wrapping_mul(result >> 32).wrapping_add(salt)
}

pub fn pick_random_target(
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

pub fn pick_random_ability(random_seed: u64) -> Ability {
    let choice = random_seed % 3;
    match choice {
        0 => Ability::BasicHit,
        1 => Ability::HeavyStrike,
        2 => Ability::QuickJab,
        _ => Ability::BasicHit,
    }
}

pub fn calculate_damage(atk: u16, def: u16, ability: Ability) -> u16 {
    let base_damage = atk.saturating_sub(def);
    let modified_damage = match ability {
        Ability::BasicHit => base_damage,
        Ability::HeavyStrike => base_damage * 3 / 2,
        Ability::QuickJab => base_damage * 3 / 4,
    };
    modified_damage.max(1)
}

pub fn calculate_buy_shares(
    current_shares: u64,
    sol_amount: u64,
    k_constant: u128,
) -> Result<u64> {
    let current_shares_u128 = current_shares as u128;
    let sol_amount_u128 = sol_amount as u128;

    let new_shares = k_constant
        .checked_div(current_shares_u128 + sol_amount_u128)
        .ok_or(GameError::CalculationOverflow)?;

    let shares_bought = current_shares_u128
        .checked_sub(new_shares)
        .ok_or(GameError::CalculationOverflow)? as u64;

    Ok(shares_bought)
}

pub fn calculate_sell_price(
    current_shares: u64,
    shares_to_sell: u64,
    k_constant: u128,
) -> Result<u64> {
    let current_shares_u128 = current_shares as u128;
    let shares_to_sell_u128 = shares_to_sell as u128;

    let new_pool = k_constant
        .checked_div(current_shares_u128 + shares_to_sell_u128)
        .ok_or(GameError::CalculationOverflow)?;

    let current_pool = k_constant
        .checked_div(current_shares_u128)
        .ok_or(GameError::CalculationOverflow)?;

    let sol_returned = current_pool
        .checked_sub(new_pool)
        .ok_or(GameError::CalculationOverflow)? as u64;

    Ok(sol_returned)
}

pub fn get_share_price(pool: u64, shares: u64) -> Result<u64> {
    if shares == 0 {
        return Ok(0);
    }

    let price = (pool as u128)
        .checked_mul(1_000_000_000)
        .ok_or(GameError::CalculationOverflow)?
        .checked_div(shares as u128)
        .ok_or(GameError::DivisionByZero)? as u64;

    Ok(price)
}
