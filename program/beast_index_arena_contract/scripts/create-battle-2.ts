import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BeastIndexArenaContract } from "../target/types/beast_index_arena_contract";

async function main() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.BeastIndexArenaContract as Program<BeastIndexArenaContract>;

    const battleId = new anchor.BN(2);

    console.log("Step 1: Creating Battle #2...");

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
        program.programId
    );

    const tx1 = await program.methods
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
            authority: provider.wallet.publicKey,
        })
        .rpc();

    console.log("Battle created! Tx:", tx1);

    console.log("\nStep 2: Creating Market with large liquidity...");

    const [marketState] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("market"), battleId.toArrayLike(Buffer, "le", 8)],
        program.programId
    );

    const tx2 = await program.methods
        .initializeMarket(battleId, new anchor.BN(10_000_000_000))
        .accounts({
            authority: provider.wallet.publicKey,
        })
        .rpc();

    console.log(" Market created! Tx:", tx2);
    console.log("\nBattle #2 is ready!");
    console.log("Battle PDA:", battleState.toBase58());
    console.log("Market PDA:", marketState.toBase58());
}

main().catch((err) => {
    console.error("Error:", err);
    process.exit(1);
});
