import { NextResponse } from 'next/server';
import path from 'path';
import fs from 'fs';
import { v4 as uuidv4 } from 'uuid';
// @ts-ignore
import * as snarkjs from 'snarkjs';
import { incrementProof } from '../../../lib/statsStore';


const PROOF_LOG_PATH = path.join(process.cwd(), 'proof_log.json');

// Initialize log if it doesn't exist
function initLog() {
  try {
    if (!fs.existsSync(PROOF_LOG_PATH)) {
      fs.writeFileSync(PROOF_LOG_PATH, JSON.stringify({ proofsGenerated: 0, walletsVerified: 0, tokensMinted: 0, logs: [] }));
    }
  } catch (e) {
    console.warn("Could not initialize local log (Read-only filesystem):", e);
  }
}


// In-Memory Cache for Verification Key to optimize verification speed
let cachedVKey: any = null;

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { age, panHash, kycScore, walletAddress, docType } = body;

    if (!age || !panHash || !kycScore || !walletAddress) {
      return NextResponse.json({ error: 'Missing required inputs' }, { status: 400 });
    }

    // ── Layer 3: API gate — reject unvalidated documents ──────────────────────
    if (!docType || docType === 'unknown') {
      return NextResponse.json(
        { error: 'Document type is unknown. Please upload a valid Aadhaar or PAN card.' },
        { status: 400 }
      );
    }

    if (typeof age !== 'number' || age < 18) {
      return NextResponse.json(
        { error: `Must be 18+ for compliance proof. Extracted age: ${age}` },
        { status: 400 }
      );
    }
    // ─────────────────────────────────────────────────────────────────────────

    // Ensure panHash is a valid BigInt string (add 0x if missing for hex)
    let formattedPanHash = panHash;
    if (typeof panHash === 'string' && /^[0-9a-fA-F]+$/.test(panHash)) {
      formattedPanHash = `0x${panHash}`;
    }
    
    // Resilient path resolution for Vercel (Fetch & Temp strategy)
    const getPath = async (file: string, reqUrl: string) => {
      // 1. Try local filesystem first
      const localPaths = [
        path.join(process.cwd(), 'apps', 'web', 'public', 'circuits', file),
        path.join(process.cwd(), 'public', 'circuits', file),
        path.join(process.cwd(), '.next', 'server', 'public', 'circuits', file),
      ];
      
      for (const p of localPaths) {
        if (fs.existsSync(p)) return p;
      }

      // 2. If on Vercel and local file not found, fetch from self and write to /tmp
      const tmpPath = path.join('/tmp', file);
      if (fs.existsSync(tmpPath)) return tmpPath;

      console.log(`Downloading ${file} to /tmp...`);
      const origin = new URL(reqUrl).origin;
      const response = await fetch(`${origin}/circuits/${file}`);
      if (!response.ok) throw new Error(`Failed to fetch ${file} from ${origin}`);
      
      const buffer = Buffer.from(await response.arrayBuffer());
      fs.writeFileSync(tmpPath, buffer);
      console.log(`✅ ${file} saved to /tmp`);
      return tmpPath;
    };

    let wasmPath, zkeyPath, vkeyPath;
    try {
      wasmPath = await getPath('compliance.wasm', req.url);
      zkeyPath = await getPath('compliance_final.zkey', req.url);
      vkeyPath = await getPath('verification_key.json', req.url);
      console.log("Paths resolved:", { wasmPath, zkeyPath, vkeyPath });
    } catch (err: any) {
      console.error("Environment Error:", err.message);
      return NextResponse.json({ 
        error: `Environment Error: ${err.message}`,
        stack: err.stack,
        cwd: process.cwd()
      }, { status: 500 });
    }





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

    // Set a timeout for the proof generation to prevent Vercel hanging
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmPath, zkeyPath);


    // Optimized Verification: Use cached vKey if available to save I/O overhead
    if (!cachedVKey) {
      cachedVKey = JSON.parse(fs.readFileSync(vkeyPath, 'utf-8'));
    }
    const isValid = await snarkjs.groth16.verify(cachedVKey, publicSignals, proof);

    if (!isValid) {
      return NextResponse.json({ error: 'Proof generated but failed local verification.' }, { status: 400 });
    }

    const proofId = uuidv4();
    const timestamp = Date.now();

    // Increment in-memory stats counter (works on Vercel)
    incrementProof();

    // Save to proof log (Resilient to Read-only filesystems)

    try {

      initLog();
      if (fs.existsSync(PROOF_LOG_PATH)) {
        const logData = JSON.parse(fs.readFileSync(PROOF_LOG_PATH, 'utf-8'));
        logData.proofsGenerated += 1;
        logData.logs.push({ proofId, walletAddress, timestamp, type: 'PROOF_GENERATED' });
        fs.writeFileSync(PROOF_LOG_PATH, JSON.stringify(logData, null, 2));
      }
    } catch (e) {
      console.warn("Skipping proof log update (Read-only environment)");
    }


    return NextResponse.json({ 
      proof, 
      publicSignals, 
      valid: true, 
      proofId, 
      timestamp 
    });

  } catch (error: any) {
    console.error('CRITICAL PROOF ERROR:', error);
    return NextResponse.json({ 
      error: error.message || 'Failed to generate proof',
      stack: error.stack,
      details: "This often happens on Vercel due to WASM or memory constraints. Check function logs."
    }, { status: 500 });
  }
}

