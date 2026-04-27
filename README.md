# Z-RWA Compliance Vault — Kamino Track Submission

## What This Is
A KYC-gated Kamino vault using zero-knowledge proofs for 
privacy-preserving compliance. Solves the institutional DeFi problem: 
compliance without sacrificing privacy.

## How It Works
1. User generates ZK proof (SP1 Groth16) proving: age ≥18, Indian national, income threshold met
2. Proof submitted to our Anchor program
3. Program verifies proof, then calls Kamino vault deposit via CPI
4. Vault shares minted with transfer hooks preventing non-compliant transfers

## Tech Stack
- Kamino Finance SDK (vault infrastructure)
- SP1 RISC-V zkVM (Groth16 proving)
- Anchor (Solana program framework)
- QuickNode RPC (with Priority Fee API)
- Next.js + Tailwind CSS

## Live Demo
[Vercel URL - To Be Deployed]

## Security & Audits
See our full threat model and security architecture in [SECURITY.md](SECURITY.md). We are prioritizing a professional audit from Adevar Labs to harden our ZK circuit soundness and Token2022 transfer hook integrations.

## Setup
```bash
# 1. Install dependencies
npm i && cd apps/web && npm i

# 2. Setup your local env
cp apps/web/.env.example apps/web/.env.local

# 3. Build the Anchor program (using nightly rust toolchain)
cd Z-RWA && anchor build

# 4. Deploy Anchor Program
anchor deploy

# 5. Run the frontend development server
npm run dev
```
