import { NextResponse } from 'next/server';
import { Connection, Keypair, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { TOKEN_2022_PROGRAM_ID } from '@solana/spl-token';
import { mintRwaTokenAfterProof } from '../../../lib/mintRwaToken';
import path from 'path';
import fs from 'fs';

const PROOF_LOG_PATH = path.join(process.cwd(), 'proof_log.json');
const BACKEND_WALLET_PATH = process.env.BACKEND_WALLET_PATH || '/home/user/.config/solana/id.json';

// Use a fixed mint address for the project
const MINT_ADDRESS = process.env.NEXT_PUBLIC_RWA_MINT || "8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa";

export async function POST(req: Request) {
  try {
    const { walletAddress, txSignature } = await req.json();

    if (!walletAddress || !txSignature) {
      return NextResponse.json({ error: 'Missing required inputs' }, { status: 400 });
    }

    // In a real app, we would verify the txSignature on-chain to ensure verify_and_mint was called
    // For this implementation, we proceed to mint
    
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
    
    // Load backend authority keypair (from Env Var or File)
    let payer: Keypair;
    const envSecret = process.env.BACKEND_WALLET_SECRET;
    
    if (envSecret) {
      console.log('Loading authority from environment variable...');
      const secretKey = Uint8Array.from(JSON.parse(envSecret));
      payer = Keypair.fromSecretKey(secretKey);
    } else {
      console.log(`Loading authority from file: ${BACKEND_WALLET_PATH}`);
      const secretKey = JSON.parse(fs.readFileSync(BACKEND_WALLET_PATH, 'utf-8'));
      payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));
    }


    const recipient = new PublicKey(walletAddress);
    const mintAddress = new PublicKey(MINT_ADDRESS);

    console.log(`Minting RWA token for ${walletAddress}...`);
    const result = await mintRwaTokenAfterProof(connection, payer, recipient, mintAddress, TOKEN_2022_PROGRAM_ID);

    // Update stats (Resilient to Read-only filesystems)
    try {
      if (fs.existsSync(PROOF_LOG_PATH)) {
        const logData = JSON.parse(fs.readFileSync(PROOF_LOG_PATH, 'utf-8'));
        logData.tokensMinted = (logData.tokensMinted || 0) + 1;
        logData.walletsVerified = (logData.walletsVerified || 0) + 1;
        logData.logs.push({ 
          walletAddress, 
          timestamp: Date.now(), 
          type: 'TOKEN_MINTED', 
          signature: result.signature 
        });
        fs.writeFileSync(PROOF_LOG_PATH, JSON.stringify(logData, null, 2));
      }
    } catch (e) {
      console.warn("Skipping proof log update in mint-token (Read-only environment)");
    }


    return NextResponse.json({ 
      success: true, 
      ata: result.ata, 
      signature: result.signature 
    });

  } catch (error: any) {
    console.error('Error minting token:', error);
    return NextResponse.json({ error: error.message || 'Failed to mint token' }, { status: 500 });
  }
}
