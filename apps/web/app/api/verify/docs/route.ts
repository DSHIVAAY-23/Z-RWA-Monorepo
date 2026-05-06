import { NextResponse } from "next/server";

export async function GET() {
  return NextResponse.json(
    {
      endpoint: "/api/verify",
      method: "POST",
      description: "Verify a Groth16 ZK compliance proof on Solana devnet",
      body: {
        proof: "object — snarkjs Groth16 proof",
        publicSignals: "array — public inputs to the circuit",
        walletAddress: "string — Solana wallet public key",
      },
      response: {
        verified: "boolean",
        txSignature: "string — Solana transaction signature if verified",
        mintAddress: "string — Token2022 mint address if minted",
      },
      example_curl:
        "curl -X POST https://z-rwa-monorepo.vercel.app/api/verify -H 'Content-Type: application/json' -d '{\"proof\":{},\"publicSignals\":[],\"walletAddress\":\"YOUR_WALLET\"}'",
      also_see: "GET /api/verify/{walletAddress} — check compliance status for any wallet",
    },
    {
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Cache-Control": "public, max-age=3600",
      },
    }
  );
}
