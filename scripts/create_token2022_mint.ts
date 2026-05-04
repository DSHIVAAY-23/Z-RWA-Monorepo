import {
  createMint,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { Connection, Keypair, clusterApiUrl } from "@solana/web3.js";
import fs from "fs";
import path from "path";

async function main() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  
  const walletPath = path.join(process.env.HOME || "", ".config/solana/id.json");
  const secretKey = JSON.parse(fs.readFileSync(walletPath, "utf-8"));
  const payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));

  console.log("Creating Token2022 Mint...");
  
  const mint = await createMint(
    connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    0,
    undefined,
    undefined,
    TOKEN_2022_PROGRAM_ID
  );

  console.log("Token2022 Mint Created:", mint.toBase58());
}

main().catch(console.error);
