# ⚡ ChainWay - Stellar Chain Speedway
### Road to ZK Validator — ZK Gaming on Stellar Protocol 25

You are a Stellar transaction. Race through the mempool, dodge threats,
collect fees, and earn your place as a ZK Validator.

## 🎮 Play
Open `frontend/index.html` in any browser. No install needed.

## 🧠 What You Learn
Each level teaches a real blockchain concept:

| Level | You Are | You Learn |
|-------|---------|-----------|
| 1 — Slow Lane | Pending TX | Fee tiers & transaction priority |
| 2 — Med Lane | Queued TX | Ed25519 signature verification |
| 3 — Fast Lane | Priority TX | MEV bots & front-running |
| 4 — All Lanes | ZK TX | ZK proof generation (Protocol 25) |
| 5 — Validator | You made it | Endless leaderboard mode |

## ⚠️ Obstacle Effects
- 🌐 **CONGESTION** — slows your speed 50% temporarily
- 🔑 **INVALID SIG** — deducts 20 fees
- ⛔ **DOUBLE SPEND** — TX rejected, game over
- 🤖 **MEV BOT** — steals 30% of your fees

## 💎 Gems
- Gold — standard fee (10pts × lane multiplier)
- Purple ZK — speed boost + proof progress (30pts)
- White P25 — Protocol 25 shield + 100pts (Level 4+)

## 🔗 Contracts (Stellar Testnet)
- Game Hub: `CB4VZAT2U3UC6XFK3N23SKRF2NDCMP3QHJYMCHHFMZO7MRQO6DQ2EMYG`
- Lane Racer: `CA7DLSPSWKZSU425D3W5TXTPS4GQMTX5T2AFUI6TREJZWML5MIKRC54S`

## ⚡ ZK Proof
The RISC Zero guest program re-simulates your entire run deterministically.
Your inputs stay private. Only your score is public. The Soroban contract
records the proof on-chain — provably fair, no server trust needed.

## 🚀 Run ZK Prover Locally
```bash
cd contracts/lane_racer_prover
cargo run --bin host
# Prover runs at localhost:3002
# Frontend auto-detects it — falls back to mock if unavailable
```

## 🛠 Stack
Soroban (Rust) · RISC Zero zkVM · Vanilla JS · Stellar SDK v11 · Protocol 25

licence MIT

