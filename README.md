# Z-RWA Compliance Vault — Kamino Track Submission

## What This Is
A KYC-gated Kamino vault using zero-knowledge proofs for 
privacy-preserving compliance. Solves the institutional DeFi problem: 
compliance without sacrificing privacy.

## How It Works
1. User generates ZK proof (SP1 Groth16) proving: age ≥18, Indian national, income threshold met
2. Proof submitted to our Anchor program
3. Program verifies proof, then calls Kamino vault deposit via CPI
4. Vault shares minted with transfer hooks preventing non-compliant transfers

## Tech Stack
- Kamino Finance SDK (vault infrastructure)
- SP1 RISC-V zkVM (Groth16 proving)
- Anchor (Solana program framework)
- QuickNode RPC (with Priority Fee API)
- Next.js + Tailwind CSS

## Live Demo
[Vercel URL - To Be Deployed]

## Technical Features
- **Client-Side OCR Gate**: Integration of `tesseract.js` for local document scanning and validation.
- **SP1 RISC-V Proving**: Efficient proof generation with ~7.4M constraints and hardware-verified proving times (~23.4s).
- **Embedded Groth16 Verifier**: Hardcoded verification keys (`ZK_RAG_VKEY`) within the Anchor program for immutable validation.
- **Token2022 Compliance**: Native support for Solana's latest token standard, enabling permanent delegates and compliance metadata.

## QuickNode Integration

Z-RWA uses **QuickNode** as its primary Solana RPC provider across all critical operations:

- **ZK Proof Submission**: All Groth16 proof transactions route through QuickNode. Standard public endpoints drop large proof payloads (~260 bytes) under load.
- **Priority Fees**: `qn_estimatePriorityFees` API ensures optimal fees for every proof transaction — no manual guessing.
- **Real-time Status**: Live slot, TPS, and fee data from QuickNode displayed in the UI (10-second refresh).
- **Transaction History**: `getSignaturesForAddress` powers the wallet activity panel in the compliance checker.

### QuickNode Endpoint Configuration
```env
NEXT_PUBLIC_QUICKNODE_RPC_URL=https://frequent-alpha-pool.solana-devnet.quiknode.pro/5f06a41cf6e077af5ca7ac464fbf1caed5c84d42/
```
**Why QuickNode (not public devnet)?**
Groth16 proofs for 7.4M constraint circuits generate extremely large transactions. Standard RPC endpoints have strict payload limits and aggressive rate limits that cause proof submissions to fail under load. QuickNode's dedicated endpoint handles this reliably.

## Solflare Integration

Z-RWA has a deep integration with **Solflare Wallet**, treating it as a core interface layer rather than a simple connect button.

- **Transaction History Visualization**: In the compliance dashboard, a rich wallet activity panel is presented for the connected Solflare user, merging their compliance status with a real-time transaction feed, powered by QuickNode.
- **Custom Transaction Signing Flow**: SP1 ZK proofs require a distinct verification sequence where the user signs an off-chain challenge matching their Solflare public key.
- **Wallet-first Onboarding**: The app leverages Solflare for signing, checking history, and eventually delegating asset rights, positioning the wallet at the center of institutional RWA actions.

## Technical Stack
- **Blockchain**: Solana (Anchor 0.31.1+)
- **ZK Proof System**: SP1 (Succinct), Groth16
- **Smart Contracts**: Rust (Anchor Framework)
- **Frontend**: Next.js, Tailwind CSS, Solana Web3.js
- **OCR Engine**: Tesseract.js (WASM)

## Setup
```bash
# 1. Install dependencies
npm i && cd apps/web && npm i

# 2. Setup your local env
cp apps/web/.env.example apps/web/.env.local

# 3. Build the Anchor program (using nightly rust toolchain)
cd Z-RWA && anchor build

# 4. Deploy Anchor Program
anchor deploy

# 5. Run the frontend development server
npm run dev
```
