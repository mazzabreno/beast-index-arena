# Beast Index Arena (MVP) ⚔️

> **An autonomous on-chain battle simulation and prediction market built on Solana.**

## Overview
**Beast Index Arena** is a fully autonomous auto-battler where four creatures fight automatically until only one remains. Players **never** influence the battle directly; instead, they interact through a **Prediction Market**, trading “Victory Shares” based purely on how the battle unfolds over time.

The market observes—it does not interfere.

---

##  How It Works

### 1. The Autonomous Battle (On-Chain Logic)
The core game loop runs entirely via a "Crank" script that triggers transactions on the Solana blockchain at set intervals.

* **Fairness:** All 4 creatures start with identical stats (ATK, DEF, SPD, HP).
* **Visuals:** Only the skins differ; no balance considerations are needed.
* **RNG & Determinism:**
    * *Target & Move Selection:* Randomized on-chain.
    * *Damage Calculation:* Deterministic (e.g., `Damage = max(1, ATK – DEF)`).
* **Elimination:** When HP hits 0, the creature is instantly removed. The last standing wins.

### 2. The Prediction Market (Parimutuel)
Players speculate on the outcome by purchasing "Victory YES" shares of a specific creature.

* **Speculation:** Players analyze turn logs, momentum, and unexpected RNG events to decide when to enter or exit positions.
* **Settlement:** When the battle ends, backers of the winning creature split the pot.
    * Winning Share Value = **1.0**.
    * Losing Share Value = **0.0**.

---

##  Technical Architecture

This repository is organized as a **Monorepo**:

| Component | Path | Description |
| :--- | :--- | :--- |
| **Program** | `/program` | **Rust/Anchor** Smart Contracts handling state, RNG, and betting logic. |
| **Client** | `/app` | **Next.js** frontend for visualization and wallet interaction. |
| **Crank** | `/scripts` | **Node.js** automation script that drives the battle forward. |

---

##  Getting Started

### Prerequisites
* Node.js (v18+)
* Rust & Cargo
* Solana CLI & Anchor

### 1. Setup Smart Contract (Backend)
```bash
cd program
# Install dependencies
yarn install
# Build the program
anchor build
# Run tests
anchor test
2. Setup Client (Frontend)
Bash

cd app
# Install dependencies
npm install
# Run local development server
npm run dev
3. Run the "Crank" (Automation)
Note: In a production environment, this runs on a server. For testing, run it locally alongside the client.

Bash

# From the root directory
ts-node scripts/crank.ts
MVP Features
4 Unique Skins: Visual flavor with identical stats.

Live Turn Logs: Players observe turn results and HP changes in real-time.

Zero Interference: Trades never affect the battle outcome.

Social Proof: Integrated sharing to post bets and battle updates to X (Twitter).

 License
MIT License
