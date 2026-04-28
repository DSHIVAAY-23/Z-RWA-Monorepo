# Z-RWA Compliance Agent × Zerion CLI

An autonomous onchain agent that enforces ZK compliance before 
executing RWA token transfers. Built on top of Zerion CLI.

## What It Does

The agent monitors a queue of RWA transfer requests. For each request,
it calls the Z-RWA compliance oracle to verify the sender has a valid
Groth16 ZK proof. Only compliant wallets get their transfers executed
via the Zerion API. Non-compliant wallets are flagged and rejected —
automatically, without human approval.

## Policy Design

```typescript
{
  allowedChains: ["solana"],          // Chain-locked
  maxSpendUsdPerTx: 10000,            // Spend limit
  requireZKCompliance: true,          // ZK proof required
  minProofFreshnessSeconds: 2592000,  // 30 days max
  blockedActions: ["bridge"],         // No bridging
  expiresAt: new Date("2026-12-31"),  // Policy TTL
}
```

## Quick Start

```bash
cd agents/zerion-compliance-agent
cp .env.example .env
# Add your ZERION_API_KEY from dashboard.zerion.io
npm install
npm run demo
```

## How It Works

1. Agent reads pending transfer queue
2. For each request → calls Z-RWA oracle: GET /api/verify/{wallet}
3. If compliant + proof fresh → executes via Zerion API
4. If non-compliant → flags and skips
5. Logs all decisions with proof hashes

## Fork of Zerion CLI

Built on: github.com/zeriontech/zerion-ai
Our fork: github.com/DSHIVAAY-23/zerion-ai

## Live Demo

Z-RWA Oracle: https://z-rwa-monorepo.vercel.app/api/verify/{wallet}
Main project: https://z-rwa-monorepo.vercel.app
