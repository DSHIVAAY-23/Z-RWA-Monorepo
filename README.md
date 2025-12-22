# Z-RWA Monorepo

## Overview
This monorepo contains the complete privacy-preserving RWA minting solution, integrating ZK-Proof generation (ZK-RAG) with Solana Program verification (Z-RWA).

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
