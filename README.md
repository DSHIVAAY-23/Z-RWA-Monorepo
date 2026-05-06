# Z-RWA — ZK Compliance Infrastructure for Indian RWA

> Privacy-preserving KYC for Real World Asset tokenization on Solana.  
> Identity never leaves your device. Compliance is proven cryptographically.

🌐 **Live Demo:** https://z-rwa-monorepo.vercel.app  
🎥 **Demo Video:** https://www.loom.com/share/7bf935bef303412ebae2f0a1cee35e24

---

## The Problem

India's DPDP Act 2023 makes storing Aadhaar/PAN on public blockchains **illegal**.  
Traditional KYC requires trusting a centralized verifier — a single point of failure.  
Institutions won't enter permissionless DeFi without cryptographic compliance guarantees.

## The Solution

Z-RWA generates a **Groth16 ZK proof** that an investor is compliant — without revealing any identity data.

```
User uploads Aadhaar/PAN locally
        ↓
Poseidon hash computed on device
        ↓
SnarkJS (Groth16) generates ZK compliance proof
        ↓
Only 260-byte proof submitted to Solana
        ↓
Anchor program verifies proof on-chain
        ↓
Token2022 RWA token minted to wallet
```

Chain learns nothing. Compliance is proven. Identity stays private.

---

## Technical Stack

| Layer | Technology |
|-------|-----------|
| ZK Proving | SnarkJS / Circom (Groth16) |
| On-chain | Solana — Anchor framework |
| Token Standard | Token2022 with custom transfer hooks |
| Hashing | Poseidon (ZK-friendly) |
| Frontend | Next.js + Wallet Adapter |
| Network | Solana Devnet |

---

## Key Metrics

| Metric | Value |
|--------|-------|
| Proof Size | 260 bytes |
| Constraints | 7,493,634 |
| Proving Time | ~2-3 seconds |
| On-chain Verification | Sub-second |
| Proof Format | Groth16 |

---

## How It Works

1. **Document Upload** — Aadhaar/PAN processed locally via OCR. Zero data transmitted.
2. **ZK Proof Generation** — SnarkJS runs the Circom compliance circuit. Groth16 proof generated.
3. **On-chain Submission** — 260-byte proof submitted to Solana via Anchor program.
4. **Token Minting** — `z_rwa_verifier` program verifies proof → Token2022 minted to wallet.
5. **Transfer Enforcement** — Every RWA transfer gated by Token2022 hook checking proof validity.

---

## Local Setup

```bash
git clone https://github.com/DSHIVAAY-23/Z-RWA-Monorepo
cd Z-RWA-Monorepo/apps/web
cp .env.example .env
npm install
npm run dev
```

Open http://localhost:3000

## Deployed Contracts (Solana Devnet)

| Contract | Address |
|----------|---------|
| Z-RWA Verifier Program | `GL8vm2SxWV7yHQbwoZegM7SkbJbEbEDn6A9m9W2XjeQe` |
| RWA Compliance Mint (Token2022) | `8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa` |
| Backend Authority | `GsPrDLXoqVbcWwofYpRZFJg4h5dzHEjyNfPyzPrcUKGd` |

**Recent verified transactions:**

- View program on Explorer: [GL8vm2...XjeQe](https://explorer.solana.com/address/GL8vm2SxWV7yHQbwoZegM7SkbJbEbEDn6A9m9W2XjeQe?cluster=devnet)
- View RWA Mint: [8GWCAZ...Jcaa](https://explorer.solana.com/address/8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa?cluster=devnet)
- Example verified tx: [2EqWJg...uTTg](https://explorer.solana.com/tx/2EqWJg6GFR2mYQKWk5hJUYyPxmHxN3qMZaAfQWYCx6GGS56JkoKRdzdDsW2K7A3BcyLM8ZMoE26VsXHScuSauTTg?cluster=devnet)

### Environment Variables

```env
BACKEND_WALLET_SECRET=    # contents of id.json (numeric array)
NEXT_PUBLIC_SOLANA_NETWORK=devnet
```

---

## Project Structure

```text
Z-RWA-Monorepo/
├── apps/web/                    # Next.js frontend + API routes
│   ├── app/                     # Pages, components, API handlers
│   ├── lib/                     # Solana, ZK, Dodo SDK wrappers
│   └── public/                  # Static assets
├── circuits/                    # Circom circuits + trusted setup
│   ├── compliance.circom        # Main KYC compliance circuit
│   └── setup/                   # Powers of Tau ceremony outputs
├── ZK-RAG/                      # ZK-RAG prover service
├── Z-RWA/
│   └── programs/
│       └── z_rwa_verifier/      # Anchor on-chain verifier (Rust)
└── scripts/                     # Deployment + utility scripts
```

---

## Hackathon Tracks

| Track | Sponsor | Branch | Status |
|-------|---------|--------|--------|
| Colosseum Frontier | Colosseum | main | ✅ Submitted |
| 100xDevs Frontier | 100xDevs | feature/100xdevs | ✅ Submitted |
| Dodo Payments | Superteam India | feature/dodo-payments | ✅ Submitted |
| Privacy Track | MagicBlock | main | ✅ Submitted |
| Security Audit | Adevar Labs | main | ✅ Submitted |
| Infrastructure | RPC Fast | main | ✅ Submitted |

---

## Why This Matters

- **SEBI** is expected to release an RWA framework in 2026 — infrastructure needs to exist before regulation
- **500M+ Indian retail investors** cannot participate in RWA DeFi today due to compliance barriers
- **DPDP Act 2023** makes centralized KYC storage a legal liability — ZK is the only compliant path

---

## Security Model

- Identity data never leaves the user's device
- Only a 260-byte Groth16 proof is submitted on-chain
- Token2022 transfer hooks enforce compliance on every transfer — not just at mint
- Proof binding via `ZK_RAG_VKEY` prevents proof replay across different verification keys

---

---

## QVAC Integration (Tether Side Track)

Z-RWA uses QVAC SDK for fully local document OCR — no cloud, no API calls.

### Why QVAC?
Our privacy guarantee is "identity never leaves your device."
QVAC OCR makes this technically enforceable:
- Aadhaar/PAN OCR runs entirely on the user's hardware
- QVAC uses Vulkan API — hardware-agnostic, works on any GPU
- Offline capable — no internet required for document processing

### QVAC Modules Used
- @qvac/sdk — Native SDK for local processing
- @qvac/ocr-onnx — Local OCR for Aadhaar/PAN document extraction

### Setup
npm install @qvac/sdk @qvac/ocr-onnx

---

## Dodo Payments Integration

**Flow:** INR Payment (UPI/Card) → Dodo Webhook → ZK Proof Generation → Token2022 Mint

**Live:** https://z-rwa-monorepo.vercel.app/invest

Setup:
1. Get testmode API key from app.dodopayments.com
2. Create a one-time payment product in Dodo dashboard  
3. Add to .env: DODO_API_KEY, DODO_WEBHOOK_SECRET, NEXT_PUBLIC_DODO_PRODUCT_ID
4. Set webhook URL to: https://z-rwa-monorepo.vercel.app/api/dodo-webhook

---

Built for Colosseum Frontier 2026 · SnarkJS · Circom · Anchor · Token2022 · Solana Devnet
