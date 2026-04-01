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
import { expect } from "chai";

describe("z_rwa_full_flow", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.ZRwa as Program<ZRwa>;
    const payer = provider.wallet.publicKey;

    it("Performs a full verify_and_mint flow on Devnet", async () => {
        console.log("🚀 Starting Full-Flow Verification...");

        const mint = new PublicKey("FhuXW2JHUyTNFF8eXW1EYsfuWcx3RfzdXHuDPvN7A7Xc");
        const proof = Buffer.from("cafe".repeat(130), "hex");
        const publicValues = Buffer.from("deadbeef".repeat(8), "hex");

        const destination = getAssociatedTokenAddressSync(
            mint,
            payer,
            false,
            TOKEN_2022_PROGRAM_ID
        );

        console.log("📡 Sending Transaction (verify_and_mint)...");
        try {
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
            expect(tx).to.be.a("string");
        } catch (e: any) {
            console.error("❌ Transaction failed:", e);
            if (e.logs) console.log("Logs:", e.logs);
            throw e;
        }
    });
});
