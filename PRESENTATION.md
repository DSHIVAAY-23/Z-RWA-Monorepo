# Z-RWA — Colosseum Frontier 2026
## Canva Presentation Outline

> Paste each slide section into a Canva slide. Suggested template: dark background, purple/teal accent palette.

---

## SLIDE 1 — TITLE

**Headline:**
# Z-RWA
## The Compliance Layer for Institutional DeFi

**Subheadline:**
ZK-proven KYC for Indian RWA Tokenization on Solana
Identity never leaves your device.

**Footer line:**
Built for Colosseum Frontier 2026

**Links:**
- 🌐 z-rwa-monorepo.vercel.app
- 💻 github.com/DSHIVAAY-23/Z-RWA-Monorepo

---

## SLIDE 2 — THE PROBLEM

**Headline:**
# India's $3.5T RWA Market Is Locked

**Three laws collide:**

❌ **DPDP Act 2023**
Storing Aadhaar/PAN on public blockchain = ILLEGAL

❌ **SEBI KYC Requirements**
RWA tokenization needs identity verification

❌ **No Trustless Solution**
Centralized KYC = single point of failure + data breach risk

**Impact callout (large text):**
> 500M+ Indian retail investors cannot access RWA DeFi today.

---

## SLIDE 3 — THE SOLUTION

**Headline:**
# Z-RWA: Prove Compliance Without Revealing Identity

**Flow diagram (build as a vertical step diagram in Canva):**

```
[User Device]
Aadhaar / PAN uploaded
        ↓
QVAC Local OCR
(zero data transmitted)
        ↓
Poseidon hash computed on-device
        ↓
SnarkJS Groth16 Circuit
7,493,634 constraints → 260-byte proof
        ↓
Solana Anchor Program
Proof verified on-chain, sub-second
        ↓
Token2022 Mint
Z-RWA-COMPLY token delivered to wallet
```

**Two-line conclusion (large, highlighted):**
✅ Chain learns: proof is valid
❌ Chain learns nothing else.

---

## SLIDE 4 — TECHNICAL ARCHITECTURE

**Headline:**
# Production-Ready ZK Stack on Solana

**Table (use Canva table element):**

| Layer | Technology |
|---|---|
| ZK Proving | Circom 2.0 + SnarkJS (Groth16) |
| On-chain | Solana Anchor + Token2022 |
| Local OCR | QVAC (@qvac/ocr-onnx) |
| INR Payments | Dodo Payments SDK |
| RPC Layer | RPC Fast |
| Agent Layer | Zerion CLI fork |
| Analytics | GoldRush / Covalent API |
| Privacy Layer | Cloak SDK (shielded transfers) |

**Key metrics (4 stat boxes):**

| Metric | Value |
|---|---|
| Proof size | 260 bytes |
| Constraints | 7,493,634 |
| Proving time | 2–3 seconds |
| On-chain verification | Sub-second |

---

## SLIDE 5 — LIVE PRODUCT

**Headline:**
# Deployed on Solana Devnet Today

**Live URL:**
🌐 z-rwa-monorepo.vercel.app

**Live stats (3 large number boxes):**

| Stat | Value |
|---|---|
| Proofs Generated | 24 |
| Wallets Verified | 13 |
| RWA Tokens Minted | 11 |

**On-chain contracts:**
- Verifier Program: `GL8vm2SxWV7yHQbwoZegM7SkbJbEbEDn6A9m9W2XjeQe`
- RWA Mint (Token2022): `8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa`

**Full flow checklist:**
- ✅ QVAC local OCR → ZK proof → Token2022 mint
- ✅ INR payment via Dodo → ZK proof → Token2022 mint
- ✅ Autonomous compliance agent (Zerion CLI)
- ✅ Live wallet analytics (GoldRush)

---

## SLIDE 6 — HACKATHON INTEGRATIONS

**Headline:**
# Built With the Frontier Ecosystem

**6 sponsor integrations (use icon + title + 2-line desc cards):**

---

🔒 **QVAC (Tether)**
Local OCR — Aadhaar/PAN processed on-device.
Zero data transmitted. Privacy technically enforced.

---

💳 **Dodo Payments (Superteam India)**
INR → ZK proof → Token2022.
India's first ZK-compliant RWA investment flow.

---

⚡ **RPC Fast**
Groth16 proof payloads exceed standard RPC limits.
RPC Fast handles large proof submissions reliably.

---

🤖 **Zerion CLI**
Autonomous compliance agent.
Scoped policies: chain lock, spend limits, proof age.

---

📊 **GoldRush / Covalent**
Live wallet analytics on Check Wallet page.
Real token balances + transaction history.

---

🔐 **Cloak**
Shielded RWA transfers.
Compliance proven + payment private simultaneously.

---

## SLIDE 7 — WHY NOW

**Headline:**
# The Regulatory Window Is Open

**Timeline (horizontal timeline in Canva):**

```
2023 ──── 2024 ──── 2026 ──── 2026
  │          │         │         │
DPDP Act   SEBI     SEBI RWA  Colosseum
passed     RWA      framework  Frontier
           paper    expected   Hackathon
```

**Positioning statement (large quote block):**
> Z-RWA is the compliance infrastructure India's RWA market needs BEFORE regulation arrives.

**Market size (3 stat boxes):**

| | |
|---|---|
| Indian RWA market | $3.5 Trillion |
| Retail investors | 500 Million+ |
| Compliant ZK solutions on Solana | 0 |

---

## SLIDE 8 — PHASE 2 ROADMAP

**Headline:**
# What's Coming

**Three roadmap items (use icon + title + desc card layout):**

---

🏛️ **DigiLocker Integration**
Documents pulled directly from India's government vault.
Zero fake document risk.

---

⚡ **SP1 RISC-V zkVM Migration**
Migrate from SnarkJS/Circom to Succinct's SP1 RISC-V zkVM for native program proving.
Hardware acceleration via GPU/ICICLE. Sub-second proving.

---

🏗️ **Full RWA Platform**
Land records, bonds, agricultural commodities.
Compliance enforced on every transfer via Token2022 hooks.

---

## SLIDE 9 — TEAM

**Headline:**
# Built by Divyank Rai

**Tagline:**
Solo developer — 3 years multi-chain experience

**Stack expertise (tag badges):**
`EVM` · `Cosmos SDK` · `Solana` · `Circom` · `SnarkJS` · `SP1`

**Projects:**

| Project | Description |
|---|---|
| Z-RWA | ZK compliance on Solana (this project) |
| TradeView | Perpetual DEX on dYdX v4 Cosmos fork |
| Universal Privacy Engine | Groth16/Circom on Oasis Sapphire |
| Citizen of Arcanis | Cairo/Starknet MMORPG contributions |

**Also building:**
Hardware-accelerated ZK prover in Rust (GPU/ICICLE target)

**Links:**
- GitHub: DSHIVAAY-23
- Twitter: @zrwaprotocol · @RaiDivyank

---

## SLIDE 10 — CLOSING

**Headline:**
# The Compliance Layer India's DeFi Needs

**Hero quote (largest text on slide):**
> "Prove compliance without revealing identity.
> On Solana. In production. Today."

**Links:**
- 🌐 z-rwa-monorepo.vercel.app
- 💻 github.com/DSHIVAAY-23/Z-RWA-Monorepo
- 🎥 [Loom demo link — add before submission]
- 🐦 @zrwaprotocol

**Footer:**
Built for Colosseum Frontier 2026
Powered by SnarkJS · Anchor · Token2022 · Solana Devnet

---

## CANVA DESIGN NOTES

- **Background:** `#000000` or `#0a0a0a`
- **Accent 1 (purple):** `#8b5cf6`
- **Accent 2 (teal):** `#14b8a6`
- **Body text:** `#9ca3af`
- **Highlight text:** `#ffffff`
- **Font (headings):** Space Grotesk Bold
- **Font (body):** Inter Regular
- **Font (code/addresses):** JetBrains Mono
- **Slide size:** 1920 × 1080 (16:9 widescreen)
