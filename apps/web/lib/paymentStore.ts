export interface PaymentState {
  walletAddress: string;
  aadhaarHash: string;
  panHash: string;
  status: 'pending' | 'processing' | 'complete' | 'failed' | 'refunded';
  proofHash?: string;
  publicValues?: string;
  txSignature?: string;
  tokenAddress?: string;
}

// Module-level map for hackathon purposes
export const paymentStore = new Map<string, PaymentState>();
