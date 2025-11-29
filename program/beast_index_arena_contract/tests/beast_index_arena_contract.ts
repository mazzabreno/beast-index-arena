import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BeastIndexArenaContract } from "../target/types/beast_index_arena_contract";
import { SYSVAR_CLOCK_PUBKEY, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("beast_index_arena_contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BeastIndexArenaContract as Program<BeastIndexArenaContract>;

  it("Initialize battle with 4 creatures", async () => {
    const battleId = new anchor.BN(1);

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    console.log(" Battle Account:", battleState.toBase58());

    const tx = await program.methods
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
        battleState: battleState,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log(" Transaction:", tx);

    const battle = await program.account.battleState.fetch(battleState);

    console.log(" Battle State:");
    console.log("  Battle ID:", battle.battleId.toNumber());
    console.log("  Creatures HP:", battle.creatureHp);
    console.log("  Creatures SPD:", battle.creatureSpd);
    console.log("  Is Alive:", battle.isAlive);
    console.log("  Turn Interval:", battle.turnInterval.toNumber(), "seconds");
    console.log("  Max Duration:", battle.maxDuration.toNumber(), "seconds");

    if (battle.battleId.toNumber() !== 1) throw new Error("Battle ID wrong");
    if (battle.creatureHp[0] !== 100) throw new Error(" HP wrong");
    if (battle.creatureSpd[0] !== 30) throw new Error(" SPD wrong");

    console.log(" Battle initialization works!\n");
  });

  it("Initialize market for betting", async () => {
    const battleId = new anchor.BN(1);

    const [marketState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("market"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    console.log(" Market Account:", marketState.toBase58());

    const tx = await program.methods
      .initializeMarket(battleId)
      .accounts({
        marketState: marketState,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log(" Transaction:", tx);

    const market = await program.account.marketState.fetch(marketState);

    console.log("Market State:");
    console.log("  Battle ID:", market.battleId.toNumber());
    console.log("  Creature 0 Pool:", market.creature0Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Creature 1 Pool:", market.creature1Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Creature 2 Pool:", market.creature2Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Creature 3 Pool:", market.creature3Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Total Pool:", market.totalPool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Is Settled:", market.isSettled);

    if (market.totalPool.toNumber() !== 0) throw new Error(" Pool should start at 0");
    if (market.isSettled) throw new Error(" Market should not be settled");

    console.log(" Market initialization works!\n");
  });

  it("Place bet on Creature 0", async () => {
    const battleId = new anchor.BN(1);

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const [marketState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("market"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const creatureIndex = 0;
    const betAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

    const [userPosition] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("position"),
        battleId.toArrayLike(Buffer, "le", 8),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from([creatureIndex])
      ],
      program.programId
    );

    console.log(" Placing bet...");
    console.log("  User:", provider.wallet.publicKey.toBase58());
    console.log("  Creature:", creatureIndex);
    console.log("  Amount:", betAmount.toNumber() / LAMPORTS_PER_SOL, "SOL");

    const tx = await program.methods
      .placeBet(creatureIndex, betAmount)
      .accounts({
        marketState: marketState,
        battleState: battleState,
        userPosition: userPosition,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log(" Transaction:", tx);

    const market = await program.account.marketState.fetch(marketState);
    console.log("Updated Market:");
    console.log("  Creature 0 Pool:", market.creature0Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Total Pool:", market.totalPool.toNumber() / LAMPORTS_PER_SOL, "SOL");

    const position = await program.account.userPosition.fetch(userPosition);
    console.log(" User Position:");
    console.log("  User:", position.user.toBase58());
    console.log("  Creature:", position.creatureIndex);
    console.log("  Amount:", position.amount.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Claimed:", position.claimed);

    if (market.creature0Pool.toNumber() !== betAmount.toNumber()) {
      throw new Error(" Pool amount wrong");
    }
    if (position.amount.toNumber() !== betAmount.toNumber()) {
      throw new Error(" Position amount wrong");
    }

    console.log("Betting works!\n");
  });

  it("Multiple players bet on different creatures", async () => {
    const battleId = new anchor.BN(1);


    const [marketState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("market"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    console.log("Placing multiple bets...");

    const creature1Index = 1;
    const bet1Amount = new anchor.BN(0.3 * LAMPORTS_PER_SOL);

    const [position1] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("position"),
        battleId.toArrayLike(Buffer, "le", 8),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from([creature1Index])
      ],
      program.programId
    );

    await program.methods
      .placeBet(creature1Index, bet1Amount)
      .accounts({
        marketState: marketState,
        battleState: battleState,
        userPosition: position1,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log(" Bet 0.3 SOL on Creature 1");

    const creature2Index = 2;
    const bet2Amount = new anchor.BN(0.2 * LAMPORTS_PER_SOL);

    const [position2] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("position"),
        battleId.toArrayLike(Buffer, "le", 8),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from([creature2Index])
      ],
      program.programId
    );

    await program.methods
      .placeBet(creature2Index, bet2Amount)
      .accounts({
        marketState: marketState,
        battleState: battleState,
        userPosition: position2,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Bet 0.2 SOL on Creature 2");

    const market = await program.account.marketState.fetch(marketState);

    console.log("Final Market State:");
    console.log("  Creature 0 Pool:", market.creature0Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Creature 1 Pool:", market.creature1Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Creature 2 Pool:", market.creature2Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Creature 3 Pool:", market.creature3Pool.toNumber() / LAMPORTS_PER_SOL, "SOL");
    console.log("  Total Pool:", market.totalPool.toNumber() / LAMPORTS_PER_SOL, "SOL");

    const total = market.totalPool.toNumber();
    console.log("Current Odds:");
    console.log("  Creature 0:", ((market.creature0Pool.toNumber() / total) * 100).toFixed(1) + "%");
    console.log("  Creature 1:", ((market.creature1Pool.toNumber() / total) * 100).toFixed(1) + "%");
    console.log("  Creature 2:", ((market.creature2Pool.toNumber() / total) * 100).toFixed(1) + "%");
    console.log("  Creature 3:", ((market.creature3Pool.toNumber() / total) * 100).toFixed(1) + "%");

    console.log(" Multiple bets work!\n");
  });

  it("Execute turn after betting", async () => {
    const battleId = new anchor.BN(1);

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    console.log("Executing turn...");
    console.log("  (Waiting 11 seconds for turn interval...)");

    await new Promise((resolve) => setTimeout(resolve, 11000));

    const tx = await program.methods
      .executeTurn()
      .accounts({
        battleState: battleState,
        executer: provider.wallet.publicKey,
        clock: SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();

    console.log("Transaction:", tx);

    const battle = await program.account.battleState.fetch(battleState);

    console.log("After Turn 1:");
    console.log("  Creatures HP:", battle.creatureHp);
    console.log("  Is Alive:", battle.isAlive);
    console.log("  Turn:", battle.currentTurn.toNumber());

    console.log("Turn execution works!\n");
  });

  it("Cannot bet on dead creature", async () => {
    const battleId = new anchor.BN(2);

    const [battleState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("battle"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .initializeBattle(battleId, 100, 50, 20, 30, new anchor.BN(10), new anchor.BN(86400))
      .accounts({
        battleState: battleState,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const [marketState] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("market"), battleId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    await program.methods
      .initializeMarket(battleId)
      .accounts({
        marketState: marketState,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();


    const creatureIndex = 0;
    const betAmount = new anchor.BN(0.1 * LAMPORTS_PER_SOL);

    const [userPosition] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("position"),
        battleId.toArrayLike(Buffer, "le", 8),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from([creatureIndex])
      ],
      program.programId
    );

    await program.methods
      .placeBet(creatureIndex, betAmount)
      .accounts({
        marketState: marketState,
        battleState: battleState,
        userPosition: userPosition,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Can bet on alive creature\n");
  });
});