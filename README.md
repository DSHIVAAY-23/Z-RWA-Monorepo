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
- **SP1 RISC-V Proving**: Efficient proof generation with ~7.4M constraints and hardware-verified proving times (~23.4s).
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

## Roadmap & Future Architecture

The Z-RWA V1 MVP establishes the core cryptographic pipeline for off-chain document verification and Token2022 minting. Upcoming phases will harden the protocol for institutional mainnet deployment.

### Phase 2: Protocol Hardening & Institutional Gating
* **Token2022 Transfer Hooks**: Upgrading the `z_rwa` marker to implement `spl_transfer_hook_interface`. This will enforce that RWA tokens can only be transferred to destination wallets that hold a valid, ZK-verified Compliance Record PDA, effectively creating a fully gated compliance ecosystem.
* **zk-Regex Integration**: Shifting the OCR validation from the client-side frontend directly into the SP1 RISC-V circuit. This guarantees cryptographically that the hidden document contains valid government ID formats before the proof is generated.

### Phase 3: Developer Abstraction & Mainnet
* **Z-RWA Developer SDK**: Releasing an NPM package that abstracts the SP1 verification logic, allowing any Solana dApp to integrate privacy-preserving KYC with minimal overhead.
* **Circuit Audits**: Comprehensive security audits of the 7.4M constraint Groth16 circuit prior to Solana Mainnet deployment.
* **Nullifier Registry**: Implementing a strict 1-to-1 nullifier PDA registry to prevent proof-replay attacks and ensure sybil resistance.

## License
Distributed under the MIT License. Copyright (c) 2026 Z-RWA Protocol.
