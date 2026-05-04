"use client";

import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import ZNavbar from '../components/ZNavbar';

export default function InvestPage() {
  const { publicKey } = useWallet();
  const [amount, setAmount] = useState<number>(1000);
  const [walletAddress, setWalletAddress] = useState<string>('');
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (publicKey && !walletAddress) {
      setWalletAddress(publicKey.toBase58());
    }
  }, [publicKey, walletAddress]);

  const handlePay = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch('/api/create-payment', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          amount_inr: amount,
          wallet_address: walletAddress,
        }),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.error || 'Failed to initialize payment');
      }

      if (data.checkoutUrl) {
        window.location.href = data.checkoutUrl;
      } else {
        throw new Error('No checkout URL returned from server');
      }
    } catch (err: any) {
      console.error('Payment Error:', err);
      setError(err.message || 'An unexpected error occurred. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-black text-white font-sans selection:bg-purple-900/30">
      <ZNavbar />

      <main className="max-w-7xl mx-auto px-6 py-20 flex flex-col items-center">
        {/* Animated Glow */}
        <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-purple-600/10 blur-[120px] pointer-events-none rounded-full" />

        <div className="relative z-10 w-full max-w-lg">
          {/* Invest Card */}
          <div className="bg-gray-900/40 backdrop-blur-2xl border border-gray-800 rounded-3xl p-8 md:p-10 shadow-2xl relative overflow-hidden">
            {/* Top accent */}
            <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-transparent via-purple-500 to-transparent opacity-50" />

            <div className="text-center mb-10">
              <h1 className="text-3xl font-extrabold tracking-tight mb-3">
                Invest in Indian RWA — <span className="text-purple-400">Pay with INR</span>
              </h1>
              <p className="text-gray-400 text-sm">
                Securely invest in real-world assets using UPI, Card, or Netbanking. 
                Identity stays private via ZK compliance.
              </p>
            </div>

            <form onSubmit={handlePay} className="space-y-6">
              {/* Amount Input */}
              <div>
                <label className="block text-xs font-mono font-bold text-gray-500 uppercase tracking-widest mb-2 ml-1">
                  Investment Amount (INR)
                </label>
                <div className="relative group">
                  <div className="absolute inset-y-0 left-4 flex items-center pointer-events-none text-xl text-gray-500 font-light">
                    ₹
                  </div>
                  <input
                    type="number"
                    min={1000}
                    max={1000000}
                    required
                    value={amount}
                    onChange={(e) => setAmount(Number(e.target.value))}
                    className="w-full bg-black/60 border border-gray-800 rounded-2xl pl-10 pr-6 py-4 text-2xl font-light focus:outline-none focus:ring-2 focus:ring-purple-500/40 focus:border-purple-500/50 transition-all placeholder-gray-700"
                    placeholder="1,000"
                  />
                  <div className="absolute right-4 top-1/2 -translate-y-1/2 text-[10px] font-mono text-gray-600 bg-gray-900/80 px-2 py-1 rounded border border-gray-800">
                    MIN ₹1,000
                  </div>
                </div>
              </div>

              {/* Wallet Input */}
              <div>
                <label className="block text-xs font-mono font-bold text-gray-500 uppercase tracking-widest mb-2 ml-1">
                  Solana Receiving Wallet
                </label>
                <input
                  type="text"
                  required
                  value={walletAddress}
                  onChange={(e) => setWalletAddress(e.target.value)}
                  className="w-full bg-black/60 border border-gray-800 rounded-2xl px-6 py-4 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-purple-500/40 focus:border-purple-500/50 transition-all placeholder-gray-700"
                  placeholder="Enter Solana wallet address"
                />
              </div>

              {/* Summary */}
              <div className="bg-purple-900/10 border border-purple-500/20 rounded-2xl p-4 flex justify-between items-center">
                <span className="text-sm text-purple-300">Estimated Tokens</span>
                <span className="text-xl font-bold text-white">{(amount / 100).toFixed(2)} RWA</span>
              </div>

              {/* CTA Button */}
              <button
                type="submit"
                disabled={isLoading}
                className={`w-full group relative flex items-center justify-center gap-2 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white font-bold py-5 px-8 rounded-2xl transition-all shadow-[0_10px_40px_-10px_rgba(139,92,246,0.5)] transform active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed`}
              >
                {isLoading ? (
                  <>
                    <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Initializing...
                  </>
                ) : (
                  <>
                    Pay with Dodo →
                    <div className="absolute inset-0 rounded-2xl bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity" />
                  </>
                )}
              </button>

              {/* Error Message */}
              {error && (
                <div className="bg-red-500/10 border border-red-500/30 text-red-400 text-xs p-4 rounded-xl text-center">
                  {error}
                </div>
              )}
            </form>

            {/* Support/Footer */}
            <div className="mt-8 pt-6 border-t border-gray-800/50 flex justify-center gap-4 text-[10px] font-mono text-gray-600">
              <span className="flex items-center gap-1">🔒 SSL SECURED</span>
              <span className="flex items-center gap-1">⚡ UPI SUPPORTED</span>
              <span className="flex items-center gap-1">✅ INSTANT MINT</span>
            </div>
          </div>

          <div className="mt-8 text-center">
            <p className="text-gray-500 text-xs">
              Powered by <span className="text-white font-bold">Dodo Payments</span> for secure INR processing.
              By continuing, you agree to the investment terms.
            </p>
          </div>
        </div>
      </main>

      <style jsx>{`
        .bg-grid {
          background-image: radial-gradient(circle at 1px 1px, #1a1a1a 1px, transparent 0);
          background-size: 40px 40px;
        }
      `}</style>
    </div>
  );
}
