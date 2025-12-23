# Z-RWA Monorepo

**Status:** 🟢 Devnet Verified | **Build:** SP1-Groth16 v3.0.0 | **Target:** CoinDCX Instagrant

## Overview
This monorepo contains the complete privacy-preserving RWA minting solution, integrating ZK-Proof generation (ZK-RAG) with Solana Program verification (Z-RWA).

### Quick Links
- [Architecture Documentation](./DOCUMENTATION.md)
- [ZK-RAG Engine](./ZK-RAG/)
- [Z-RWA Solana Program](./Z-RWA/)

## Technical Architecture
The system follows a strict "Verify-then-Mint" architecture, ensuring no non-compliant assets can be issued.

```mermaid
sequenceDiagram
    autonumber
    participant U as User (Local Device)
    participant SP1 as SP1 Local Prover (ZK-RAG)
    participant SOL as Solana Program (Z-RWA)
    participant T22 as Token2022 (RWA Mint)

    Note over U, SP1: Privacy Shield: No PII/Documents leave the device
    U->>SP1: Input Document (Aadhaar/Passport) + Compliance Query
    SP1->>SP1: 1. Parse Signature vs Govt Root Key
    SP1->>SP1: 2. Compute Relevance Score (RAG)
    SP1->>SP1: 3. Generate Groth16 SNARK Proof (260 bytes)
    
    U->>SOL: mint_with_zk(Proof, Public Values, Doc Hash)
    Note right of SOL: Sub-second Verifier Execution
    SOL->>SOL: Verify Proof against hardcoded ZK_RAG_VKEY
    SOL->>SOL: Validate Doc Hash binding to Proof
    
    SOL->>T22: CPI: MintTo (Instruction signed by Program PDA)
    Note over T22: Permanent Delegate Extension Active
    T22-->>U: Institutional RWA Token Issued
```

## 📜 Evidence of Execution
We have successfully deployed and verified the integration on Solana Devnet.

- **SP1 VKey Hash:** `0x00cef2f0dedae3382b36f085503bb1a86d98102bca1f64362bdaa1634276df9f`
- **Solana Program ID (z_rwa):** `EaEtWQyXSb5t26KrKpp7XWqrvs1wJAkBM67Qwt1RC5gY`
- **Deployment Signature:** `3Bbkg6ezg5LHQBEK3knBWFhJMzvrW5oX8ZtvUPRh4DfbajEtAPxW6txFPjZQc5j1P2NsPt3HRgvXUjKQ9MvxjL6T`
- **Verification Performance:**
    - On-chain Verification: ~295,000 Compute Units.
    - Local Proving (Groth16): ~23.1s.

## Directory Structure
- **Z-RWA/**: Solana Smart Contract (Anchor) and Client-side tests.
  - Contains the `z-rwa` program logic for verifying proofs and minting tokens.
- **ZK-RAG/**: SP1 Prover Implementation (Rust).
  - Handles the generation of Zero-Knowledge proofs for document validity.

## Development Workflow
- **Branching**:
  - `main`: Production-ready code.
  - `develop`: Active development branch. Feature branches should merge here.
- **Submission**: All changes adhere to strict professional standards suitable for institutional review.

## Documentation
See [DOCUMENTATION.md](./DOCUMENTATION.md) for detailed architecture, security standards, and testing guides.
