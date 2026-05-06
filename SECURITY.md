# Z-RWA Security Architecture & Threat Model

## 1. Overview
Security is the foundational pillar of the Z-RWA protocol. Our architecture bridges off-chain zero-knowledge compliance proofs with on-chain Solana state transitions. Due to the high-stakes nature of Real World Assets (RWAs) and the experimental nature of custom ZK circuits, we operate under a zero-trust assumption, particularly regarding proof generation and transfer hook execution.

## 2. Threat Model & Attack Surfaces

### A. ZK Circuits (Proof Generation)
The core compliance engine relies on Circom + SnarkJS to generate Groth16 proofs of authorized participant status without revealing PII.
* **Soundness Bugs:** An attacker maliciously crafts inputs to generate a mathematically valid proof for an uncompliant state.
* **Completeness Bugs:** Valid, compliant participants are unable to generate a proof due to constraint errors, resulting in a denial of service.
* **Edge-Case Inputs:** Exploits leveraging zero values, unconstrained variables, or `max_uint64` overflows within the Rust circuit logic.

### B. Anchor On-Chain Programs (State & Verification)
The Solana program acts as the ultimate source of truth, verifying the Groth16 proofs and managing state.
* **Account Validation:** Missing or incorrect `Signer` checks, or account confusion attacks where an attacker passes a malicious account masquerading as the verifier or compliance state account.
* **Arithmetic Overflows:** Unchecked math operations in token amounts or compliance limits.
* **Proof Replay Attacks:** An attacker re-submitting a previously verified ZK proof to bypass current compliance checks.

### C. Token2022 Transfer Hooks (Execution)
We leverage Token2022 to enforce compliance on every single token transfer.
* **Hook Bypass:** Attackers finding edge cases in the SPL token program to execute direct transfers that fail to trigger the custom transfer hook.
* **Reentrancy & CPI Abuse:** Malicious Cross-Program Invocations (CPIs) during the hook execution phase attempting to drain liquidity or alter compliance state.
* **Stale State Acceptance:** The transfer hook evaluating a state that has been maliciously delayed or front-run.

## 3. Current Test Coverage & Gaps
* **Unit Testing:** Comprehensive Rust unit tests for individual circuit constraints.
* **Integration Testing:** Typescript test suite covering standard Anchor program interactions and successful/failed Token2022 transfers.
* **Identified Gaps:** We currently lack rigorous fuzzing on the Circom circuit inputs and formal verification of the Anchor account structs against edge-case CPIs.

## 4. Audit Goals
We are actively seeking a professional audit from Adevar Labs to focus heavily on the ZK circuit soundness and the Token2022 transfer hook integration, as these represent the most novel and critical attack vectors in the protocol.
