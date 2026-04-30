import { NextResponse } from 'next/server';
import path from 'path';
import fs from 'fs';
import { v4 as uuidv4 } from 'uuid';
// @ts-ignore
import * as snarkjs from 'snarkjs';

const PROOF_LOG_PATH = path.join(process.cwd(), 'proof_log.json');

// Initialize log if it doesn't exist
function initLog() {
  if (!fs.existsSync(PROOF_LOG_PATH)) {
    fs.writeFileSync(PROOF_LOG_PATH, JSON.stringify({ proofsGenerated: 0, walletsVerified: 0, tokensMinted: 0, logs: [] }));
  }
}

// In-Memory Cache for Verification Key to optimize verification speed
let cachedVKey: any = null;

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { age, panHash, kycScore, walletAddress } = body;

    if (!age || !panHash || !kycScore || !walletAddress) {
      return NextResponse.json({ error: 'Missing required inputs' }, { status: 400 });
    }

    // Ensure panHash is a valid BigInt string (add 0x if missing for hex)
    let formattedPanHash = panHash;
    if (typeof panHash === 'string' && /^[0-9a-fA-F]+$/.test(panHash)) {
      formattedPanHash = `0x${panHash}`;
    }
    
    // We expect the wasm and zkey to be in public/circuits which we copied in setup.sh
    const wasmPath = path.join(process.cwd(), 'public', 'circuits', 'compliance.wasm');
    const zkeyPath = path.join(process.cwd(), 'public', 'circuits', 'compliance_final.zkey');
    const vkeyPath = path.join(process.cwd(), 'public', 'circuits', 'verification_key.json');

    // The template has public inputs [minAge, minKycScore] and private inputs [age, panHash, kycScore].
    // minAge = 18, minKycScore = 700 are hardcoded here or passed as constants.
    const minAge = 18;
    const minKycScore = 700;

    const input = {
      age,
      panHash: formattedPanHash,
      kycScore,
      minAge,
      minKycScore
    };

    const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmPath, zkeyPath);

    // Optimized Verification: Use cached vKey if available to save I/O overhead
    if (!cachedVKey) {
      cachedVKey = JSON.parse(fs.readFileSync(vkeyPath, 'utf-8'));
    }
    const isValid = await snarkjs.groth16.verify(cachedVKey, publicSignals, proof);

    if (!isValid) {
      return NextResponse.json({ error: 'Proof generated but failed local verification.' }, { status: 400 });
    }

    // Save to proof log
    initLog();
    const logData = JSON.parse(fs.readFileSync(PROOF_LOG_PATH, 'utf-8'));
    
    const proofId = uuidv4();
    const timestamp = Date.now();
    
    logData.proofsGenerated += 1;
    logData.logs.push({ proofId, walletAddress, timestamp, type: 'PROOF_GENERATED' });
    fs.writeFileSync(PROOF_LOG_PATH, JSON.stringify(logData, null, 2));

    return NextResponse.json({ 
      proof, 
      publicSignals, 
      valid: true, 
      proofId, 
      timestamp 
    });

  } catch (error: any) {
    console.error('Error generating proof:', error);
    return NextResponse.json({ error: error.message || 'Failed to generate proof' }, { status: 500 });
  }
}
