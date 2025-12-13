import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import { BeastIndexArenaContract } from "./program/beast_index_arena_contract/target/types/beast_index_arena_contract";
import idl from "./program/beast_index_arena_contract/target/idl/beast_index_arena_contract.json";

async function main() {
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");

    const wallet = anchor.Wallet.local();

    const provider = new anchor.AnchorProvider(connection, wallet, {
        commitment: "confirmed",
    });

    const programId = new anchor.web3.PublicKey("28VkZmQABZWqq3gmossB41hYF9846gG2TWMyk4u6jTd4");
    const program = new Program(idl as any, programId, provider) as Program<BeastIndexArenaContract>;

    const battleId = new anchor.BN(2);

    console.log("\nInitializing Battle #2 with proper liquidity...\n");
    console.log("Using wallet:", wallet.publicKey.toBase58());

    const [battlePDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
        programId
    );

    console.log("Battle PDA:", battlePDA.toBase58());

    try {
        const battleTx = await program.methods
            .initializeBattle(
                battleId,
                100,
                50,
                20,
                30,
                new anchor.BN(10),
                new anchor.BN(86400)
            )
            .accounts({
                battleState: battlePDA,
                authority: wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        console.log("Battle initialized! Tx:", battleTx);
    } catch (error: any) {
        if (error.message?.includes("already in use")) {
            console.log("Battle already exists, continuing...");
        } else {
            throw error;
        }
    }

    const [marketPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("market"), battleId.toArrayLike(Buffer, "le", 8)],
        programId
    );

    console.log("💰 Market PDA:", marketPDA.toBase58());

    const initialLiquidity = new anchor.BN(10_000_000_000);

    try {
        const marketTx = await program.methods
            .initializeMarket(battleId, initialLiquidity)
            .accounts({
                marketState: marketPDA,
                authority: wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        console.log("Market initialized with 10 SOL liquidity! Tx:", marketTx);
    } catch (error: any) {
        if (error.message?.includes("already in use")) {
            console.log(" Market already exists");
        } else {
            throw error;
        }
    }

    console.log("\n Battle #2 is ready!");
    console.log("\n Summary:");
    console.log("  Battle ID: 2");
    console.log("  Battle PDA:", battlePDA.toBase58());
    console.log("  Market PDA:", marketPDA.toBase58());
    console.log("  Initial Liquidity: 10 SOL (10,000,000,000 lamports)");
    console.log("\nUsers can now place bets!\n");
}

main().catch(console.error);
