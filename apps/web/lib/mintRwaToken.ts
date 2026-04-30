import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";

export async function mintRwaTokenAfterProof(
  connection: Connection,
  payer: Keypair,  // backend authority keypair
  recipient: PublicKey,
  mintAddress: PublicKey,
  tokenProgramId: PublicKey = TOKEN_2022_PROGRAM_ID
) {
  const ata = await getOrCreateAssociatedTokenAccount(
    connection, 
    payer, 
    mintAddress, 
    recipient, 
    false,
    undefined, 
    undefined, 
    tokenProgramId
  );

  const sig = await mintTo(
    connection, 
    payer, 
    mintAddress, 
    ata.address, 
    payer,
    1, // 1 RWA compliance token
    [], 
    undefined, 
    tokenProgramId
  );

  return { ata: ata.address.toBase58(), signature: sig };
}
