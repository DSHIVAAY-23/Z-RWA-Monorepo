import { 
    Connection, 
    PublicKey, 
    Transaction, 
    SystemProgram, 
    ComputeBudgetProgram,
    clusterApiUrl
} from "@solana/web3.js";
import { AnchorProvider, Program, Idl } from "@coral-xyz/anchor";
import { 
    getAssociatedTokenAddressSync, 
    createAssociatedTokenAccountInstruction, 
    TOKEN_2022_PROGRAM_ID 
} from "@solana/spl-token";
import { Buffer } from "buffer";
import idl from "./idl/z_rwa.json";
import { ZRwa } from "./idl/z_rwa";

export const SOLANA_NETWORK = "devnet";
export const RPC_URL = clusterApiUrl("devnet");

export const PROGRAM_ID = new PublicKey(idl.address);
const Z_RWA_PROGRAM_ID = new PublicKey("3SN3zAmuW5HWgJy5mcWjvy8vwDZRLosEajqydbuxiEZC");

// Token Settings
export const RWA_MINT = new PublicKey(
  process.env.NEXT_PUBLIC_RWA_MINT || "FhuXW2JHUyTNFF8eXW1EYsfuWcx3RfzdXHuDPvN7A7Xc"
);

export interface SubmitProofResult {
    success: boolean;
    txHash: string;
    mintAddress: string;
}

/**
 * Submits the generated proof, public values, and document hash to the z-rwa Program
 */
export async function submitProof(
    wallet: any,
    proof: string, 
    publicValues: string, 
    docHash: string
): Promise<SubmitProofResult> {
    if (!wallet.publicKey) throw new Error("Wallet not connected");

    try {
        const connection = new Connection(RPC_URL, "confirmed");
        const provider = new AnchorProvider(connection, wallet, {
            commitment: "confirmed",
        });
        const program = new Program(idl as Idl, provider) as unknown as Program<ZRwa>;

        const payer = wallet.publicKey;
        const proofBuffer = Buffer.from(proof.replace('0x', ''), 'hex');
        const publicValuesBuffer = Buffer.from(publicValues.replace('0x', ''), 'hex');

        const destination = getAssociatedTokenAddressSync(
            RWA_MINT,
            payer,
            false,
            TOKEN_2022_PROGRAM_ID
        );

        // Derive PDA
        const [mintAuthority] = PublicKey.findProgramAddressSync(
            [Buffer.from("mint_authority")],
            Z_RWA_PROGRAM_ID
        );

        const destinationInfo = await connection.getAccountInfo(destination);
        const preInstructions = [
            ComputeBudgetProgram.setComputeUnitLimit({ units: 800_000 })
        ];

        if (!destinationInfo) {
            preInstructions.push(
                createAssociatedTokenAccountInstruction(
                    payer,
                    destination,
                    payer,
                    RWA_MINT,
                    TOKEN_2022_PROGRAM_ID
                )
            );
        }

        console.log("Submitting proof to Solana Devnet...");
        
        // Execute RPC call with explicit accounts
        const tx = await program.methods
            .verifyAndMint(proofBuffer, publicValuesBuffer)
            .accounts({
                payer: payer,
                mint: RWA_MINT,
                destination: destination,
                mintAuthority: mintAuthority,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            } as any)
            .preInstructions(preInstructions)
            .rpc();

        return {
            success: true,
            txHash: tx,
            mintAddress: RWA_MINT.toBase58()
        };
    } catch (error: any) {
        console.error("Verification/Minting failed:", error);
        throw error;
    }
}

/**
 * Checks if wallet has Z-RWA token
 */
export async function checkComplianceToken(walletAddress: string): Promise<boolean> {
  // Real implementation for Token2022
  try {
      const connection = new Connection(RPC_URL, "confirmed");
      const pubkey = new PublicKey(walletAddress);
      const ata = getAssociatedTokenAddressSync(
          RWA_MINT,
          pubkey,
          false,
          TOKEN_2022_PROGRAM_ID
      );
      const balance = await connection.getTokenAccountBalance(ata);
      return parseFloat(balance.value.amount) > 0;
  } catch (e) {
      return false;
  }
}

/**
 * Triggers a message signing request to verify wallet binding
 */
export async function signVerificationMessage(
    wallet: any,
    message: string
): Promise<string> {
    if (!wallet.publicKey) throw new Error("Wallet not connected");
    if (!wallet.signMessage) throw new Error("Wallet does not support message signing");

    const encodedMessage = new TextEncoder().encode(message);
    const signature = await wallet.signMessage(encodedMessage);
    
    // Convert Uint8Array signature to hex
    return Buffer.from(signature).toString('hex');
}

/**
 * Returns Solana explorer devnet URL
 */
export function getExplorerUrl(txHash: string): string {
    return `https://explorer.solana.com/tx/${txHash}?cluster=devnet`;
}
