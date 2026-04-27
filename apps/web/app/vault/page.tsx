"use client";
import { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { getOptimalPriorityFee } from "../../lib/priorityFee";

export default function VaultPage() {
  const { publicKey, signTransaction } = useWallet();
  const [depositAmount, setDepositAmount] = useState("");
  const [isGeneratingProof, setIsGeneratingProof] = useState(false);
  const [txHash, setTxHash] = useState("");

  const vaultStats = {
    tvl: 125000,
    apy: 8.2,
    userBalance: publicKey ? 500 : 0,
    complianceRate: 100,
  };

  async function handleDeposit() {
    if (!publicKey) return alert("Connect wallet first");
    
    setIsGeneratingProof(true);
    setTxHash("");
    
    try {
      // Task 4: Generate ZK proof using existing SP1 endpoint
      console.log("Requesting SP1 Compliance Proof...");
      const proofRes = await fetch("/api/prove", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          docType: "Aadhaar",
          docHash: "demo_user_hash_999",
          query: "age>=18;nationality=IN;income>=threshold"
        })
      });
      
      if (!proofRes.ok) throw new Error("Proof generation failed");
      const { proof, publicValues } = await proofRes.json();
      console.log("Proof generated natively:", proof.substring(0, 20) + "...");

      // Task 5: Get QN Priority Fee
      const priorityFee = await getOptimalPriorityFee();
      console.log(`Using Quicknode Priority Fee: ${priorityFee} micro-lamports`);

      // TODO: Submit deposit transaction via Anchor
      // Since Anchor build is deferred, we simulate the deposit submission
      console.log("Constructing Anchor CPI Transaction with fee:", priorityFee);
      
      // Simulate Tx Delay
      await new Promise(r => setTimeout(r, 2000));
      
      // We would use ComputeBudgetProgram.setComputeUnitPrice({ microLamports: priorityFee })
      // followed by await program.methods.depositWithProof(...)

      setTxHash("3J9z...dummy...tx_hash");
      alert(`Successfully deposited ${depositAmount} USDC into Kamino Compliance Vault!`);
      setDepositAmount("");

    } catch (error) {
      console.error(error);
      alert("Deposit failed: " + error);
    } finally {
      setIsGeneratingProof(false);
    }
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900 p-6">
      <div className="max-w-6xl mx-auto mb-8 flex justify-between items-center pt-8">
        <h1 className="text-3xl font-bold text-white font-space">
          Z-RWA Compliance Vault
        </h1>
        <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700 transition-colors" />
      </div>

      <div className="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
        <StatCard label="Total Value Locked" value={`$${vaultStats.tvl.toLocaleString()}`} />
        <StatCard label="Current APY" value={`${vaultStats.apy}%`} />
        <StatCard label="Your Balance" value={`$${vaultStats.userBalance}`} />
        <StatCard label="Compliance Rate" value={`${vaultStats.complianceRate}%`} />
      </div>

      <div className="max-w-2xl mx-auto bg-gray-800/50 backdrop-blur border border-gray-700 rounded-xl p-6">
        <h2 className="text-xl font-semibold text-white mb-4">Deposit to Kamino Earn</h2>
        
        <input
          type="number"
          value={depositAmount}
          onChange={(e) => setDepositAmount(e.target.value)}
          placeholder="Amount in USDC"
          className="w-full bg-gray-900 border border-gray-700 rounded-lg px-4 py-3 text-white mb-4"
        />

        <button
          onClick={handleDeposit}
          disabled={!publicKey || isGeneratingProof || !depositAmount}
          className="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white font-semibold py-3 rounded-lg transition"
        >
          {isGeneratingProof ? "Generating SP1 ZK Proof & Estimating QN Fees..." : "Deposit with Compliance Proof"}
        </button>

        {txHash && (
          <p className="text-green-400 text-center text-sm font-mono mt-4">
            Tx Success: {txHash}
          </p>
        )}

        <p className="text-sm text-gray-400 mt-4 text-center">
          ✓ Your identity data stays private via zero-knowledge proofs
        </p>
      </div>
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="bg-gray-800/50 backdrop-blur border border-gray-700 rounded-xl p-4">
      <p className="text-sm text-gray-400 mb-1">{label}</p>
      <p className="text-2xl font-bold text-white">{value}</p>
    </div>
  );
}
