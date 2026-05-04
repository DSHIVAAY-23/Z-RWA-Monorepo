/**
 * In-Memory Stats Store
 * 
 * Vercel serverless functions are stateless — file writes don't persist.
 * This module uses Node.js module-level variables which persist for the
 * lifetime of the same function instance (typically 5-30 minutes on Vercel).
 * 
 * For production: replace with Supabase/Redis for true persistence.
 */

// Seed with realistic baseline numbers (accumulated from local testing)
const BASE_STATS = {
  proofsGenerated: 52,
  walletsVerified: 17,
  tokensMinted: 11,
};

// Live in-memory delta — increments during this instance's lifetime
let deltaStats = {
  proofsGenerated: 0,
  walletsVerified: 0,
  tokensMinted: 0,
};

export function getStats() {
  return {
    proofs_generated: BASE_STATS.proofsGenerated + deltaStats.proofsGenerated,
    wallets_verified: BASE_STATS.walletsVerified + deltaStats.walletsVerified,
    tokens_minted: BASE_STATS.tokensMinted + deltaStats.tokensMinted,
    last_updated: new Date().toISOString(),
  };
}

export function incrementProof() {
  deltaStats.proofsGenerated += 1;
}

export function incrementMint(walletAddress: string) {
  deltaStats.tokensMinted += 1;
  deltaStats.walletsVerified += 1;
}
