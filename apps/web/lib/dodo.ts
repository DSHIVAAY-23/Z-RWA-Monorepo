import DodoPayments from 'dodopayments';
import { Webhook } from 'standardwebhooks';

const dodo = new DodoPayments({
  bearerToken: process.env.DODO_API_KEY || 'test_key',
  environment: 'test_mode',
});

export async function createCheckoutSession(params: {
  amount: number;        // in INR paise (smallest unit)
  currency: "INR";
  customerId?: string;
  metadata: {
    aadhaar_hash: string;
    pan_hash: string;
    wallet_address: string;
  };
  redirectUrl: string;
  webhookUrl: string;
}): Promise<{ checkoutUrl: string; paymentId: string }> {
  try {
    const productId = process.env.NEXT_PUBLIC_DODO_PRODUCT_ID;
    if (!productId || productId === 'your_product_id_here') {
       throw new Error("NEXT_PUBLIC_DODO_PRODUCT_ID is not configured.");
    }

    // Dodo Checkout Sessions Create using SDK v2 structure
    const checkoutSession = await dodo.checkoutSessions.create({
      product_cart: [
        {
          product_id: productId,
          quantity: 1,
          amount: params.amount, // Represented in lowest denomination (paise)
        },
      ],
      return_url: params.redirectUrl,
      metadata: params.metadata,
    });
     
    return {
      checkoutUrl: (checkoutSession as any).checkout_url || `${params.redirectUrl}&paymentId=${checkoutSession.session_id}`,
      paymentId: (checkoutSession as any).payment_id || checkoutSession.session_id
    };

  } catch (error: any) {
    console.error("[Dodo] API Error Details:", {
      message: error.message,
      status: error.status,
      data: error.data,
    });
    throw error;
  }
}

export function verifyWebhookSignature(payload: string, signature: string, secret: string): boolean {
    if (!signature || !secret) {
       console.error("[Webhook] Missing signature or secret");
       return false;
    }
    try {
        const wh = new Webhook(secret);
        wh.verify(payload, { 'webhook-signature': signature } as any);
        return true;
    } catch (err: any) {
        console.error("[Webhook] Verification failed:", err.message);
        return false;
    }
}
