# Z-RWA × Eitherway × QuickNode — Submission

## Track: QuickNode (Primary)
## Live dApp: https://z-rwa-monorepo.vercel.app
## Demo video: [Loom URL — add after recording]

## One Line
ZK compliance infrastructure for institutional RWA — powered by 
QuickNode for real-time proof submission, fee estimation, and 
wallet analytics.

## QuickNode Integration — What We Built

### 1. Primary RPC Endpoint
All Z-RWA transactions — ZK proof submission, Token2022 minting, 
compliance verification — go through QuickNode. Reason: Groth16 
proof payloads (~260 bytes) exceed standard public RPC limits under 
load. QuickNode provides reliable throughput for production ZK workloads.

Endpoint: https://frequent-alpha-pool.solana-devnet.quiknode.pro/5f06a41cf6e077af5ca7ac464fbf1caed5c84d42/

### 2. Priority Fee API (qn_estimatePriorityFees)
Every ZK proof submission uses QuickNode's fee estimation API to 
set optimal priority fees. Large proof transactions need priority 
to avoid being dropped during congestion.

API call:
```json
{
  "method": "qn_estimatePriorityFees",
  "params": { "last": 100 }
}
```
Result: optimal microlamport fee added to every proof transaction.

### 3. Real-time Network Status Bar
Live slot number, TPS, and priority fee displayed in the UI — 
fetched from QuickNode every 10 seconds. Judges and users see 
real network data, not hardcoded values.

### 4. Wallet Transaction History
QuickNode's getSignaturesForAddress powers the transaction history 
panel in the compliance checker — shows a wallet's recent on-chain 
activity alongside their compliance status.

## Why QuickNode Is Core (Not Cosmetic)

| Feature | Without QuickNode | With QuickNode |
|---|---|---|
| ZK proof submission | Drops under load | Reliable every time |
| Priority fees | Guessed manually | Optimal, real-time |
| Network status | Hardcoded/stale | Live, 10s refresh |
| Tx history | Not available | Real, queryable |

## Architecture
User → Compliance Checker → QuickNode RPC → Solana Devnet
                          ↑
              Priority fee API (qn_estimatePriorityFees)
              getSignaturesForAddress (tx history)
              getSlot + getRecentPerformanceSamples (status bar)

## Technical Stack
- QuickNode Solana Devnet endpoint (primary RPC)
- @solana/web3.js via QuickNode connection
- Next.js 14 app router
- SP1 Groth16 ZK proof generation
- Token2022 compliance-gated transfers
- Solflare wallet adapter

## Colosseum Submission
[Add link after submitting to Colosseum]

## GitHub
https://github.com/DSHIVAAY-23/Z-RWA-Monorepo/tree/feature/quicknode-rpc
