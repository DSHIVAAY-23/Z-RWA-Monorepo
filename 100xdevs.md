# Z-RWA — 100xDevs Frontier Hackathon Submission

## One-Line Pitch

ZK-proven KYC for Real World Asset tokenization on Solana — your Aadhaar/PAN never leaves your device, only a 260-byte proof goes on-chain.

---

## The Problem

India's DPDP Act 2023 makes it **illegal** to store Aadhaar or PAN data on-chain. Yet SEBI's upcoming RWA framework (expected 2026) requires verified investor identity for tokenized assets. Every existing solution forces a choice: compliance OR privacy.

Centralized KYC providers are honeypots for identity theft. Storing hashes on-chain still leaks linkable identity data. There is no production-grade, privacy-first KYC layer for Indian RWA on Solana today.

---

## The Solution

Z-RWA uses the **SP1 RISC-V zkVM** to run a compliance circuit locally on the user's device. It outputs a **Groth16 proof** — a 260-byte cryptographic certificate that says "this person is KYC-compliant" without revealing who they are or what their documents contain.

That proof is verified on Solana via an **Anchor program**. A **Token2022** compliance token is minted only after on-chain proof verification. Every subsequent RWA transfer is gated by a **Token2022 transfer hook** that checks proof validity — no middleman, no data exposure, fully on-chain.

---

## Judging Criteria — Direct Response

### Technical Execution

| Component | Detail |
|---|---|
| ZK Circuit | SP1 RISC-V zkVM, Groth16 proving scheme |
| Constraint count | 7,493,634 |
| Proof size | 260 bytes |
| Proving time | ~23.4 seconds (hardware-benchmarked) |
| On-chain program | Anchor framework, Solana Devnet |
| Token standard | Token2022 with custom transfer hooks |
| Hashing | Poseidon (ZK-friendly, Solana-native) |
| Frontend | Next.js 14, TypeScript, @solana/wallet-adapter |

The ZK circuit enforces three constraints: Aadhaar number validity (Verhoeff checksum), PAN format, and investor age sanity. No raw values leave the client.

### Innovation

- **First SP1-based KYC compliance layer on Solana** — most ZK projects on Solana use SnarkJS/Circom; SP1 RISC-V proves arbitrary Rust programs
- **DPDP Act-compliant by design** — ZK is not a feature, it is the legal requirement
- **Token2022 transfer hooks as a compliance enforcement layer** — every transfer re-checks proof validity, not just at mint time
- **Poseidon hashing** — standard Keccak/SHA is expensive in ZK circuits; Poseidon is designed for SNARKs and runs natively on Solana

### Real-World Use Case

- 500M+ Indian retail investors cannot access tokenized RWA today due to the KYC/privacy conflict
- SEBI RWA framework is expected in 2026 — Z-RWA is the compliant infrastructure layer
- Aadhaar/PAN cannot legally be stored on-chain under DPDP Act 2023 — ZK is the only viable solution
- Standard public RPCs reject Groth16 proof payloads (too large) — production deployment requires premium RPC, demonstrating real infrastructure thinking

### User Experience

1. Connect Solana wallet (Phantom/Solflare on Devnet)
2. Enter Aadhaar number + PAN — both masked, never transmitted
3. Click "Generate ZK Proof" — SP1 circuit runs, status indicator shows each stage
4. Proof verified on-chain, Token2022 minted to wallet
5. Every stat card updates live: proof status, token balance, recent transactions with Explorer links

Developer Mode panel (toggleable) exposes raw circuit inputs, proof bytes, constraint count, proving time, and on-chain verification result — designed for technical judges.

### Completeness

- Live on Solana Devnet
- Full end-to-end flow: document → ZK proof → on-chain verification → Token2022 mint
- Production Vercel deployment
- Token2022 transfer hooks deployed and active
- README with architecture diagram and 5-command local setup

---

## Architecture

```
User Device                    Solana Devnet
─────────────────────          ──────────────────────────────
Aadhaar + PAN input
        │
        ▼
SP1 RISC-V zkVM
  compliance circuit
  (7.4M constraints)
        │
        ▼
Groth16 Proof (260 bytes) ──►  Anchor Program: verify_and_mint
                                        │
                                        ▼
                               Token2022 Mint
                               (compliance token → user ATA)
                                        │
                                        ▼
                               Transfer Hook
                               (every RWA transfer gated
                                by proof validity check)
```

---

## ZK Flow — How It Works

1. User inputs Aadhaar + PAN locally (password-masked, never sent to any server)
2. SP1 RISC-V zkVM runs the compliance circuit on a Poseidon hash of the inputs
3. Circuit checks: Verhoeff checksum (Aadhaar), PAN regex format, age sanity
4. Groth16 prover outputs a 260-byte proof + public signals
5. Proof submitted to Solana Anchor program via `verify_and_mint` instruction
6. On-chain verifier checks proof against the verification key stored in the program
7. If valid: Token2022 compliance token minted to user's associated token account
8. Transfer hook enforces the same check on every subsequent RWA transfer

---

## Local Setup (5 commands)

```bash
git clone https://github.com/DSHIVAAY-23/Z-RWA-Monorepo
cd Z-RWA-Monorepo
cp apps/web/.env.example apps/web/.env.local   # add BACKEND_WALLET_SECRET
npm install
cd apps/web && npm run dev                       # http://localhost:3000
```

---

## Deployment

| Resource | Value |
|---|---|
| Live Frontend | https://z-rwa-monorepo-3uvdrrfr7-dshivaay23s-projects.vercel.app |
| Anchor Program ID | `3SN3zAmuW5HWgJy5mcWjvy8vwDZRLosEajqydbuxiEZC` |
| Token2022 Mint | `FhuXW2JHUyTNFF8eXW1EYsfuWcx3RfzdXHuDPvN7A7Xc` |
| Network | Solana Devnet |
| Explorer | [View Program](https://explorer.solana.com/address/3SN3zAmuW5HWgJy5mcWjvy8vwDZRLosEajqydbuxiEZC?cluster=devnet) |

---

## Project Structure

```
Z-RWA-Monorepo/
├── circuits/                    # SP1 RISC-V compliance circuit (Rust)
├── Z-RWA/programs/z_rwa/        # Anchor program: verify_and_mint + transfer hook
├── apps/web/
│   ├── app/page.tsx             # Main dashboard (wallet, proof, tokens, txns)
│   ├── app/api/prove/           # SP1 proof generation API
│   ├── app/api/mint-token/      # Backend Token2022 minting authority
│   ├── hooks/useProofGeneration.ts
│   └── lib/mintRwaToken.ts
└── scripts/                     # Devnet setup and minting utilities
```

---

## Why This Will Scale

- ZK proofs are stateless and verifiable by anyone — no oracle, no trusted third party
- Token2022 transfer hooks make compliance enforcement **automatic and permanent**
- The same ZK circuit can be extended to income proofs, accreditation checks, or FATF travel rule compliance
- SP1 RISC-V proving is hardware-acceleratable — proving time drops to under 1 second with dedicated provers (Succinct Network)

---

*Built for the 100xDevs Frontier Track — Colosseum Frontier Hackathon 2026*
*GitHub: [DSHIVAAY-23/Z-RWA-Monorepo](https://github.com/DSHIVAAY-23/Z-RWA-Monorepo)*
