# Z-RWA Protocol — ZK-Powered Compliance for Indian RWAs

## ⚡ Overview
Z-RWA is a privacy-preserving compliance system for Institutional DeFi on Solana. It enables investors to prove they meet regulatory requirements (Aadhaar, PAN, age, etc.) using **Zero-Knowledge Proofs**, ensuring that identity data never touches the blockchain.

## 🚀 Key Features
- **ZK-ID Engine**: Real Circom circuits for identity verification without data leakage.
- **On-Chain Verifier**: Groth16 proof verification implemented in Anchor.
- **Compliant Assets**: Real-time minting of **Token2022** assets upon proof verification.
- **Audit-Ready**: Built-in logging and verification trails for institutional reporting.

## 🛠️ Tech Stack
- **ZK**: Circom, SnarkJS (Groth16)
- **Blockchain**: Solana (Anchor), Token2022
- **Frontend**: Next.js, Web3.js
- **RPC**: QuickNode

## 🔗 Final Submission Documentation
For detailed achievements, deployment addresses, and how to verify, please see:
👉 **[SUBMISSION_ZK_RWA.md](SUBMISSION_ZK_RWA.md)**

---

## 🏃 Quick Start

```bash
# 1. Install dependencies
npm install

# 2. Build for production (Crucial for ZK performance)
cd apps/web
npm run build

# 3. Start the optimized production server
npm run start
```

## ⚡ Performance Metrics
- **Proof Generation**: ~1-2 seconds (Production optimized)
- **Proof Size**: 260-byte lightweight Groth16 proof
- **Verification**: Sub-second on-chain verification via Anchor
- **Constraints**: 7.4M SP1-compatible constraints


## 🌐 Vercel Deployment

To deploy this project to Vercel, follow these steps:

1. **Root Directory**: Set the "Root Directory" to `apps/web` in your Vercel project settings.
2. **Environment Variables**:
   - `BACKEND_WALLET_SECRET`: Paste the contents of your `id.json` (the numeric array) to allow the backend to mint tokens.
   - `NEXT_PUBLIC_SOLANA_NETWORK`: `devnet`
3. **Framework Preset**: Ensure "Next.js" is selected.

---

## 📜 Project Structure
- `circuits/`: ZK circuits and setup scripts.
- `apps/web/`: Next.js frontend and ZK-backend.
- `Z-RWA/programs/z_rwa_verifier/`: Anchor verifier program.

---
*Developed for Colosseum Frontier 2026.*
