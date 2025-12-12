# Beast Index Arena Bot

Automated bot for managing the Beast Index Arena game on Solana. This bot handles:

- **Battle Creation**: Automatically creates new battles when previous ones end
- **Turn Execution**: Executes combat turns every 10 seconds
- **Game Loop**: Maintains continuous gameplay with configurable delays between battles
- **Market Initialization**: Sets up betting markets with initial liquidity

## Prerequisites

- Node.js 16+ and npm
- Solana wallet with SOL on devnet
- Solana CLI installed (for generating wallets if needed)

## Setup

1. **Install dependencies**:
```bash
npm install
```

2. **Configure your wallet**:

The bot looks for your wallet at `~/.config/solana/id.json` by default.

To use a custom wallet location, set the `WALLET_PATH` environment variable:
```bash
export WALLET_PATH=/path/to/your/wallet.json
```

To generate a new wallet:
```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

3. **Fund your wallet** (devnet):
```bash
solana airdrop 2 $(solana address) --url devnet
```

You'll need at least ~11 SOL per battle (10 SOL for initial liquidity + transaction fees).

## Configuration

Edit `config.ts` to customize bot behavior:

- `TURN_INTERVAL_SECONDS`: Time between combat turns (default: 10s)
- `DELAY_BETWEEN_BATTLES_SECONDS`: Wait time after battle ends (default: 60s)
- `CREATURE_HP/ATK/DEF/SPD`: Initial creature stats
- `INITIAL_LIQUIDITY_SOL`: Starting liquidity for betting markets (default: 10 SOL)
- `STARTING_BATTLE_ID`: Which battle number to start from (default: 102)
- `RPC_ENDPOINT`: Solana RPC endpoint (default: devnet)

## Running the Bot

**Development mode** (with ts-node):
```bash
npm run dev
```

**Production mode** (compiled):
```bash
npm run bot
```

Or manually:
```bash
npm run build
npm start
```

## How It Works

1. **Initialization**: Bot checks if the current battle exists. If not, it creates and initializes it with a betting market.

2. **Battle Execution**:
   - Monitors battle state continuously
   - Executes turns every 10 seconds (respecting turn interval)
   - Displays creature HP and status after each turn
   - Continues until a winner is determined or timeout occurs

3. **Battle Completion**:
   - Announces winner
   - Waits 60 seconds (configurable)
   - Increments battle ID and starts next battle

4. **User Interaction**:
   - Users can bet on creatures while battle is running
   - Winners can claim rewards anytime (even after new battles start)

## Bot Output

The bot provides detailed logging:

```
 Beast Index Arena Bot Started!
 RPC: https://api.devnet.solana.com
 Starting Battle ID: 102
  Turn Interval: 10s
 Delay Between Battles: 60s

 Creating Battle #102...
 Initializing Battle #102...
 Battle PDA: AbC123...
 Battle initialized! Tx: xyz789...

 Initializing Market for Battle #102...
 Market PDA: DeF456...
 Initial Liquidity: 10 SOL
 Market initialized! Tx: abc123...

  Starting Battle #102 monitoring...
  Executing Turn 1...
 Turn executed! Tx: def456...
  HP: [95, 98, 92, 100]
  Alive: [true, true, true, true]

...

 Battle #102 ended!
  Turn: 15
  HP: [0, 45, 0, 0]
  Alive: [false, true, false, false]
  Winner: Creature 1

 Waiting 60s before next battle...
```

## Stopping the Bot

Press `Ctrl+C` to gracefully shut down. The bot will display the last battle processed.

## Troubleshooting

**Insufficient SOL**: Make sure your wallet has enough SOL for creating battles and paying transaction fees.

**RPC Rate Limits**: If using public devnet RPC, you may hit rate limits. Consider using a private RPC endpoint like Helius or QuickNode.

**Account Already In Use**: If a battle already exists, the bot will skip initialization and start executing turns.

**Turn Interval Not Met**: The bot will automatically wait if you try to execute turns too quickly.

## Architecture

- `index.ts`: Main entry point, wallet loading, program initialization
- `battleManager.ts`: Core battle management logic
- `config.ts`: Configuration constants
- `tsconfig.json`: TypeScript compiler settings
- `package.json`: Dependencies and scripts
