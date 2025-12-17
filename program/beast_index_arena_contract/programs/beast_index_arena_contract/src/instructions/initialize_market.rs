use anchor_lang::prelude::*;
use crate::state::MarketState;

pub fn initialize_market(
    ctx: Context<InitializeMarket>,
    battle_id: u64,
    initial_liquidity: u64,
) -> Result<()> {
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
