# Z-RWA Privacy Model

## What data is private
- Aadhaar number (12-digit Indian national ID)
- PAN number (Permanent Account Number)
- Purchase amounts (via MagicBlock Private Payments)
- Recipient wallet address (via MagicBlock shielded transfers)

## How identity privacy works

1. User enters Aadhaar/PAN locally in the browser
2. Data is hashed using Poseidon (Solana-native ZK-friendly hash function)
3. Hashed values are fed into the Circom circuit as private inputs
4. SnarkJS executes the Circom circuit locally, outputs a Groth16 proof
5. Only the proof is submitted to Solana — no raw or hashed identity data
6. Anchor program verifies the proof on-chain
7. Token2022 transfer hook checks proof validity before every RWA transfer

**What the chain sees:** Proof bytes + verification result. Nothing else.
**What the chain never sees:** Aadhaar number, PAN number, or their hashes.

## How payment privacy works

RWA purchases are settled via MagicBlock's Private Payments API /
Private Ephemeral Rollup (PER). Payment amounts and recipient 
addresses are shielded from public on-chain observers.

The ZK compliance proof is linked to the private payment via a 
proof hash — proving the buyer is compliant without revealing who they are.

## Threat model

| Threat | Mitigation |
|---|---|
| Aadhaar/PAN leaked on-chain | Never transmitted — stays on user device |
| Identity data stolen from server | No server receives identity data |
| On-chain observer links payment to identity | MagicBlock PER shields amounts + recipients |
| Centralized KYC provider sells data | No centralized KYC provider in the flow |
| Regulatory non-compliance | ZK proofs are cryptographically verifiable by regulators |

## Regulatory context

India's Digital Personal Data Protection (DPDP) Act 2023 prohibits 
storing personal identifiers (Aadhaar, PAN) on public blockchains.
Z-RWA is architected from the ground up to comply with DPDP — 
no personal data is ever stored on-chain or on any server.

## MagicBlock integration

Z-RWA uses MagicBlock's Private Payments API for shielded RWA 
purchases. This adds payment privacy on top of the existing 
identity privacy from ZK proofs — creating a full-stack 
private compliance system.
