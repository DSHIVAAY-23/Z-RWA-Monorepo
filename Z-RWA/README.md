# Z-RWA: Real World Assets on Solana

This repository contains the Solana programs for the Z-RWA protocol.

## ZK-RAG x Z-RWA Handshake

We have integrated a **Zero-Knowledge Retrieval-Augmented Generation (ZK-RAG)** system to enable privacy-preserving compliance for RWA minting.

### 🔄 The Flow
1.  **Private Data**: User holds a private document (e.g., Passport, Utility Bill).
2.  **Local Proving (ZK-RAG)**: The user runs the ZK-RAG prover locally.
    -   Hashes the document.
    -   Proves relevance/validity against a query using SP1 (Succinct Processor 1).
    -   Generates a **Groth16** proof.
3.  **On-Chain Verification (Z-RWA)**:
    -   User calls `verify_and_mint` on the `z-rwa` program.
    -   Program verifies the Groth16 proof using `sp1-solana`.
    -   Program checks the bound **Document Hash**.
4.  **Mint**: If valid, RWA tokens are minted to the user's wallet.

### 🛠️ Components
-   **Gatekeeper**: `programs/z-rwa` - Verifies proofs and mints tokens.
-   **Prover**: `../ZK-RAG` - Generates the proofs.
