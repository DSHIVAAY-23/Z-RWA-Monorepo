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
    if (event.type === 'payment.succeeded') {
      const paymentData = event.data;
      const metadata = paymentData?.metadata || {};
      const walletAddress = metadata.wallet_address;
      const paymentId = paymentData.payment_id;

      if (walletAddress && paymentId) {
        console.log(`[Webhook] Payment ${paymentId} succeeded for ${walletAddress}. Triggering ZK flow...`);
        
        // 3. Update state to 'processing' for the UI poller
        const { paymentStore } = await import('../../../lib/paymentStore');
        const state = paymentStore.get(paymentId);
        if (state) {
          state.status = 'processing';
          paymentStore.set(paymentId, state);
        }

        // 4. Trigger ZK Orchestrator (Background-ish)
        // Note: In a real prod environment, this should be a background job.
        // For the hackathon, we trigger it here.
        const { generateAndSubmitProof } = await import('../../../lib/zkMintOrchestrator');
        
        // We don't 'await' here to avoid webhook timeout, but Vercel might kill the process
        // unless we use edge functions or a queue. For the demo, we'll try to let it run.
        generateAndSubmitProof({
          aadhaarHash: metadata.aadhaar_hash || '00000000',
          panHash: metadata.pan_hash || '00000000',
          walletAddress: walletAddress,
          paymentId: paymentId
        }).then(result => {
          console.log(`[Webhook] ZK Flow completed for ${paymentId}:`, result.txSignature);
        }).catch(err => {
          console.error(`[Webhook] ZK Flow failed for ${paymentId}:`, err);
          if (state) {
            state.status = 'failed';
            paymentStore.set(paymentId, state);
          }
        });
      } else {
        console.warn('[Webhook] No wallet_address or payment_id found');
      }
    }

    // Always return 200 to acknowledge receipt
    return NextResponse.json({ status: 'ok' });

  } catch (error) {
    console.error('[Webhook] Processing error:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
