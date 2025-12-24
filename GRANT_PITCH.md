# Z-RWA — Grant Pitch

## One-line summary
Privacy-preserving Real-World Asset tokenization pipeline for institutional issuers using ZK-proofs (ZK-RAG) and Solana on-chain verification (Z-RWA).

## Funding ask (suggested)
$250,000 to accelerate mainnet readiness, audits, and integrations (negotiable).

## Problem
Institutional RWA issuance lacks privacy-preserving compliance tooling that enables proof-of-eligibility without exposing PII.

## Solution
On-device RAG + Groth16 prover (ZK-RAG) produces succinct proofs verified on-chain by the `z-rwa` Anchor program to mint Token2022 RWA tokens under program-controlled governance.

## Key deliverables (3–12 months)
- Month 0–3: Security hardening, automated CI, developer docs, and devnet reference deployment.
- Month 3–6: Third-party smart contract audit, production-grade proving pipeline optimizations, and compliance docs (legal posture).
- Month 6–12: Pilot with an institutional partner, mainnet deployment support, and governance tooling for custody/operations.

## Milestones & budget allocation
- Engineering (45%): audits, performance, CI, developer DX.
- Security & compliance (25%): audits, legal review, KYC/AML integration design.
- Integrations & pilots (20%): partner integrations, pilot operations.
- Operations & community (10%): docs, bounties, outreach.

## Success metrics
- On-chain verifier GAS/compute within target limits for production (~<=300k CU on Solana).  
- Pilot issuance of at least one institutional RWA token within 12 months.  
- Third-party audit completed and public summary published.

## Team & contact
- Core Maintainers: Z-RWA contributors (see repository).  
- Contact: ops@zrwa.example (replace with real contact)  

## Demo & reproducibility
See repository root README and `Z-RWA` folder for devnet program IDs, prover engine (ZK-RAG), and test scripts. Provide a short demo video link in the application if available.
