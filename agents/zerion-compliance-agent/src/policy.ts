import { AgentPolicy, TransferRequest } from "./types";
import { checkZKCompliance } from "./compliance";

export const DEFAULT_RWA_POLICY: AgentPolicy = {
  name: "Z-RWA Compliance Policy v1",
  allowedChains: ["solana"],           // Chain locked to Solana only
  maxSpendUsdPerTx: 10000,             // $10k max per transaction
  maxSpendUsdPerDay: 50000,            // $50k daily limit
  expiresAt: new Date("2026-12-31"),   // Policy expiry
  requireZKCompliance: true,           // MUST have valid ZK proof
  blockedActions: ["bridge"],          // No bridging allowed
  minProofFreshnessSeconds: 30 * 24 * 60 * 60, // Proof must be < 30 days old
};

export type PolicyDecision = 
  | { approved: true; reason: string; proofHash: string }
  | { approved: false; reason: string; blockedBy: string };

export async function evaluatePolicy(
  request: TransferRequest,
  policy: AgentPolicy
): Promise<PolicyDecision> {
  
  // 1. Check policy expiry
  if (new Date() > policy.expiresAt) {
    return { 
      approved: false, 
      reason: "Agent policy has expired", 
      blockedBy: "POLICY_EXPIRED" 
    };
  }
  
  // 2. Check spend limit
  if (request.amount > policy.maxSpendUsdPerTx) {
    return {
      approved: false,
      reason: `Amount $${request.amount} exceeds per-tx limit $${policy.maxSpendUsdPerTx}`,
      blockedBy: "SPEND_LIMIT_EXCEEDED",
    };
  }
  
  // 3. Check ZK compliance (the core gate)
  if (policy.requireZKCompliance) {
    console.log(`Checking ZK compliance for ${request.fromWallet}...`);
    
    const compliance = await checkZKCompliance(request.fromWallet);
    
    if (!compliance.compliant) {
      return {
        approved: false,
        reason: `Wallet ${request.fromWallet} has no valid ZK compliance proof`,
        blockedBy: "ZK_COMPLIANCE_FAILED",
      };
    }
    
    // 4. Check proof freshness
    if (compliance.freshnessSeconds > policy.minProofFreshnessSeconds) {
      return {
        approved: false,
        reason: `ZK proof is ${Math.floor(compliance.freshnessSeconds / 86400)} days old — exceeds ${Math.floor(policy.minProofFreshnessSeconds / 86400)} day limit`,
        blockedBy: "PROOF_STALE",
      };
    }
    
    return {
      approved: true,
      reason: `ZK proof valid. Proof hash: ${compliance.proofHash?.slice(0, 16)}...`,
      proofHash: compliance.proofHash!,
    };
  }
  
  return {
    approved: true,
    reason: "All policy checks passed",
    proofHash: "",
  };
}
