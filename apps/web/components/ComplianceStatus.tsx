'use client';

import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { submitProof, getExplorerUrl } from '../lib/solana';

interface ComplianceStatusProps {
  proof: string | null;
  publicValues: string | null;
  docHash: string | null;
  isActive: boolean;
}

export default function ComplianceStatus({ proof, publicValues, docHash, isActive }: ComplianceStatusProps) {
  const { connected } = useWallet();
  const [status, setStatus] = useState<'pending' | 'verifying' | 'success' | 'error'>('pending');
  const [txHash, setTxHash] = useState<string>('');
  const [mintAddress, setMintAddress] = useState<string>('');

  const handleVerify = async () => {
    if (!proof || !docHash || !publicValues || !connected) return;

    setStatus('verifying');
    try {
      const result = await submitProof(proof, publicValues, docHash);
      if (result.success) {
        setTxHash(result.txHash);
        setMintAddress(result.mintAddress);
        setStatus('success');
      } else {
        setStatus('error');
      }
    } catch (e: any) {
      setStatus('error');
    }
  };

  return (
    <div className={`rounded-xl border p-6 transition-all duration-300 ${isActive || status === 'success' ? 'border-purple-500/50 bg-black/40' : 'border-white/8 bg-black/20 opacity-50'}`}>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className={`w-8 h-8 rounded-full text-sm font-bold flex items-center justify-center ${status === 'success' ? 'bg-accent-green text-black' : 'bg-gray-700 text-gray-400'}`}>
            {status === 'success' ? '✓' : '3'}
          </div>
          <span className="font-semibold text-white">
            🏆 Mint RWA Compliance Token
          </span>
        </div>
      </div>
      
      {!connected ? (
        <>
          <p className="text-gray-400 text-sm mb-4">
            Connect your Solana wallet to verify the ZK proof on-chain and receive your Token2022 asset.
          </p>
          <div className="w-full flex justify-center py-2">
            <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-500 !transition-colors !rounded-xl !w-full !justify-center !font-space" />
          </div>
        </>
      ) : status === 'pending' || status === 'error' ? (
        <>
          <p className="text-gray-400 text-sm mb-4">
            Proof verified by z-rwa program. Token2022 minted to your wallet upon success.
          </p>
          {status === 'error' && (
            <div className="text-red-400 text-sm mb-3 font-mono bg-red-400/10 p-2 rounded">Tx Failed. Try again.</div>
          )}
          <button 
            onClick={handleVerify}
            disabled={!isActive}
            className="w-full py-3 rounded-xl border-2 border-purple-500 text-purple-400 font-space font-semibold hover:bg-purple-500/10 transition-all shadow-[0_0_15px_rgba(124,58,237,0.15)] disabled:opacity-40 disabled:shadow-none"
          >
            Submit Proof & Mint Token →
          </button>
        </>
      ) : status === 'verifying' ? (
        <div className="py-8 flex flex-col items-center justify-center rounded-xl bg-purple-500/5 border border-purple-500/20">
          <div className="w-6 h-6 border-2 border-purple-500 border-t-transparent rounded-full animate-spin mb-3"></div>
          <p className="text-purple-400 font-mono text-sm">Awaiting Devnet Confirmation...</p>
        </div>
      ) : (
        <div className="p-5 rounded-xl border border-accent-green/30 bg-accent-green/5">
          <div className="flex items-center gap-2 mb-4 text-accent-green">
            <span className="text-xl">🏆</span>
            <span className="font-semibold font-space">Token2022 Minted successfully</span>
          </div>
          
          <div className="space-y-2 text-sm font-mono text-gray-400">
            <div className="flex justify-between border-b border-white/5 pb-2">
              <span>Token:</span> <span className="text-white">Z-RWA-COMPLY</span>
            </div>
            <div className="flex justify-between border-b border-white/5 pb-2">
              <span>Standard:</span> <span className="text-white">Token2022</span>
            </div>
            <div className="flex justify-between border-b border-white/5 pb-2">
              <span>Mint:</span> <span className="text-purple-400 truncate w-32 text-right">{mintAddress}</span>
            </div>
          </div>
          
          <a href={getExplorerUrl(txHash)} target="_blank" rel="noreferrer" className="mt-4 block w-full text-center py-2 bg-white/5 hover:bg-white/10 text-white rounded-lg transition-colors text-sm font-mono border border-white/10">
            View on Solscan ↗
          </a>
        </div>
      )}
    </div>
  );
}
