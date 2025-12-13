import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BeastIndexArenaContract } from "../target/types/beast_index_arena_contract";

async function main() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.BeastIndexArenaContract as Program<BeastIndexArenaContract>;

    const battleId = new anchor.BN(102);

    console.log("Creating Battle #102...");
    await program.methods
        .initializeBattle(battleId, 100, 50, 20, 30, new anchor.BN(10), new anchor.BN(86400))
        .accounts({ authority: provider.wallet.publicKey })
        .rpc();
    console.log("Battle created!");

    console.log("Creating Market with 100 billion liquidity...");
    await program.methods
        .initializeMarket(battleId, new anchor.BN(100_000_000_000))
        .accounts({ authority: provider.wallet.publicKey })
        .rpc();
    console.log("Market created!");
    console.log("Battle #102 ready!");
}

main().catch(console.error);