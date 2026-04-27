# Z-RWA × RPC Fast — Submission

## Project
Z-RWA: ZK-Proven RWA Compliance Infrastructure on Solana

Z-RWA aims to bridge the $3.5T Indian Real-World Asset (RWA) market with decentralized finance on Solana. By leveraging SP1 RISC-V to generate Groth16 proofs natively on a user's device, we allow off-chain Aadhaar/PAN validation that is later asserted on-chain. Token2022 transfer hooks ensure that Z-RWA marker movement respects privacy without compromising the requirement for decentralized KYC verifications.

## Why We Use RPC Fast

Our Groth16 proofs stem from a 7.4M-constraint SP1 circuit, which naturally yields large instruction payloads that must be asserted on-chain. Standard public RPC endpoints (such as `devnet.solana.com`) rigorously enforce strict payload size limits and extremely aggressive rate limitations. This consistently causes our proof transactions to either fail validation or drop completely under load. 

RPC Fast mitigates this entirely. It provides reliable and high payload tolerance critical for verifying complex ZK proof representations directly on-chain. Furthermore, the 500 req/s rate limits alongside unlimited bandwidth and low-latency dedicated infrastructure ensures robust uptime for our institutional-gated compliance infrastructure, even under heavy volume.

## Integration

Z-RWA dynamically loads the RPC endpoint, prioritizing `RPC_ENDPOINT` from environment variables before falling back. We instantiate `Connection` instances using this loaded endpoint in our frontend to broadcast transactions. Here is a snippet from our primary integration point in `apps/web/lib/solana.ts`:

```typescript
export const SOLANA_NETWORK = process.env.NEXT_PUBLIC_SOLANA_NETWORK || "devnet";
export const RPC_URL = process.env.RPC_ENDPOINT || process.env.NEXT_PUBLIC_RPC_URL || clusterApiUrl("devnet");
```

## Benchmark Results

| Endpoint | Avg Latency | Min | Max |
|---|---|---|---|
| RPC Fast | [run benchmark] | | |
| Solana Public | [run benchmark] | | |

> Run `yarn benchmark` to reproduce.

## Colosseum Submission
- Main project: [Colosseum project URL — add after submitting]
- GitHub: https://github.com/DSHIVAAY-23/Z-RWA-Monorepo
- Live demo: https://z-rwa-monorepo-fzeb4r6c1-dshivaay23s-projects.vercel.app/
- Branch: main

## RPC Fast Plan Used
Frontier Hackathon Plan (120M CU/month, 500 req/s, Frankfurt endpoint)
