use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::{BattleState, MarketState, UserPosition};
use crate::errors::GameError;
use crate::utils::{calculate_buy_shares, get_share_price};

pub fn place_bet(
    ctx: Context<PlaceBet>,
    creature_index: u8,
    amount: u64,
) -> Result<()> {
    let market = &mut ctx.accounts.market_state;
    let battle = &ctx.accounts.battle_state;
    let position = &mut ctx.accounts.user_position;

    require!(creature_index < 4, GameError::InvalidCreatureIndex);
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
    let shares_bought = calculate_buy_shares(current_shares, amount, market.k_constant)?;

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: market.to_account_info(),
        },
    );
    transfer(cpi_context, amount)?;
    match creature_index {
        0 => {
            market.creature_0_pool += amount;
            market.creature_0_shares -= shares_bought;
        }
        1 => {
            market.creature_1_pool += amount;
            market.creature_1_shares -= shares_bought;
        }
        2 => {
            market.creature_2_pool += amount;
            market.creature_2_shares -= shares_bought;
        }
        3 => {
            market.creature_3_pool += amount;
            market.creature_3_shares -= shares_bought;
        }
        _ => return Err(GameError::InvalidCreatureIndex.into()),
    }

    market.total_pool += amount;

    if position.user == Pubkey::default() {
        position.user = ctx.accounts.user.key();
        position.battle_id = battle.battle_id;
        position.creature_index = creature_index;
        position.amount = shares_bought;
        position.claimed = false;
        position.bump = ctx.bumps.user_position;
    } else {
        position.amount = position
            .amount
            .checked_add(shares_bought)
            .ok_or(GameError::CalculationOverflow)?;
    }

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

#[derive(Accounts)]
#[instruction(creature_index: u8, amount: u64)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        seeds = [b"market", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = market_state.bump,
    )]
    pub market_state: Account<'info, MarketState>,

    #[account(
        seeds = [b"battle", battle_state.battle_id.to_le_bytes().as_ref()],
        bump = battle_state.bump,
    )]
    pub battle_state: Account<'info, BattleState>,
    #[account(
        init_if_needed,
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
