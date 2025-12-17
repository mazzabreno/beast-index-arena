use anchor_lang::prelude::*;

declare_id!("H3EA4meFoepS9ZvufFg83ZJ3E6Ma98hQAbHhx64A9NoB");

pub mod state;
pub mod instructions;
pub mod errors;
pub mod utils;

use instructions::*;

#[program]
pub mod beast_index_arena_contract {
    use super::*;

    pub fn initialize_global(ctx: Context<InitializeGlobal>) -> Result<()> {
        instructions::initialize_global::initialize_global(ctx)
    }

    pub fn update_current_battle(ctx: Context<UpdateCurrentBattle>, battle_id: u64) -> Result<()> {
        instructions::update_current_battle::update_current_battle(ctx, battle_id)
    }

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
        instructions::initialize_battle::initialize_battle(
            ctx,
            battle_id,
            hp,
            atk,
            def,
            spd,
            turn_interval,
            max_duration,
        )
    }

    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        battle_id: u64,
        initial_liquidity: u64,
    ) -> Result<()> {
        instructions::initialize_market::initialize_market(ctx, battle_id, initial_liquidity)
    }

    pub fn execute_turn(ctx: Context<ExecuteTurn>) -> Result<()> {
        instructions::execute_turn::execute_turn(ctx)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        creature_index: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::place_bet::place_bet(ctx, creature_index, amount)
    }

    pub fn sell_shares(ctx: Context<SellShares>, shares_to_sell: u64) -> Result<()> {
        instructions::sell_shares::sell_shares(ctx, shares_to_sell)
    }

    pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
        instructions::claim_winnings::claim_winnings(ctx)
    }
}
