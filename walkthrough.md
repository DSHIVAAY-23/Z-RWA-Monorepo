# Z-RWA: Institutional Compliance Walkthrough

This guide provides a walkthrough of the Z-RWA document-to-token compliance journey from the perspective of an institutional user or verifier.

## Step 1: Document Upload & Local Validation
The protocol interface features a cyber-industrial dashboard designed for high-security compliance workflows.
- **Selection**: Users select the identity document type (e.g., Aadhaar, PAN Card, Land Record).
- **Upload**: The document is dragged into the local processing zone.
- **Client-Side Gate**: The dashboard's built-in OCR validates the document's structure locally. Upon success, a "Valid ID Detected" status is displayed along with a unique document hash. **No PII is archived or transmitted.**

## Step 2: ZK Proving Engine
Once validated, the user triggers the ZK Proving Engine.
- **Technical Initialization**: The Z-Terminal UI streams real-time logs as the SnarkJS Groth16 prover initializes.
- **Proof Generation**: The prover executes the Circom compliance circuit. The dashboard displays key technical metrics:
  - **Constraint Count**: ~7,493,634
  - **Proof Format**: Groth16 (SnarkJS)
  - **Proof Size**: 260 bytes
  - **Proving Time**: ~2-3 seconds
- **Binding**: The terminal confirms the verification key is loaded, ensuring the proof is architecturally locked to the protocol's verification parameters.

## Step 3: Wallet Integration & On-Chain Minting
The final stage involves the bridging of the ZK proof to the Solana blockchain.
- **Wallet Connection**: The user connects their Solana wallet (e.g., Phantom, Solflare).
- **Transaction Submission**: A "Verify Wallet Connection" message is signed, followed by the "Submit Proof & Mint Token" action.
- **On-Chain Assertion**: The transaction executes the `z_rwa` program, which verifies the Groth16 proof sub-second.
- **Issuance Success**: Upon confirmation, the "Token2022 Minted Successfully" card appears.
- **Transparency**: A direct link to the **Solana Explorer** is provided, where users can verify the on-chain minting of the `Z-RWA-COMPLY` asset, confirming the user's compliance status without exposing the underlying document data.

## Verification for Third Parties
Institutional verifiers can interact with the user's public address to confirm the presence of the `Z-RWA-COMPLY` marker. The presence of this token on-chain serves as proof-of-compliance, backed by the cryptographic guarantees of the Groth16 proof verified on Solana.
