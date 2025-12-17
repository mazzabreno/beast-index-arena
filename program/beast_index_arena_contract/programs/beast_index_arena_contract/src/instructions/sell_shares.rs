use anchor_lang::prelude::*;
use crate::state::{BattleState, MarketState, UserPosition};
use crate::errors::GameError;
use crate::utils::calculate_sell_price;

pub fn sell_shares(ctx: Context<SellShares>, shares_to_sell: u64) -> Result<()> {
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

    let sol_returned = calculate_sell_price(current_shares, shares_to_sell, market.k_constant)?;

    **market.to_account_info().try_borrow_mut_lamports()? -= sol_returned;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += sol_returned;

    match creature_index {
        0 => {
            market.creature_0_pool -= sol_returned;
            market.creature_0_shares += shares_to_sell;
        }
        1 => {
            market.creature_1_pool -= sol_returned;
            market.creature_1_shares += shares_to_sell;
        }
        2 => {
            market.creature_2_pool -= sol_returned;
            market.creature_2_shares += shares_to_sell;
        }
        3 => {
            market.creature_3_pool -= sol_returned;
            market.creature_3_shares += shares_to_sell;
        }
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
