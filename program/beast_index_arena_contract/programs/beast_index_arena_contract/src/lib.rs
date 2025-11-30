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

    pub fn initialize_market(ctx: Context<InitializeMarket>, battle_id: u64, initial_liquidity: u64,) -> Result<()> {
        let market = &mut ctx.accounts.market_state;
        
        market.battle_id = battle_id;
        market.creature_0_pool = 0;
        market.creature_1_pool = 0;
        market.creature_2_pool = 0;
        market.creature_3_pool = 0;
        market.total_pool = 0;
        market.is_settled = false;

        market.creature_0_shares = initial_liquidity;
        market.creature_1_shares = initial_liquidity;
        market.creature_2_shares = initial_liquidity;
        market.creature_3_shares = initial_liquidity;
        market.k_constant = (initial_liquidity as u128).pow(2);
        market.bump = ctx.bumps.market_state;
        
        msg!("Market initialized for battle {}", battle_id);
        
        Ok(())
    }

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

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        creature_index: u8,
        amount: u64,
    ) -> Result<()> {
        let market =&mut ctx.accounts.market_state;
        let battle =&ctx.accounts.battle_state;
        let position =&mut ctx.accounts.user_position;

        require!(creature_index<4, GameError::InvalidCreatureIndex);
        require!(!battle.is_battle_over, GameError::BattleAlreadyOver);
        require!(amount >= 10_000_000, GameError::BetTooSmall);
        require!(
            battle.is_alive[creature_index as usize],
            GameError::CreatureIsDead
        );

        let current_shares = match creature_index {
            0 => market.creature_0_shares,
            1 => market.creature_1_shares,
            2 => market.creature_2_shares,
            3 => market.creature_3_shares,
            _ => return Err(GameError::InvalidCreatureIndex.into()),
        };
        let shares_bought = calculate_buy_shares(
            current_shares,
            amount,
            market.k_constant,
        )?;    

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer{
            from: ctx.accounts.user.to_account_info(),
            to: market.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_context, amount)?;    
    match creature_index {
        0 => {
            market.creature_0_pool += amount;
            market.creature_0_shares -= shares_bought;  
        },
        1 => {
            market.creature_1_pool += amount;
            market.creature_1_shares -= shares_bought;
        },
        2 => {
            market.creature_2_pool += amount;
            market.creature_2_shares -= shares_bought;
        },
        3 => {
            market.creature_3_pool += amount;
            market.creature_3_shares -= shares_bought;
        },
        _ => return Err(GameError::InvalidCreatureIndex.into()),
    }
    
    market.total_pool += amount;
    
    position.user = ctx.accounts.user.key();
    position.battle_id = battle.battle_id;
    position.creature_index = creature_index;
    position.amount = shares_bought; 
    position.claimed = false;
    position.bump = ctx.bumps.user_position;
    
    msg!(
        "{} bought {} shares of Creature {} for {} lamports",
        ctx.accounts.user.key(),
        shares_bought,
        creature_index,
        amount
    );
    
    let current_price = get_share_price(
        match creature_index {
            0 => market.creature_0_pool,
            1 => market.creature_1_pool,
            2 => market.creature_2_pool,
            3 => market.creature_3_pool,
            _ => 0,
        },
        match creature_index {
            0 => market.creature_0_shares,
            1 => market.creature_1_shares,
            2 => market.creature_2_shares,
            3 => market.creature_3_shares,
            _ => 0,
        },
    )?;
    msg!("Current price per share: {}", current_price);
    
        Ok(())
    }

    pub fn sell_shares(
        ctx: Context<SellShares>,
        shares_to_sell: u64,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market_state;
        let battle = &ctx.accounts.battle_state;
        let position = &mut ctx.accounts.user_position;
        
        require!(!battle.is_battle_over, GameError::BattleAlreadyOver);
        
        require!(
            shares_to_sell <= position.amount,
            GameError::InsufficientShares
        );
        
        let creature_index = position.creature_index;
        
        let current_shares = match creature_index {
            0 => market.creature_0_shares,
            1 => market.creature_1_shares,
            2 => market.creature_2_shares,
            3 => market.creature_3_shares,
            _ => return Err(GameError::InvalidCreatureIndex.into()),
        };
        
        let sol_returned = calculate_sell_price(
            current_shares,
            shares_to_sell,
            market.k_constant,
        )?;
        
        let battle_id_bytes = battle.battle_id.to_le_bytes();
        let seeds = &[
            b"market",
            battle_id_bytes.as_ref(),
            &[market.bump],
        ];
        let signer = &[&seeds[..]];
        
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &market.key(),
            &ctx.accounts.user.key(),
            sol_returned,
        );
        
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_ix,
            &[
                market.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer,
        )?;
        
        match creature_index {
            0 => {
                market.creature_0_pool -= sol_returned;
                market.creature_0_shares += shares_to_sell;  
            },
            1 => {
                market.creature_1_pool -= sol_returned;
                market.creature_1_shares += shares_to_sell;
            },
            2 => {
                market.creature_2_pool -= sol_returned;
                market.creature_2_shares += shares_to_sell;
            },
            3 => {
                market.creature_3_pool -= sol_returned;
                market.creature_3_shares += shares_to_sell;
            },
            _ => return Err(GameError::InvalidCreatureIndex.into()),
        }
        
        market.total_pool -= sol_returned;
        
        position.amount -= shares_to_sell;
        
        if position.amount == 0 {
            position.claimed = true;
        }
        
        msg!(
            "{} sold {} shares of Creature {} for {} lamports",
            ctx.accounts.user.key(),
            shares_to_sell,
            creature_index,
            sol_returned
        );
        
        Ok(())
    }

    pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
        let battle = &ctx.accounts.battle_state;
        let market = &mut ctx.accounts.market_state;
        let position = &mut ctx.accounts.user_position;
        let user = &ctx.accounts.user;
        require!(battle.is_battle_over, GameError::BattleNotOver);
        let winner = battle.winner.ok_or(GameError::NoWinner)?;
        require!(
            position.creature_index == winner,
            GameError::NotAWinner
        );
        require!(!position.claimed, GameError::AlreadyClaimed);
        let winning_pool = match winner {
            0 => market.creature_0_pool,
            1 => market.creature_1_pool,
            2 => market.creature_2_pool,
            3 => market.creature_3_pool,
            _ => return Err(GameError::InvalidCreatureIndex.into()),
        };
        
        let payout = (position.amount as u128)
        .checked_mul(market.total_pool as u128)
        .ok_or(GameError::CalculationOverflow)?
        .checked_div(winning_pool as u128)
        .ok_or(GameError::DivisionByZero)?
        as u64;

         let battle_id_bytes = battle.battle_id.to_le_bytes();
    let seeds = &[
        b"market",
        battle_id_bytes.as_ref(),
        &[market.bump],
    ];
    let signer = &[&seeds[..]];
    
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &market.key(),
        &user.key(),
        payout,
    );
    
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_ix,
        &[
            market.to_account_info(),
            user.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer,
    )?;
    position.claimed = true;
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
#[instruction(battle_id: u64)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = MarketState::LEN,
        seeds = [b"market", battle_id.to_le_bytes().as_ref()],
        bump
    )]
    pub market_state: Account<'info, MarketState>,
    
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

#[derive(Accounts)]
#[instruction(creature_index:u8, amount: u64)]
pub struct PlaceBet<'info> {
    #[account(
        mut, 
        seeds=[b"market", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = market_state.bump,
    )]
    pub market_state: Account<'info, MarketState>,

    #[account(
        seeds = [b"battle", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = battle_state.bump,
    )]
    pub battle_state: Account<'info, BattleState>,
    #[account(
        init,
        payer = user,
        space = UserPosition::LEN,
        seeds = [
            b"position",
            battle_state.battle_id.to_le_bytes().as_ref(),
            user.key().as_ref(),
            &[creature_index]
        ],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct SellShares<'info> {
    #[account(
        seeds = [b"battle", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = battle_state.bump,
    )]
    pub battle_state: Account<'info, BattleState>,
    
    #[account(
        mut,
        seeds = [b"market", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = market_state.bump,
    )]
    pub market_state: Account<'info, MarketState>,
    
    #[account(
        mut,
        seeds = [
            b"position",
            battle_state.battle_id.to_le_bytes().as_ref(),
            user.key().as_ref(),
            &[user_position.creature_index]
        ],
        bump = user_position.bump,
        has_one = user,
    )]
    pub user_position: Account<'info, UserPosition>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(
        seeds = [b"battle", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = battle_state.bump,
    )]
    pub battle_state: Account<'info, BattleState>,

    #[account(
        mut,
        seeds = [b"market", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = market_state.bump,
    )]
    pub market_state: Account<'info, MarketState>,
    #[account(
        mut,
        seeds = [
            b"position",
            battle_state.battle_id.to_le_bytes().as_ref(),
            user.key().as_ref(),
            &[user_position.creature_index]
        ],
        bump = user_position.bump,
        has_one = user,
    )]
    pub user_position: Account<'info, UserPosition>,
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
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

    pub start_time: i64,
    pub last_turn_time: i64,
    pub turn_interval: i64,
    pub max_duration: i64,

    pub bump: u8,
}

#[account]
pub struct TurnLog {
    pub battle_id: u64,
    pub turn_number: u64,
    pub timestamp: i64,
    pub attacks: [Attack; 4],
    pub attack_count: u8,
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
        + 8
        + 8
        + 8
        + 8
        + 1
        + 100;
}

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

#[account]
pub struct MarketState {
    pub battle_id: u64,

    pub creature_0_pool: u64,
    pub creature_1_pool: u64,
    pub creature_2_pool: u64,
    pub creature_3_pool: u64,

    pub total_pool: u64,
    pub is_settled: bool,

    pub creature_0_shares: u64,  
    pub creature_1_shares: u64,
    pub creature_2_shares: u64,
    pub creature_3_shares: u64,
    
    pub k_constant: u128,

    pub bump: u8,
}

impl MarketState {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 8 + 16 + 1 + 100;
}

impl TurnLog {
    pub const LEN: usize = 8 + 8 + 8 + 8 + (Attack::LEN * 4) + 1 + 1 + 50;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct Attack {
    pub attacker: u8,
    pub target: u8,
    pub ability: Ability,
    pub damage: u16,
    pub target_hp: u16,
    pub target_died: bool,
}

impl Attack {
    pub const LEN: usize = 1 + 1 + 1 + 2 + 2 + 1;

    pub fn default() -> Self {
        Self {
            attacker: 0,
            target: 0,
            ability: Ability::BasicHit,
            damage: 0,
            target_hp: 0,
            target_died: false,
        }
    }
}

fn get_random_seed(clock: &Clock, salt: u64) -> u64 {
    let slot = clock.slot;
    let timestamp = clock.unix_timestamp as u64;

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

fn calculate_buy_shares(
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

fn calculate_sell_price(
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

fn get_share_price(pool: u64, shares: u64) -> Result<u64> {
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

#[error_code]
pub enum GameError {
    #[msg("Battle is already over")]
    BattleAlreadyOver,

    #[msg("Turn interval not met - wait longer between turns")]
    TurnIntervalNotMet,

    #[msg("Battle has exceeded maximum duration")]
    BattleDurationExceeded,
    #[msg("Invalid creature index (must be 0-3)")]
    InvalidCreatureIndex,
    
    #[msg("Bet amount too small (minimum 0.01 SOL)")]
    BetTooSmall,
    
    #[msg("Cannot bet on a dead creature")]
    CreatureIsDead,
    #[msg("Battle is not over yet")]
    BattleNotOver,
    
    #[msg("Battle has no winner (draw or timeout)")]
    NoWinner,
    
    #[msg("You didn't bet on the winning creature")]
    NotAWinner,
    
    #[msg("You already claimed your winnings")]
    AlreadyClaimed,
    
    #[msg("Calculation overflow")]
    CalculationOverflow,
    
    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Insufficient shares to sell")]
    InsufficientShares,
}
