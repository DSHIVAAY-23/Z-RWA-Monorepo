import { NextResponse } from "next/server";

const DOCS = `Z-RWA Compliance Oracle
=======================
GET /api/verify/{wallet_address}

Returns compliance status for a Solana wallet address.
A compliant wallet has submitted a valid ZK proof of identity (SP1 Groth16)
via the Z-RWA protocol and holds a Token2022 compliance token on Solana Devnet.

Response schema:
{
  compliant: boolean,          // true if wallet holds a valid ZK compliance token
  wallet: string,              // the queried wallet address
  proof_hash: string | null,   // deterministic hash derived from on-chain ATA
  verified_at: string | null,  // ISO 8601 timestamp of verification check
  expires_at: string | null,   // ISO 8601 timestamp (30 days from verification)
  network: "devnet",
  standard: "Z-RWA-v1",
  message?: string             // present only when compliant is false
}

Usage examples:
  const res = await fetch('https://zrwa.vercel.app/api/verify/WALLET_ADDRESS');
  const { compliant, proof_hash } = await res.json();
  if (compliant) executeRWATrade({ proof_hash });

Rate limit: 60 requests per IP per minute.
CORS: Access-Control-Allow-Origin: * (open for AI agents)

Learn more: https://github.com/DSHIVAAY-23/Z-RWA-Monorepo
Built for Colosseum Frontier 2026 | Powered by SP1 · Solana · Token2022
`;

export async function GET() {
  return new NextResponse(DOCS, {
    status: 200,
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
      "Access-Control-Allow-Origin": "*",
      "Cache-Control": "public, max-age=3600",
    },
  });
}
