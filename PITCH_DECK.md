% Z-RWA Pitch Deck

---

# Slide 1 — Title

Z-RWA: Privacy-preserving Institutional RWA Tokenization

One-liner: On-device ZK proofs + on-chain verification for compliant RWA issuance.

---

# Slide 2 — Problem

- Institutional RWA issuance requires proof of eligibility and compliance.  
- Existing workflows expose PII or require trusted off-chain intermediaries.

---

# Slide 3 — Solution

- Local RAG + Groth16 prover (ZK-RAG) generates compact proofs of compliance.  
- `z-rwa` Anchor program verifies proofs and mints Token2022 RWA tokens under programmable governance.

---

# Slide 4 — Architecture

- On-device: Document parsing, RAG scoring, Groth16 proof generation.  
- On-chain: Verifier, mint PDA, Token2022 extensions (permanent delegate, freeze).  
- Interop: multisig & CPI flows for cross-chain operations.

---

# Slide 5 — Traction

- Devnet deployment with program IDs and verification metrics.  
- ZK-RAG prover implemented in Rust; example proofs & tests included.

---

# Slide 6 — Roadmap & Ask

- 0–3mo: CI, security hardening, devnet reference deploys.  
- 3–6mo: Third-party audit, proving optimizations.  
- 6–12mo: Institutional pilot & mainnet support.  
- Ask: $250k (example) to complete milestones.

---

# Slide 7 — Team & Contacts

- Core maintainers: Z-RWA contributors.  
- Contact: ops@zrwa.example (replace with real contact).

---

# Slide 8 — Appendix / Demo

- Links: README, GRANT_PITCH.md, CODEBASE_SUMMARY.md, Anchor.toml (devnet IDs).  
- Demo: devnet mint flow and prover run commands in repository.
