# Z-RWA: Technical Architecture & Data Pipeline

This document describes the cryptographic workflow and data pipeline for the Z-RWA privacy-preserving compliance infrastructure.

## Stage 1: Client-Side Validation Gate
The protocol begins on the user's local machine to ensure no Personally Identifiable Information (PII) is transmitted to a server or stored on-chain.
- **WASM OCR**: The user uploads a government-issued document (Aadhaar/PAN). A local WebAssembly-based instance of `tesseract.js` performs character recognition.
- **Validation**: Forgiving regex patterns validate that the document conforms to standard identity formats.
- **Hashing**: A deterministic hash of the document content is generated locally.
- **Data Security**: At no point does the raw document or its text content leave the user's device. Only the document type and hash are preserved for the proving stage.

## Stage 2: ZK Proof Generation (SP1 zkVM)
The validated document metadata is passed to the SP1 RISC-V Prover.
- **zkVM Execution**: A RISC-V program iterates through the compliance logic, verifying that the document hash and type meet the protocol's requirements.
- **Artifact Generation**: The SP1 engine generates a Groth16 proof.
- **Proof Compression**: The resulting proof is 260 bytes, making it highly efficient for on-chain storage and verification.
- **Benchmarks**: The process typically involves ~7,493,634 constraints and is optimized for low-latency proof generation (~1.4s).

## Stage 3: Transaction Construction
The React frontend coordinates the assembly of the Solana transaction.
- **Compute Budget Management**: Since ZK verification is computationally intensive, the frontend injects a `ComputeBudgetProgram.setComputeUnitLimit` instruction requesting 1,400,000 units.
- **ATA Resolution**: The protocol derives the user's Associated Token Account (ATA) for the RWA compliance mint. If the account does not exist, an initialization instruction is added.
- **Instruction Mapping**: The frontend maps the `proof` and `public_values` buffers to the Anchor `verify_and_mint` instruction.

## Stage 4: On-Chain Verification & CPI
The transaction is submitted to the `z_rwa` program on Solana Devnet.
- **Groth16 Verification**: The Solana program natively verifies the SP1 proof using the hardcoded `ZK_RAG_VKEY`.
- **Proof-to-Identity Binding**: The program confirms that the `public_values` match the submitted document hash, binding the ZK proof to the specific compliance request.
- **Mint Execution**: Upon successful verification, the program executes a Cross-Program Invocation (CPI) to the Token2022 program.
- **Asset Issuance**: One unit of the `Z-RWA-COMPLY` marker is minted to the user's wallet, certifying them as a compliant participant in the RWA ecosystem.

## Security Considerations
- **Trustless VKey**: The Verification Key is embedded in the smart contract, ensuring that only proofs from the authorized SP1 circuit can trigger a mint.
- **Replay Protection**: The document hash acting as a unique identifier prevents the same document from being used to mint multiple compliance markers.
