import { NextResponse } from 'next/server';
import { verifyWebhookSignature } from '../../../lib/dodo';
import { paymentStore } from '../../../lib/paymentStore';
import { generateAndSubmitProof } from '../../../lib/zkMintOrchestrator';

export async function POST(request: Request) {
  try {
    const signature = request.headers.get('webhook-signature') || ''; 
    const payload = await request.text();
    const webhookSecret = process.env.DODO_WEBHOOK_SECRET || 'your_webhook_secret_here';

    // 1. Verify standard webhook
    if (!verifyWebhookSignature(payload, signature, webhookSecret)) {
      console.warn("Invalid Dodo webhook signature");
      return NextResponse.json({ error: "Invalid signature" }, { status: 401 });
    }

    // 2. Parse Dodo Payment Webhook Event
    const event = JSON.parse(payload);
    const data = event.data || event; 
    const status = data?.status || event.status;
    const paymentId = data?.payment_id || data?.session_id || event.payment_id;

    if (!paymentId) {
      return NextResponse.json({ received: true, ignored: true, reason: 'no_payment_id' });
    }

    // 3. Status explicit check (must be successful)
    if (status !== 'succeeded' && status !== 'paid' && status !== 'complete') {
        console.log(`Ignoring payment webhook for ${paymentId} because status is ${status}`);
        return NextResponse.json({ received: true, ignored: true, reason: 'not_paid' });
    }

    // 4. Look up in Store
    const storedState = paymentStore.get(paymentId);
    if (!storedState) {
        console.log(`Payment ID not found in local store: ${paymentId}`);
        // Returning 404 per instructions, although webhooks usually expect 200 for 'we acknowledge we dont have it'
        return NextResponse.json({ error: "Not found" }, { status: 404 });
    }

    // 6/7. If valid state 
    if (storedState.status === 'processing' || storedState.status === 'complete') {
        return NextResponse.json({ received: true, ignored: true, reason: 'already_processed' });
    }

    // 8. Update map explicitly 
    storedState.status = 'processing';
    paymentStore.set(paymentId, storedState);

    console.log(`[Webhook] Payment ${paymentId} confirmed. Triggering background ZK Mint Orchestrator...`);

    // 7. Trigger async flow (MUST NOT BLOCK)
    setTimeout(() => {
        generateAndSubmitProof({
             paymentId,
             aadhaarHash: storedState.aadhaarHash,
             panHash: storedState.panHash,
             walletAddress: storedState.walletAddress
        }).catch(err => {
            console.error(`Background proof error for ${paymentId}`, err);
            const state = paymentStore.get(paymentId);
            if (state) {
                state.status = 'failed';
                paymentStore.set(paymentId, state);
            }
        });
    }, 0);

    // 9. Return 200 immediately
    return NextResponse.json({ received: true });

  } catch (error) {
     console.error("Webhook processing error:", error);
     return NextResponse.json({ error: "Internal Server Error" }, { status: 500 });
  }
}
