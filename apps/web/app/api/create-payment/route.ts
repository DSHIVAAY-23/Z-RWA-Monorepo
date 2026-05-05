import { NextResponse } from 'next/server';
import { createCheckoutSession } from '../../../lib/dodo';
import { paymentStore } from '../../../lib/paymentStore';

/**
 * POST /api/create-payment
 * Accepts: { amount_inr: number, wallet_address: string }
 * Returns: { checkoutUrl: string, paymentId: string }
 *
 * Creates a Dodo Payments one-time checkout session and returns the
 * hosted checkout URL for the client to redirect to.
 */
export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { amount_inr, wallet_address } = body;

    // --- Validation ---
    if (!amount_inr || typeof amount_inr !== 'number' || amount_inr < 1000 || amount_inr > 1000000) {
      return NextResponse.json(
        { error: 'amount_inr must be a number between 1000 and 1000000' },
        { status: 400 }
      );
    }

    if (!wallet_address || typeof wallet_address !== 'string' || wallet_address.trim().length < 32) {
      return NextResponse.json(
        { error: 'Valid Solana wallet_address is required' },
        { status: 400 }
      );
    }

    const walletTrimmed = wallet_address.trim();
    
    // Dynamically determine app URL from headers if env is missing or localhost
    const host = request.headers.get('host');
    const protocol = host?.includes('localhost') ? 'http' : 'https';
    let appUrl = process.env.NEXT_PUBLIC_APP_URL || `${protocol}://${host}`;
    
    // Ensure no trailing slash
    if (appUrl.endsWith('/')) appUrl = appUrl.slice(0, -1);

    console.log(`[create-payment] Initiating INR ${amount_inr} payment for wallet ${walletTrimmed}`);

    // --- Create Dodo Checkout Session ---
    // amount_inr → paise (×100) for Dodo's smallest unit
    // Dodo appends session/payment ID to return_url automatically
    const { checkoutUrl, paymentId } = await createCheckoutSession({
      amount: amount_inr * 100,
      currency: 'INR',
      metadata: {
        aadhaar_hash: '',
        pan_hash: '',
        wallet_address: walletTrimmed,
      },
      redirectUrl: `${appUrl}/invest/success?wallet=${encodeURIComponent(walletTrimmed)}`,
      webhookUrl: `${appUrl}/api/dodo-webhook`,
    });

    // --- Store state for webhook correlation ---
    paymentStore.set(paymentId, {
      walletAddress: wallet_address.trim(),
      aadhaarHash: '',
      panHash: '',
      status: 'pending',
    });

    console.log(`[create-payment] Session created → paymentId=${paymentId}, url=${checkoutUrl}`);

    return NextResponse.json({ checkoutUrl, paymentId });

  } catch (error: any) {
    console.error('[create-payment] Error:', error);
    return NextResponse.json(
      { error: error?.message || 'Failed to create payment session' },
      { status: 500 }
    );
  }
}
