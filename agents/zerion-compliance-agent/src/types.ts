// Agent policy types — defines constraints on agent behavior

export interface AgentPolicy {
  name: string;
  
  // Chain lock — agent only acts on specified chains
  allowedChains: ("solana" | "ethereum" | "polygon")[];
  
  // Spend limit — max USD value per transaction
  maxSpendUsdPerTx: number;
  
  // Daily spend limit
  maxSpendUsdPerDay: number;
  
  // Expiry — policy expires at this timestamp
  expiresAt: Date;
  
  // Compliance gate — wallet must have valid Z-RWA proof
  requireZKCompliance: boolean;
  
  // Blocked actions
  blockedActions: ("swap" | "bridge" | "send")[];
  
  // Minimum proof freshness in seconds (reject proofs older than this)
  minProofFreshnessSeconds: number;
}

export interface ComplianceResult {
  compliant: boolean;
  walletAddress: string;
  proofHash: string | null;
  verifiedAt: string | null;
  expiresAt: string | null;
  network: string;
}

export interface AgentAction {
  type: "transfer" | "swap" | "bridge" | "skip" | "flag";
  walletAddress: string;
  amount?: number;
  token?: string;
  reason: string;
  complianceProofHash?: string;
  timestamp: Date;
}

export interface TransferRequest {
  id: string;
  fromWallet: string;
  toWallet: string;
  tokenMint: string;
  amount: number;
  requestedAt: Date;
  status: "pending" | "approved" | "rejected" | "executed";
}
