// Constants
export const Z_RWA_PROGRAM_ID = process.env.NEXT_PUBLIC_Z_RWA_PROGRAM_ID || "YOUR_PROGRAM_ID_HERE";
export const SOLANA_NETWORK = process.env.NEXT_PUBLIC_SOLANA_NETWORK || "devnet";
export const RPC_URL = process.env.NEXT_PUBLIC_RPC_URL || "https://api.devnet.solana.com";

/**
 * Submits the generated proof, public values, and document hash to the z-rwa Program
 */
export async function submitProof(proof: string, publicValues: string, docHash: string): Promise<{
  success: boolean;
  txHash: string;
  mintAddress: string;
}> {
  // In a real implementation this connects to Anchor program and sends the `verify_and_mint` instruction
  
  // For now use mock implementation that:
  // - Simulates 2s "submission" delay
  // - Returns mock TX hash
  // - Shows success state
  await new Promise(resolve => setTimeout(resolve, 2000));
  
  // Generate fake tx hash randomly for demo
  const mockTxId = Array.from(crypto.getRandomValues(new Uint8Array(32)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('') + Array.from(crypto.getRandomValues(new Uint8Array(32)))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');

  // Generate fake public key for Mint
  const mockMintPubkey = "Mint" + Array.from(crypto.getRandomValues(new Uint8Array(20)))
    .map(b => b.toString(36).padStart(2, '0'))
    .join('').substring(0, 30);

  return {
    success: true,
    txHash: mockTxId,
    mintAddress: mockMintPubkey
  };
}

/**
 * Checks if wallet has Z-RWA token
 */
export async function checkComplianceToken(walletAddress: string): Promise<boolean> {
  // Mock implementation
  return false;
}

/**
 * Returns Solana explorer devnet URL
 */
export function getExplorerUrl(txHash: string): string {
  if (SOLANA_NETWORK === 'mainnet-beta') {
    return `https://explorer.solana.com/tx/${txHash}`;
  }
  return `https://explorer.solana.com/tx/${txHash}?cluster=${SOLANA_NETWORK}`;
}
