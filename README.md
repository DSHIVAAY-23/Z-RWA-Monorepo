# Z-RWA: Privacy-Preserving Compliance Infrastructure for Indian RWAs

![Solana Devnet](https://img.shields.io/badge/Solana-Devnet-14F195?style=flat-square&logo=solana&logoColor=white) 
![SP1 Groth16](https://img.shields.io/badge/SP1-Groth16-7C3AED?style=flat-square) 
![Anchor Framework](https://img.shields.io/badge/Anchor-Framework-000000?style=flat-square) 
![License MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)

Z-RWA is a technical framework designed to bridge the $3.5T Indian Real-World Asset (RWA) market with decentralized finance on Solana. The protocol addresses a fundamental friction point: the requirement for identity verification (KYC/Aadhaar/PAN) against the privacy risks of publishing Personally Identifiable Information (PII) on a public ledger.

## Core Problem
Tokenizing Indian physical assets currently necessitates the exposure of sensitive government-issued document data on-chain or via centralized intermediaries. This creates a privacy-compliance paradox where regulatory transparency conflicts with individual data security, effectively stalling institutional adoption of on-chain RWAs.

## Protocol Solution
Z-RWA utilizes a decentralized privacy pipeline to verify compliance without data leakage:
- **Local Verification**: Government documents (Aadhaar/PAN/Land Records) are processed entirely on the user's device using local OCR and hashing.
- **SP1 zkVM Integration**: A Succinct SP1 RISC-V program generates a Zero-Knowledge proof (Groth16) confirming document validity against specific criteria.
- **On-Chain Assertion**: The Solana program verifies the proof artifacts (260 bytes) in sub-second time.
- **Native Issuance**: Upon successful verification, the protocol executes a Cross-Program Invocation (CPI) to the Token2022 program to mint a compliance-gated RWA marker (`Z-RWA-COMPLY`).

## Technical Features
- **Client-Side OCR Gate**: Integration of `tesseract.js` for local document scanning and validation.
- **SP1 RISC-V Proving**: Efficient proof generation with ~7.4M constraints and optimized proving times (~1.4s).
- **Embedded Groth16 Verifier**: Hardcoded verification keys (`ZK_RAG_VKEY`) within the Anchor program for immutable validation.
- **Token2022 Compliance**: Native support for Solana's latest token standard, enabling permanent delegates and compliance metadata.

## Technical Stack
- **Blockchain**: Solana (Anchor 0.31.1+)
- **ZK Proof System**: SP1 (Succinct), Groth16
- **Smart Contracts**: Rust (Anchor Framework)
- **Frontend**: Next.js, Tailwind CSS, Solana Web3.js
- **OCR Engine**: Tesseract.js (WASM)

## Installation & Setup

### Prerequisites
- Node.js v20+
- Anchor CLI v0.31.0+
- Solana CLI v1.18+

### Execution Flow

1. **Deploy Solana Program**:
```bash
cd Z-RWA
anchor build
anchor deploy --provider.cluster devnet
```

2. **Launch Dashboard**:
```bash
cd apps/web
npm install
npm run dev
```

The application will be accessible at `http://localhost:3000`.

## Architecture & Data Flow
Detailed technical documentation on the cryptographic pipeline can be found in [flow.md](./flow.md).

## User Journey
A comprehensive guide for institutional users and verifiers is available in [walkthrough.md](./walkthrough.md).

## License
Distributed under the MIT License. Copyright (c) 2026 Z-RWA Protocol.
