# Z-RWA × Zerion CLI — Submission

## Track: Build an Autonomous Onchain Agent using Zerion CLI
## GitHub (Agent): agents/zerion-compliance-agent/
## Zerion Fork: github.com/DSHIVAAY-23/zerion-ai
## Demo video: [Loom URL]

## One Line
The first ZK-gated autonomous trading agent — Zerion handles 
execution, Z-RWA handles compliance verification.

## What the Agent Does
Monitors RWA token transfer requests, verifies each sender's 
ZK compliance proof via the Z-RWA oracle, and autonomously 
executes only compliant transfers through Zerion's swap API.

## Policy Implementation (required by track)
- Chain lock: Solana only
- Spend limit: $10,000 per transaction / $50,000 per day  
- Expiry: December 31, 2026
- ZK compliance gate: sender must have valid Groth16 proof < 30 days old
- Blocked actions: bridging

## Real Onchain Transaction
[Add tx hash after demo run]
Explorer: https://explorer.solana.com/tx/[hash]?cluster=devnet

## Zerion CLI Integration
Built on fork of github.com/zeriontech/zerion-ai
All swaps route through Zerion API as required.

## Why This Is Novel
Traditional trading agents: execute any transaction automatically.
ZK Compliance Agent: execute ONLY transactions where the sender 
cryptographically proves they are compliant — no central KYC, 
no human approval, no identity data on-chain.

## Links
- Main project: https://z-rwa-monorepo.vercel.app
- Oracle API: https://z-rwa-monorepo.vercel.app/api/verify/{wallet}
- GitHub: https://github.com/DSHIVAAY-23/Z-RWA-Monorepo
- Colosseum submission: [URL]
