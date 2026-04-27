import { NextRequest, NextResponse } from "next/server";
import { PublicKey } from "@solana/web3.js";
import { verifyWallet } from "../../../../lib/verify";

const CORS_SVG = {
  "Content-Type": "image/svg+xml",
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=300",
};

function buildBadgeSVG(compliant: boolean): string {
  if (compliant) {
    return `<svg width="200" height="28" xmlns="http://www.w3.org/2000/svg">
  <rect width="120" height="28" rx="4" fill="#1a1a2e"/>
  <rect x="120" width="80" height="28" rx="4" fill="#00c4a0"/>
  <text x="8" y="18" font-family="monospace" font-size="11" fill="#fff">Z-RWA</text>
  <text x="128" y="18" font-family="monospace" font-size="11" fill="#000" font-weight="bold">&#x2713; verified</text>
</svg>`;
  } else {
    return `<svg width="200" height="28" xmlns="http://www.w3.org/2000/svg">
  <rect width="120" height="28" rx="4" fill="#1a1a2e"/>
  <rect x="120" width="80" height="28" rx="4" fill="#666"/>
  <text x="8" y="18" font-family="monospace" font-size="11" fill="#fff">Z-RWA</text>
  <text x="128" y="18" font-family="monospace" font-size="11" fill="#fff" font-weight="bold">&#x2717; unverified</text>
</svg>`;
  }
}

export async function GET(
  _request: NextRequest,
  { params }: { params: { wallet: string } }
) {
  const { wallet } = params;

  // Invalid address → gray badge (graceful degradation)
  try {
    new PublicKey(wallet);
  } catch {
    return new NextResponse(buildBadgeSVG(false), { headers: CORS_SVG });
  }

  try {
    const result = await verifyWallet(wallet);
    return new NextResponse(buildBadgeSVG(result.compliant), {
      headers: CORS_SVG,
    });
  } catch {
    return new NextResponse(buildBadgeSVG(false), { headers: CORS_SVG });
  }
}
