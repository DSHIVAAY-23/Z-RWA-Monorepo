import { NextResponse } from 'next/server';
import path from 'path';
import fs from 'fs';

const PROOF_LOG_PATH = path.join(process.cwd(), 'proof_log.json');

export async function GET() {
  try {
    if (!fs.existsSync(PROOF_LOG_PATH)) {
      return NextResponse.json({
        proofsGenerated: 0,
        walletsVerified: 0,
        tokensMinted: 0
      });
    }

    const logData = JSON.parse(fs.readFileSync(PROOF_LOG_PATH, 'utf-8'));
    
    return NextResponse.json({
      proofsGenerated: logData.proofsGenerated || 0,
      walletsVerified: logData.walletsVerified || 0,
      tokensMinted: logData.tokensMinted || 0,
    });
  } catch (error) {
    console.error('Error reading proof stats:', error);
    return NextResponse.json({ error: 'Failed to read stats' }, { status: 500 });
  }
}
