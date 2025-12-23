# Z-RWA — Litepaper

## Abstract

Z-RWA provides a privacy-first pipeline for institutional Real-World Asset (RWA) tokenization by combining a local Retrieval-Augmented Generation (RAG) prover with Groth16 zero-knowledge proofs and an on-chain verifier implemented as an Anchor program on Solana. The design enables issuers to prove compliance or eligibility without exposing underlying PII to on-chain or third-party services.

## Technical Overview

- Prover (ZK-RAG): Runs client-side (Rust) and ingests documents and public roots (e.g., government PKI). It uses RAG to extract relevant assertions and compiles them into a succinct Groth16 proof.
- On-chain Verifier (Z-RWA): Anchor-based program with a hardcoded/managed verification key that performs proof verification and enforces minting logic via Token2022 extensions.
- Token Standard: Token2022 is used for minting, enabling extensions such as permanent delegate and freeze controls for compliance-driven operations.

## Security Model

- Threats: proof forgery, replay attacks, compromised prover devices, regulatory subpoenas.  
- Mitigations: Groth16 soundness, order-nonces & request replay protection, prover attestation roadmap, legal/compliance section for custody.

## Architecture Diagram

See repository DOCUMENTATION.md for sequence diagrams and CPI flows. Key properties: Verify-then-Mint, PDA-authority controlled mint, atomic burn/transfer for DvP flows.

## Deployment & Reproducibility

- Devnet artifacts and program IDs included in repository.  
- `Anchor.toml` configured for devnet; tests and `ts-mocha` scripts provided in `Z-RWA` for reproducible demos.

## Roadmap

- Phase 1: CI, security hardening, reproducible builds.  
- Phase 2: Third-party smart contract audit and prover validation.  
- Phase 3: Pilot with an institutional issuer and governance tooling.

## Legal & Compliance

High-level guidance only: token issuance should align with issuer jurisdiction. Provide legal attachments during pilot; a compliance playbook will be prepared post-funding.

## Contact

Ops: ops@zrwa.example — replace with real contact before submission.
