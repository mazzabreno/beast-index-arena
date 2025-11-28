import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BeastIndexArenaContract } from "../target/types/beast_index_arena_contract";
import { SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";

describe("beast_index_arena_contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BeastIndexArenaContract as Program<BeastIndexArenaContract>;

  it("Phase 2: Initialize battle with 4 creatures", async () => {
    const battleId = new anchor.BN(1);

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    console.log("Battle Account:", battleState.toBase58());

    const tx = await program.methods
      .initializeBattle(
        battleId,
        100,
        50,
        20
      )
      .accounts({
        battleState: battleState,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction:", tx);
    const battle = await program.account.battleState.fetch(battleState);

    console.log("Battle State:");
    console.log("Battle ID:", battle.battleId.toNumber());
    console.log("Creatures HP:", battle.creatureHp);
    console.log("Is Alive:", battle.isAlive);
    console.log("Battle Over:", battle.isBattleOver);

    if (battle.battleId.toNumber() !== 1) throw new Error("Battle ID wrong");
    if (battle.creatureHp[0] !== 100) throw new Error("Creature 0 HP wrong");
    if (battle.creatureHp[3] !== 100) throw new Error("Creature 3 HP wrong");
    if (!battle.isAlive[0]) throw new Error("Creature 0 should be alive");
    if (battle.isBattleOver) throw new Error("Battle should not be over");
    console.log("Phase 2 initialization works!");
  });

  it("Phase 2: Execute turns with RNG", async () => {
    const battleId = new anchor.BN(1);
    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    console.log("Testing RNG-based combat");
    await program.methods
      .executeTurn()
      .accounts({
        battleState: battleState,
        executer: provider.wallet.publicKey,
        clock: SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();
    const battle = await program.account.battleState.fetch(battleState);

    console.log("After Turn 1:");
    console.log("Creatures HP:", battle.creatureHp);
    console.log("Is Alive:", battle.isAlive);
    console.log("Turn:", battle.currentTurn.toNumber());

    let totalHp = 0;
    for (let i = 0; i < 4; i++) {
      totalHp += battle.creatureHp[i];
    }

    if (totalHp >= 400) throw new Error("No damage was dealt!");
    if (battle.currentTurn.toNumber() !== 1) throw new Error("Turn should be 1");

    console.log("RNG combat works!");
  });

  it("Phase 2: Battle until winner", async () => {
    const battleId = new anchor.BN(1);

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    console.log("Running battle until end...");

    let turnCount = 0;
    const maxTurns = 30;

    while (turnCount < maxTurns) {
      let battle = await program.account.battleState.fetch(battleState);

      if (battle.isBattleOver) {
        console.log(`Battle ended after ${battle.currentTurn.toNumber()} turns!`);
        console.log("Final HP:", battle.creatureHp);
        console.log("Is Alive:", battle.isAlive);

        if (battle.winner !== null) {
          console.log(`Winner: Creature ${battle.winner}`);
        } else {
          console.log("Result: Draw");
        }

        break;
      }

      console.log(
        `Turn ${turnCount + 1}: HP=[${battle.creatureHp}], Alive=[${battle.isAlive}]`
      );

      await program.methods
        .executeTurn()
        .accounts({
          battleState: battleState,
          executer: provider.wallet.publicKey,
          clock: SYSVAR_CLOCK_PUBKEY,
        })
        .rpc();
      turnCount++;
      await new Promise((resolve) => setTimeout(resolve, 100));
    }

    const finalBattle = await program.account.battleState.fetch(battleState);

    if (!finalBattle.isBattleOver) {
      throw new Error("Battle should have ended");
    }
    console.log("Battle completed successfully!");
  });
});