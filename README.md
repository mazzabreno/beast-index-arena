# Beast Index Arena - Smart Contract

An autonomous on-chain battle simulation and prediction market built on Solana.

## Overview

Beast Index Arena is a fully autonomous auto-battler where four creatures fight automatically until only one remains. Players interact through a prediction market, trading shares based on how the battle unfolds over time. The market observes but never influences the battle outcome.

## System Flow

The complete system consists of three components working together:

1. **Smart Contract** : Handles all battle logic, betting, and payouts on Solana
2. **Bot Server**: Automatically creates new battles and executes turns every few seconds
3. **Frontend**: Displays live battles and allows users to place bets

### How a Battle Works

1. The bot initializes a new battle on-chain with 4 creatures
2. The bot executes turns automatically (every 3-5 seconds)
3. Each turn, the smart contract randomly selects an attacker and target, calculates damage
4. Players watch the battle unfold and place bets on creatures they think will win
5. When only one creature survives, the battle ends
6. Winners can claim their share of the prize pool
7. The bot starts a new battle and the cycle repeats

All game logic happens on the blockchain - the bot simply triggers the transactions. This ensures fairness and transparency.

### The Autonomous Battle

- **Fairness**: All 4 creatures start with identical stats (ATK, DEF, SPD, HP)
- **Visuals**: Only the skins differ - Yeti, Mapinguari, Zmey, and Naga
- **RNG & Determinism**:
  - Target and move selection are randomized on-chain
  - Damage calculation is deterministic (e.g., Damage = max(1, ATK - DEF))
- **Elimination**: When HP hits 0, the creature is removed. The last one standing wins.

### The Prediction Market

Players speculate on the outcome by purchasing shares of a specific creature.

- **Speculation**: Players analyze battle state, momentum, and RNG events to decide when to enter or exit positions
- **Settlement**: When the battle ends, backers of the winning creature split the pot
  - Winning share value: 1.0
  - Losing share value: 0.0

## Technical Architecture

This is the smart contract component of the Beast Index Arena system.

**Tech Stack**:
- Solana blockchain (devnet)
- Anchor framework (Rust)
- On-chain RNG for target selection
- Deterministic combat calculation

**Core Features**:
- Battle state management (HP, stats, turn tracking)
- Parimutuel betting market
- Position tracking per user per creature
- Winner settlement and payout distribution
- Battle initialization and turn execution

## Program Structure

```
program/
└── beast_index_arena_contract/
    ├── src/
    │   ├── lib.rs           # Main program logic
    │   ├── state/           # Account structures
    │   └── instructions/    # Transaction handlers
    └── Cargo.toml
```

## Key Instructions

- `initialize_battle`: Creates a new battle with 4 creatures
- `execute_turn`: Processes one combat turn (target selection, damage, elimination)
- `place_bet`: Allows users to buy shares of a creature
- `sell_shares`: Allows users to sell shares before battle ends
- `claim_winnings`: Distributes payouts to winners after battle ends
- `end_battle`: Marks battle as complete and determines winner

## Battle Mechanics

1. Each creature has ATK, DEF, SPD, and HP stats
2. Every turn, a random alive creature attacks another random alive target
3. Damage is calculated as: max(1, attacker.ATK - target.DEF)
4. When a creature's HP reaches 0, it is eliminated
5. Battle continues until only one creature remains
6. The last surviving creature is declared the winner

## Market Mechanics

- Players buy shares using SOL (minimum 0.01 SOL)
- Share prices are dynamic based on total pool and creature pool
- All bets go into a total pool
- Winners receive proportional payout: (user_shares / winning_pool) * total_pool
- Losers receive nothing (shares become worthless)

## Development

The contract is written in Rust using the Anchor framework and deployed on Solana devnet.

## License

MIT
