import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";
import {
  getAssociatedTokenAddressSync,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

const RPC_URL =
  process.env.RPC_ENDPOINT ||
  process.env.NEXT_PUBLIC_RPC_URL ||
  clusterApiUrl("devnet");

const RWA_MINT = new PublicKey(
  process.env.NEXT_PUBLIC_RWA_MINT || "FhuXW2JHUyTNFF8eXW1EYsfuWcx3RfzdXHuDPvN7A7Xc"
);

export interface VerifyResult {
  compliant: boolean;
  wallet: string;
  proof_hash: string | null;
  verified_at: string | null;
  expires_at: string | null;
  network: string;
  standard: string;
  message?: string;
}

/**
 * Core compliance check — reused by /api/verify and /api/badge
 * Checks whether a wallet holds a Z-RWA Token2022 compliance token.
 */
export async function verifyWallet(walletAddress: string): Promise<VerifyResult> {
  const pubkey = new PublicKey(walletAddress);
  const connection = new Connection(RPC_URL, "confirmed");

  const ata = getAssociatedTokenAddressSync(
    RWA_MINT,
    pubkey,
    false,
    TOKEN_2022_PROGRAM_ID
  );

  let balance = 0;
  try {
    const tokenBalance = await connection.getTokenAccountBalance(ata);
    balance = parseFloat(tokenBalance.value.amount);
  } catch {
    balance = 0;
  }

  if (balance > 0) {
    const proofHashBytes = ata.toBytes();
    const proofHash =
      "0x" +
      Array.from(proofHashBytes)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");

    const now = new Date();
    const verifiedAt = now.toISOString();
    const expiresAt = new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000).toISOString();

    return {
      compliant: true,
      wallet: walletAddress,
      proof_hash: proofHash,
      verified_at: verifiedAt,
      expires_at: expiresAt,
      network: "devnet",
      standard: "Z-RWA-v1",
    };
  } else {
    return {
      compliant: false,
      wallet: walletAddress,
      proof_hash: null,
      verified_at: null,
      expires_at: null,
      network: "devnet",
      standard: "Z-RWA-v1",
      message:
        "Wallet has no valid compliance proof. Visit https://z-rwa.vercel.app/check to get verified.",
    };
  }
}
