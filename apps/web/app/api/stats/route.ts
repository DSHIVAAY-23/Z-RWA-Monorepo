import { NextResponse } from "next/server";
import { getStats } from "../../../lib/statsStore";
import path from 'path';
import fs from 'fs';

const PROOF_LOG_PATH = path.join(process.cwd(), 'proof_log.json');

export async function GET() {
  try {
    // On Vercel: use in-memory store (file system is read-only)
    // On local: merge real log file data with in-memory store
    const baseStats = getStats();

    if (fs.existsSync(PROOF_LOG_PATH)) {
      const logData = JSON.parse(fs.readFileSync(PROOF_LOG_PATH, 'utf-8'));
      return NextResponse.json({
        proofs_generated: baseStats.proofs_generated + (logData.proofsGenerated || 0),
        wallets_verified: baseStats.wallets_verified + (logData.walletsVerified || 0),
        tokens_minted: baseStats.tokens_minted + (logData.tokensMinted || 0),
        last_updated: new Date().toISOString(),
      });
    }

    return NextResponse.json(baseStats);

  } catch (error) {
    console.error('Error reading stats:', error);
    return NextResponse.json(getStats());
  }
}

