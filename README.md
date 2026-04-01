# Z-RWA: Trustless Indian RWA Compliance on Solana

![Solana Devnet](https://img.shields.io/badge/Solana-Devnet-14F195?style=flat-square&logo=solana&logoColor=white) 
![SP1 Groth16](https://img.shields.io/badge/SP1-Groth16-7C3AED?style=flat-square) 
![Anchor Framework](https://img.shields.io/badge/Anchor-Framework-000000?style=flat-square) 
![License MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)

> **Indian farmers and landowners hold $3.5T in assets that cannot be tokenized without exposing highly sensitive Aadhaar/PAN details on-chain. Z-RWA solves this missing link with Zero-Knowledge proofs.**

Z-RWA provides a privacy-preserving compliance portal for Indian Real World Assets (RWA). It leverages SP1 to locally generate a Groth16 proof validating a user's government documents, which is then verified sub-second on Solana via an Anchor program to mint a compliant Token2022 asset.

---

## Technical Benchmarks

| Metric | Value |
|--------|-------|
| Proof System | SP1 Groth16 v3.0.0 |
| Proof Size | 260 bytes |
| Constraints | 7,493,634 |
| Proving Time | ~23 seconds |
| On-chain Verification | Sub-second |
| Token Standard | Token2022 |

---

## 🏗 Architecture

```text
[User Device]
   │
   ├─► Document (NEVER leaves device)
   │
[SP1 Local Prover]
   │
   ├─► Groth16 Proof (260 bytes)
   │
[Solana z-rwa Program]
   │
   ├─► VKey Verification (sub-second)
   │
[Token2022 Mint]
   │
   ▼
[User Wallet] ← RWA Compliance Token Issued
```

---

## 🚀 Quick Start

Ensure you have Node.js and npm installed.

```bash
# Navigate to the frontend app
cd apps/web

# Install dependencies
npm install

# Start the development server (Defaults to MOCK_MODE=true for faster UI iteration)
npm run dev
```

Visit `http://localhost:3000` to interact with the portal.

---

## 🇮🇳 The Problem & Solution

### The Bottleneck for Indian RWAs
Asset owners in India face strict KYC requirements to participate in global DeFi protocols. However, uploading documents like **Aadhaar cards** or **PAN cards** to IPFS or exposing them on public block explorers is a severe violation of privacy, preventing the onboarding of legitimate RWA liquidity.

### The Z-RWA Solution
- **Zero-data exposure**: The compliance document is checked entirely locally within a secure executing environment using SP1.
- **On-chain verifiability**: A Groth16 proof accompanied by a public document hash is posted to Solana.
- **Composability**: Upon successful verification, a Token2022 is mapped directly to the user's wallet containing permanent limits/delegates ensuring they are certified clean.

---

## Solana Program Deployment

*(Program code is located in `/programs/z-rwa/`)*

```bash
anchor build
anchor test
anchor deploy --provider.cluster devnet
```

After deploying, copy the new Program ID to `apps/web/.env` as `NEXT_PUBLIC_Z_RWA_PROGRAM_ID`.

---

## Grant Context: Superteam India

This project is built to showcase a highly secure, privacy-preserving path to tokenize Indian assets on Solana for Superteam. The focus is specifically addressing India's complex regulatory intersections by combining local SP1 proving and blazing-fast Solana verifications.

---

## 🗺 Roadmap

- [x] **Phase 1**: Aadhaar/PAN compliance portal with SP1 Groth16 proofs
- [ ] **Phase 2**: Integration with Land records (Digital Bhulekh)
- [ ] **Phase 3**: Credit score validity proofs (CIBIL integration)
- [ ] **Phase 4**: Cross-chain settlement (UPE integration)

## License

MIT Copyright (c) 2026 Z-RWA Protocol
