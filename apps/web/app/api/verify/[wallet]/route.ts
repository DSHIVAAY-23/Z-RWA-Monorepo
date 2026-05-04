import { NextRequest, NextResponse } from "next/server";
import { PublicKey } from "@solana/web3.js";
import { verifyWallet } from "../../../../lib/verify";

// ── Rate-limit store ────────────────────────────────────────────────────────
const RATE_LIMIT_MAX = 60;
const RATE_LIMIT_WINDOW_MS = 60_000;

interface RateLimitEntry {
  count: number;
  resetAt: number;
}
const rateLimitMap = new Map<string, RateLimitEntry>();

function checkRateLimit(ip: string): boolean {
  const now = Date.now();
  const entry = rateLimitMap.get(ip);

  if (!entry || now > entry.resetAt) {
    rateLimitMap.set(ip, { count: 1, resetAt: now + RATE_LIMIT_WINDOW_MS });
    return true;
  }

  if (entry.count >= RATE_LIMIT_MAX) {
    return false;
  }

  entry.count += 1;
  return true;
}

// ── CORS headers ─────────────────────────────────────────────────────────────
const CORS_HEADERS = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET, OPTIONS",
  "Content-Type": "application/json",
};

export async function OPTIONS() {
  return new NextResponse(null, { status: 204, headers: CORS_HEADERS });
}

export async function GET(
  request: NextRequest,
  { params }: { params: { wallet: string } }
) {
  // Rate limiting
  const ip =
    request.headers.get("x-forwarded-for")?.split(",")[0]?.trim() ??
    request.headers.get("x-real-ip") ??
    "unknown";

  if (!checkRateLimit(ip)) {
    return NextResponse.json(
      { error: "Rate limit exceeded. Max 60 requests per minute." },
      {
        status: 429,
        headers: { ...CORS_HEADERS, "Retry-After": "60" },
      }
    );
  }

  const { wallet } = params;

  // Validate wallet address
  try {
    const pk = new PublicKey(wallet);
    if (!PublicKey.isOnCurve(pk.toBytes())) {
      throw new Error("Not on curve");
    }
  } catch {
    return NextResponse.json(
      { error: "Invalid wallet address" },
      { status: 400, headers: CORS_HEADERS }
    );
  }

  try {
    const result = await verifyWallet(wallet);

    return NextResponse.json(result, {
      status: 200,
      headers: {
        ...CORS_HEADERS,
        "Cache-Control": "public, max-age=30",
      },
    });
  } catch (err: unknown) {
    const message = err instanceof Error ? err.message : "Unknown error";
    console.error("[verify] Error:", message);
    return NextResponse.json(
      { error: "Internal server error", details: message },
      { status: 500, headers: CORS_HEADERS }
    );
  }
}
