import { NextResponse } from 'next/server';
import { verifyWebhookSignature } from '../../../lib/dodo';

export async function POST(request: Request) {
  console.log('[Webhook] Received Dodo Payment event');
  
  try {
    const signature = request.headers.get('webhook-signature') || '';
    const payload = await request.text();
    const webhookSecret = process.env.DODO_WEBHOOK_SECRET || '';

    // 1. Verify webhook signature
    if (!verifyWebhookSignature(payload, signature, webhookSecret)) {
      console.warn('[Webhook] Invalid signature');
      return NextResponse.json({ error: 'Invalid signature' }, { status: 401 });
    }

    const event = JSON.parse(payload);
    console.log(`[Webhook] Event type: ${event.type || 'unknown'}`);

    // 2. Handle payment.succeeded
    // Note: Dodo events usually follow the structure { type: 'payment.succeeded', data: { ... } }
    if (event.type === 'payment.succeeded') {
      const paymentData = event.data;
      const metadata = paymentData?.metadata || {};
      const walletAddress = metadata.wallet_address;

      if (walletAddress) {
        console.log(`[Webhook] Payment succeeded for wallet: ${walletAddress}. Triggering mint...`);
        
        // Call existing /api/mint-token
        // We use the absolute URL to ensure it works in all environments
        const appUrl = process.env.NEXT_PUBLIC_APP_URL || 'http://localhost:3000';
        
        // Trigger the minting process. 
        // We pass a dummy txSignature if the existing endpoint requires it, 
        // or we assume it's been updated to handle payment-triggered mints.
        try {
          const mintResponse = await fetch(`${appUrl}/api/mint-token`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ 
              walletAddress,
              txSignature: `DODO_${paymentData.payment_id || Date.now()}` // Internal ref
            }),
          });
          
          if (mintResponse.ok) {
            console.log(`[Webhook] Minting triggered successfully for ${walletAddress}`);
          } else {
            const err = await mintResponse.text();
            console.error(`[Webhook] Minting failed: ${err}`);
          }
        } catch (mintErr) {
          console.error('[Webhook] Error calling mint-token API:', mintErr);
        }
      } else {
        console.warn('[Webhook] No wallet_address found in payment metadata');
      }
    } else {
      console.log(`[Webhook] Event ${event.type} received, no action taken.`);
    }

    // Always return 200 to acknowledge receipt
    return NextResponse.json({ status: 'ok' });

  } catch (error) {
    console.error('[Webhook] Processing error:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
