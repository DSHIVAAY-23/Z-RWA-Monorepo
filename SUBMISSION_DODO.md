# Z-RWA × Dodo Payments — Superteam India Submission

## Project
Z-RWA: India's First ZK-Compliant RWA Onboarding with INR Payments

## One Line
Pay in INR via UPI → get ZK-verified RWA tokens on Solana. 
No USDC needed. No KYC data stored. Fully compliant.

## The Problem
Indian retail investors cannot access RWA tokenization because:
1. Onboarding requires USDC — retail investors don't hold stablecoins
2. KYC laws (DPDP Act 2023) prohibit storing Aadhaar/PAN on-chain
3. Minimum investment sizes exclude small investors (₹50,000+)

## The Solution
Z-RWA + Dodo Payments creates the complete INR → RWA pipeline:

- **Dodo Payments** handles INR collection (UPI, card, netbanking)
- **SnarkJS Groth16** generates ZK compliance proof locally via Circom circuit (no PII on-chain)
- **Token2022 + Anchor** mints RWA tokens, gated on valid ZK proof
- **Settlement in < 60 seconds**, not T+2 days

## Why Solana + Stablecoins Beat the Status Quo
Traditional RWA: T+2 settlement, centralized KYC, ₹50k minimums
Z-RWA + Dodo: <60s settlement, ZK privacy, ₹1,000 minimum, 
              500M+ addressable users with UPI

## Dodo Integration
```typescript
// lib/dodo.ts
export async function createCheckoutSession(params: {
  amount: number;        
  currency: "INR";
  // ... metadata ...
}) {
  const checkoutSession = await dodo.checkoutSessions.create({
     product_cart: [{
             name: "Z-RWA Investment Token",
             amount: params.amount,
             price: params.amount,
             currency: params.currency,
             quantity: 1,
        } as any],
        return_url: params.redirectUrl,
        metadata: params.metadata,
  });
  return checkoutSession;
}
```

Webhook flow:
```typescript
// api/dodo-webhook/route.ts
if (!verifyWebhookSignature(payload, signature, webhookSecret)) {
  return NextResponse.json({ error: "Invalid signature" }, { status: 401 });
}
// Trigger async ZK Ment Orchestrator on valid payment notification
setTimeout(() => {
    generateAndSubmitProof({ ...storedState }).catch(err => console.error(err));
}, 0);
```

## Market Opportunity
SEBI's RWA tokenization framework is expected in 2026.
India has ₹300 trillion in household savings, mostly in 
fixed deposits earning <7% — RWA tokens offering 10-12% 
yield on real assets represent a massive market.
Dodo + Z-RWA is the onboarding layer that makes this accessible.

## Links
- Live demo: https://z-rwa-monorepo-fzeb4r6c1-dshivaay23s-projects.vercel.app/invest
- GitHub (feature/dodo-payments branch): https://github.com/DSHIVAAY-23/Z-RWA-Monorepo/tree/feature/dodo-payments
- Demo video: https://www.loom.com/share/7bf935bef303412ebae2f0a1cee35e24
- Colosseum submission: [URL]
