import { NextResponse } from 'next/server';
import { createCheckoutSession } from '../../../lib/dodo';
import { paymentStore } from '../../../lib/paymentStore';

export async function POST(request: Request) {
  try {
    const { amount, walletAddress, aadhaarHash, panHash } = await request.json();

    // 1. Validate inputs
    if (!amount || amount <= 0) {
      return NextResponse.json({ error: "Invalid amount" }, { status: 400 });
    }
    if (!walletAddress) {
      return NextResponse.json({ error: "Valid Solana pubkey required" }, { status: 400 });
    }
    if (!aadhaarHash || !panHash) {
      return NextResponse.json({ error: "Identity proofs required" }, { status: 400 });
    }

    const appUrl = process.env.NEXT_PUBLIC_APP_URL || 'http://localhost:3000';

    // 2. Call createCheckoutSession() from lib/dodo.ts
    const { checkoutUrl, paymentId } = await createCheckoutSession({
      amount: amount * 100, // Assuming incoming is INR, change to paise
      currency: "INR",
      metadata: {
        aadhaar_hash: aadhaarHash,
        pan_hash: panHash,
        wallet_address: walletAddress
      },
      redirectUrl: `${appUrl}/invest`,
      webhookUrl: `${appUrl}/api/dodo-webhook`
    });

    // 3. Store paymentId + metadata in the Map
    paymentStore.set(paymentId, {
      walletAddress,
      aadhaarHash,
      panHash,
      status: 'pending'
    });

    console.log(`[Checkout] Payment initialized ${paymentId} -> ${checkoutUrl}`);

    // 4. Return checkout details
    return NextResponse.json({ checkoutUrl, paymentId });

  } catch (error: any) {
    console.error("Checkout creation failed:", error);
    return NextResponse.json({ error: "Checkout error" }, { status: 500 });
  }
}
