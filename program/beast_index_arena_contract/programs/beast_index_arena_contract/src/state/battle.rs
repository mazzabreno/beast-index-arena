use anchor_lang::prelude::*;

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum Ability {
    BasicHit,
    HeavyStrike,
    QuickJab,
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
