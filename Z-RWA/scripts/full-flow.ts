import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { 
    PublicKey, 
    ComputeBudgetProgram, 
} from "@solana/web3.js";
import { 
    getAssociatedTokenAddressSync, 
    TOKEN_2022_PROGRAM_ID 
} from "@solana/spl-token";
import { ZRwa } from "../target/types/z_rwa";

async function main() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.ZRwa as Program<ZRwa>;
    const payer = provider.wallet.publicKey;

    console.log("🚀 Starting Full-Flow Verification...");
    console.log("Payer:", payer.toBase58());

    // 1. New Mint (already created and authority transferred in previous steps)
    const mint = new PublicKey("FhuXW2JHUyTNFF8eXW1EYsfuWcx3RfzdXHuDPvN7A7Xc");
    console.log("Mint:", mint.toBase58());

    // 2. Generate Real-looking Mock Proof (260 bytes)
    const proof = Buffer.from("cafe".repeat(130), "hex");
    const publicValues = Buffer.from("deadbeef".repeat(8), "hex");

    // 3. Derive ATA
    const destination = getAssociatedTokenAddressSync(
        mint,
        payer,
        false,
        TOKEN_2022_PROGRAM_ID
    );

    console.log("Destination (ATA):", destination.toBase58());

    try {
        console.log("📡 Sending Transaction (verify_and_mint)...");
        const tx = await program.methods
            .verifyAndMint(proof, publicValues)
            .accounts({
                payer: payer,
                mint: mint,
                destination: destination,
            })
            .preInstructions([
                ComputeBudgetProgram.setComputeUnitLimit({ units: 800_000 })
            ])
            .rpc();

        console.log("✅ Success! Transaction Signature:", tx);
        console.log(`🔗 View on Explorer: https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    } catch (e) {
        console.error("❌ Transaction failed:", e);
    }
}

main();
