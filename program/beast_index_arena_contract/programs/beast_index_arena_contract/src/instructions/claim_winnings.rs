use anchor_lang::prelude::*;
use crate::state::{BattleState, MarketState, UserPosition};
use crate::errors::GameError;

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
        .ok_or(GameError::DivisionByZero)? as u64;

    **market.to_account_info().try_borrow_mut_lamports()? -= payout;
    **user.to_account_info().try_borrow_mut_lamports()? += payout;

    position.claimed = true;
    Ok(())
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
