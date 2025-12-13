import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import { BattleManager } from "./battleManager";
import { BOT_CONFIG } from "./config";
import fs from "fs";
import path from "path";

const IDL = JSON.parse(
    fs.readFileSync(
        path.join(__dirname, "../program/beast_index_arena_contract/target/idl/beast_index_arena_contract.json"),
        "utf8"
    )
);

async function main() {
    console.log("Starting Beast Index Arena Bot...\n");

    const walletPath = process.env.WALLET_PATH || path.join(
        process.env.HOME || "",
        ".config/solana/id.json"
    );

    console.log(`Loading wallet from: ${walletPath}`);

    let keypair: Keypair;
    try {
        const secretKey = JSON.parse(fs.readFileSync(walletPath, "utf8"));
        keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));
        console.log(` Wallet loaded: ${keypair.publicKey.toBase58()}\n`);
    } catch (error: any) {
        console.error(` Failed to load wallet from ${walletPath}`);
        console.error(`   Error: ${error.message}`);
        console.error(`\nMake sure you have a Solana wallet at ${walletPath}`);
        console.error(`   Or set WALLET_PATH environment variable to your wallet location`);
        process.exit(1);
    }

    console.log(` Connecting to: ${BOT_CONFIG.RPC_ENDPOINT}`);
    const connection = new Connection(BOT_CONFIG.RPC_ENDPOINT, BOT_CONFIG.COMMITMENT);

    try {
        const balance = await connection.getBalance(keypair.publicKey);
        console.log(` Wallet balance: ${balance / 1_000_000_000} SOL\n`);

        if (balance === 0) {
            console.warn(` Warning: Wallet has 0 SOL balance`);
            console.warn(`   You may need to airdrop some SOL for devnet:`);
            console.warn(`   solana airdrop 2 ${keypair.publicKey.toBase58()} --url devnet\n`);
        }
    } catch (error: any) {
        console.error(`Failed to check balance: ${error.message}\n`);
    }

    const wallet = new anchor.Wallet(keypair);
    const provider = new anchor.AnchorProvider(connection, wallet, {
        commitment: BOT_CONFIG.COMMITMENT,
    });

    const programId = new anchor.web3.PublicKey(BOT_CONFIG.PROGRAM_ID);
    const program = new anchor.Program(IDL, provider);

    console.log(` Program ID: ${programId.toBase58()}`);
    console.log(` Program initialized\n`);

    const battleManager = new BattleManager(
        connection,
        wallet,
        program,
        BOT_CONFIG.STARTING_BATTLE_ID
    );

    process.on("SIGINT", () => {
        console.log("\n\n Shutting down bot gracefully...");
        console.log(` Last battle processed: #${battleManager.getCurrentBattleId()}`);
        process.exit(0);
    });

    try {
        await battleManager.start();
    } catch (error: any) {
        console.error("\n Fatal error in bot:");
        console.error(error);
        process.exit(1);
    }
}

main().catch((error) => {
    console.error(" Unhandled error:", error);
    process.exit(1);
});
