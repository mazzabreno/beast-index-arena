import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { BOT_CONFIG } from "./config";

export class BattleManager {
    private program: any;
    private provider: anchor.AnchorProvider;
    private currentBattleId: number;

    constructor(
        connection: Connection,
        wallet: anchor.Wallet,
        program: any,
        startingBattleId: number
    ) {
        this.provider = new anchor.AnchorProvider(connection, wallet, {
            commitment: BOT_CONFIG.COMMITMENT,
        });
        this.program = program;
        this.currentBattleId = startingBattleId;
    }

    getBattlePDA(battleId: number): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("battle"),
                new anchor.BN(battleId).toArrayLike(Buffer, "le", 8)
            ],
            this.program.programId
        );
        return pda;
    }


    getMarketPDA(battleId: number): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("market"),
                new anchor.BN(battleId).toArrayLike(Buffer, "le", 8)
            ],
            this.program.programId
        );
        return pda;
    }

    getGlobalPDA(): PublicKey {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from("global")],
            this.program.programId
        );
        return pda;
    }

    async updateCurrentBattle(battleId: number): Promise<void> {
        try {
            const globalPDA = this.getGlobalPDA();

            await this.program.methods
                .updateCurrentBattle(new anchor.BN(battleId))
                .accounts({
                    globalState: globalPDA,
                    authority: this.provider.wallet.publicKey,
                })
                .rpc();

            console.log(` Updated global state to battle #${battleId}`);
        } catch (error: any) {
            console.error(` Failed to update global state:`, error.message);
        }
    }

    async battleExists(battleId: number): Promise<boolean> {
        try {
            const battlePDA = this.getBattlePDA(battleId);
            await this.program.account.battleState.fetch(battlePDA);
            return true;
        } catch {
            return false;
        }
    }


    async getBattleState(battleId: number) {
        const battlePDA = this.getBattlePDA(battleId);
        return await this.program.account.battleState.fetch(battlePDA);
    }


    async initializeBattle(battleId: number): Promise<string> {
        const battlePDA = this.getBattlePDA(battleId);

        console.log(`\nInitializing Battle #${battleId}...`);
        console.log(`Battle PDA: ${battlePDA.toBase58()}`);

        const tx = await this.program.methods
            .initializeBattle(
                new anchor.BN(battleId),
                BOT_CONFIG.CREATURE_HP,
                BOT_CONFIG.CREATURE_ATK,
                BOT_CONFIG.CREATURE_DEF,
                BOT_CONFIG.CREATURE_SPD,
                new anchor.BN(BOT_CONFIG.TURN_INTERVAL),
                new anchor.BN(BOT_CONFIG.MAX_DURATION)
            )
            .accounts({
                battleState: battlePDA,
                authority: this.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        console.log(` Battle initialized! Tx: ${tx}`);
        return tx;
    }

    async initializeMarket(battleId: number): Promise<string> {
        const marketPDA = this.getMarketPDA(battleId);
        const initialLiquidity = new anchor.BN(
            BOT_CONFIG.INITIAL_LIQUIDITY_SOL * 1_000_000_000
        );

        console.log(`\nInitializing Market for Battle #${battleId}...`);
        console.log(`Market PDA: ${marketPDA.toBase58()}`);
        console.log(`Initial Liquidity: ${BOT_CONFIG.INITIAL_LIQUIDITY_SOL} SOL`);

        const tx = await this.program.methods
            .initializeMarket(new anchor.BN(battleId), initialLiquidity)
            .accounts({
                marketState: marketPDA,
                authority: this.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        console.log(`Market initialized! Tx: ${tx}`);
        return tx;
    }


    async executeTurn(battleId: number): Promise<string> {
        const battlePDA = this.getBattlePDA(battleId);

        const tx = await this.program.methods
            .executeTurn()
            .accounts({
                battleState: battlePDA,
                executer: this.provider.wallet.publicKey,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
            })
            .rpc();

        return tx;
    }


    async runBattle(battleId: number): Promise<void> {
        console.log(`\nStarting Battle #${battleId} monitoring...`);

        while (true) {
            try {
                const battle = await this.getBattleState(battleId);

                if (battle.isBattleOver) {
                    console.log(`\nBattle #${battleId} ended!`);
                    console.log(`  Turn: ${battle.currentTurn.toNumber()}`);
                    console.log(`  HP: ${battle.creatureHp}`);
                    console.log(`  Alive: ${battle.isAlive}`);

                    if (battle.winner !== null) {
                        console.log(`  Winner: Creature ${battle.winner}`);
                    } else {
                        console.log(`  Winner: Draw/Timeout`);
                    }
                    break;
                }

                const now = Math.floor(Date.now() / 1000);
                const timeSinceLastTurn = now - battle.lastTurnTime.toNumber();
                const timeToWait = BOT_CONFIG.TURN_INTERVAL - timeSinceLastTurn;

                if (timeToWait > 0) {
                    console.log(`Waiting ${timeToWait}s for next turn...`);
                    await this.sleep(timeToWait * 1000);
                }

                console.log(`\nExecuting Turn ${battle.currentTurn.toNumber() + 1}...`);
                const tx = await this.executeTurn(battleId);
                console.log(`Turn executed! Tx: ${tx.substring(0, 20)}...`);

                const updatedBattle = await this.getBattleState(battleId);
                console.log(`  HP: ${updatedBattle.creatureHp}`);
                console.log(`  Alive: ${updatedBattle.isAlive}`);

            } catch (error: any) {
                if (error.message?.includes("TurnIntervalNotMet")) {
                    console.log(`Turn interval not met, waiting...`);
                    await this.sleep(2000);
                } else if (error.message?.includes("BattleAlreadyOver")) {
                    console.log(`Battle already over`);
                    break;
                } else {
                    console.error(`Error executing turn:`, error.message);
                    await this.sleep(5000);
                }
            }
        }
    }


    async start(): Promise<void> {
        console.log(`\nBeast Index Arena Bot Started!`);
        console.log(` RPC: ${BOT_CONFIG.RPC_ENDPOINT}`);
        console.log(` Starting Battle ID: ${this.currentBattleId}`);
        console.log(` Turn Interval: ${BOT_CONFIG.TURN_INTERVAL}s`);
        console.log(`Delay Between Battles: ${BOT_CONFIG.DELAY_BETWEEN_BATTLES_SECONDS}s\n`);

        while (true) {
            try {
                const exists = await this.battleExists(this.currentBattleId);

                if (!exists) {
                    console.log(`\n Creating Battle #${this.currentBattleId}...`);
                    await this.initializeBattle(this.currentBattleId);
                    await this.initializeMarket(this.currentBattleId);
                    await this.updateCurrentBattle(this.currentBattleId);
                    console.log(`\n Battle #${this.currentBattleId} ready for bets!`);
                } else {
                    console.log(`\n Battle #${this.currentBattleId} already exists`);
                    await this.updateCurrentBattle(this.currentBattleId);
                }

                await this.runBattle(this.currentBattleId);

                console.log(`\n Waiting ${BOT_CONFIG.DELAY_BETWEEN_BATTLES_SECONDS}s before next battle...`);
                await this.sleep(BOT_CONFIG.DELAY_BETWEEN_BATTLES_SECONDS * 1000);

                this.currentBattleId++;

            } catch (error: any) {
                console.error(`\n Error in bot loop:`, error.message);
                console.log(` Retrying in 10 seconds...`);
                await this.sleep(10000);
            }
        }
    }

    getCurrentBattleId(): number {
        return this.currentBattleId;
    }

    private sleep(ms: number): Promise<void> {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}
