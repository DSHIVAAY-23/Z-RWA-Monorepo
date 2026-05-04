# Z-RWA — ZK Compliance Infrastructure for Indian RWA

> Privacy-preserving KYC for Real World Asset tokenization on Solana.  
> Identity never leaves your device. Compliance is proven cryptographically.

🌐 **Live Demo:** https://z-rwa-monorepo.vercel.app  
📦 **GitHub:** https://github.com/DSHIVAAY-23/Z-RWA-Monorepo  
🎥 **Demo Video:** [ADD LOOM LINK HERE]

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
SP1 RISC-V zkVM generates Groth16 proof
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
| ZK Proving | SP1 RISC-V zkVM (Groth16) |
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
| Proving Time | ~23.4 seconds |
| On-chain Verification | Sub-second |
| Proof Format | Groth16 |

---

## How It Works

1. **Document Upload** — Aadhaar/PAN processed locally via OCR. Zero data transmitted.
2. **ZK Proof Generation** — SP1 RISC-V zkVM runs the compliance circuit. Groth16 proof generated.
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

### Environment Variables

```env
BACKEND_WALLET_SECRET=    # contents of id.json (numeric array)
NEXT_PUBLIC_SOLANA_NETWORK=devnet
```

---

## Project Structure

```
Z-RWA-Monorepo/
├── apps/web/              # Next.js frontend + API routes
│   ├── app/               # Pages and components
│   └── api/               # mint-token, verify, stats endpoints
├── circuits/              # ZK circuits and setup scripts
└── Z-RWA/
    └── programs/
        └── z_rwa_verifier/ # Anchor on-chain verifier program
```

---

## Hackathon Tracks

| Track | Branch | Deadline | Status |
|-------|--------|----------|--------|
| Colosseum Frontier (Main) | main | May 2026 | ✅ Live |
| 100xDevs Frontier | feature/100xdevs | May 25 | ✅ Submitted |
| Privacy Track — MagicBlock | main | May 27 | 🔄 In Progress |
| Dodo Payments — Superteam India | feature/dodo-payments | May 26 | 🔄 In Progress |
| Eitherway / QuickNode | feature/quicknode-rpc | May 27 | 🔄 In Progress |
| Zerion Agent | feature/zerion-agent | May 26 | 🔄 In Progress |
| Encrypt & Ika | feature/encrypt-ika | Jun 1 | 📋 Planned |
| Adevar Labs Security | main | Jun 10 | 📋 Planned |

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

## Dodo Payments Integration

**Superteam India × Dodo Payments Hackathon Track**

### Flow

```
INR Payment (UPI / Card / Net Banking)
        ↓  Dodo Payments hosted checkout
Payment Confirmed (webhook → /api/dodo-webhook)
        ↓  Signature verified via standardwebhooks
ZK Proof Generated (SP1 Groth16 — identity stays private)
        ↓  260-byte proof submitted to Solana
Token2022 RWA Token Minted to Investor Wallet
```

### Setup

1. Get a **testmode API key** from [app.dodopayments.com](https://app.dodopayments.com)
2. Create a **one-time payment product** in the Dodo dashboard — note the Product ID
3. Add the following to your `.env` file:
   ```env
   DODO_API_KEY=your_dodo_testmode_api_key_here
   DODO_WEBHOOK_SECRET=your_webhook_secret_here
   NEXT_PUBLIC_DODO_PRODUCT_ID=your_product_id_here
   NEXT_PUBLIC_APP_URL=https://your-vercel-url.vercel.app
   ```
4. In the Dodo dashboard, set your **Webhook URL** to:
   ```
   https://your-domain.vercel.app/api/dodo-webhook
   ```

### Key API Routes

| Route | Method | Purpose |
|-------|--------|---------|
| `/api/create-payment` | POST | Creates Dodo checkout session, returns `checkoutUrl` |
| `/api/dodo-webhook` | POST | Verifies signature, triggers ZK proof + mint |
| `/api/payment-status/:id` | GET | Polls payment/mint status for UI |
| `/invest` | GET | INR investment UI (3-step: ID → Pay → Receive) |
| `/invest/success` | GET | Post-payment success page with ZK status |

### Live Demo

- **Invest page:** https://z-rwa-dodo.vercel.app/invest
- **Branch:** `feature/dodo-payments`

---

Built for Colosseum Frontier 2026 · Powered by SP1 · Solana · Token2022  
Developer: [@DSHIVAAY-23](https://github.com/DSHIVAAY-23)
