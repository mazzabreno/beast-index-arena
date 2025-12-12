import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import fs from "fs";
import { BOT_CONFIG } from "./config";
import IDL from "./program/beast_index_arena_contract/target/idl/beast_index_arena_contract.json";

const connection = new Connection(BOT_CONFIG.RPC_ENDPOINT, BOT_CONFIG.COMMITMENT);
const walletPath = `${process.env.HOME}/.config/solana/id.json`;
const keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, "utf-8")))
);

const wallet = new anchor.Wallet(keypair);
const provider = new anchor.AnchorProvider(connection, wallet, { commitment: BOT_CONFIG.COMMITMENT });
const program = new Program(IDL as any, provider);

async function initializeGlobal() {
    const [globalPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("global")],
        program.programId
    );

    console.log("🌍 Initializing global state...");
    console.log("   Global PDA:", globalPDA.toString());
    console.log("   Program ID:", program.programId.toString());

    try {
        const tx = await program.methods
            .initializeGlobal()
            .accounts({
                globalState: globalPDA,
                authority: keypair.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        console.log("✅ Global state initialized!");
        console.log("   Transaction:", tx);
    } catch (error: any) {
        if (error.message.includes("already in use")) {
            console.log("⚠️  Global state already initialized");
        } else {
            console.error("❌ Error initializing global state:", error);
            throw error;
        }
    }
}

initializeGlobal().catch(console.error);
