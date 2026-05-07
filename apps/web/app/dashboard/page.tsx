"use client";

import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import ZNavbar from '../../components/ZNavbar';

export default function DashboardPage() {
  const { publicKey } = useWallet();
  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (publicKey) {
      setLoading(true);
      fetch(`/api/dashboard?wallet=${publicKey.toBase58()}`)
        .then(res => res.json())
        .then(resData => {
          setData(resData);
          setLoading(false);
        })
        .catch(err => {
          console.error(err);
          setLoading(false);
        });
    } else {
      setData(null);
    }
  }, [publicKey]);

  return (
    <div className="min-h-screen bg-black text-white font-sans selection:bg-purple-900/30">
      <ZNavbar />

      <main className="max-w-7xl mx-auto px-6 py-20">
        <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-teal-600/10 blur-[120px] pointer-events-none rounded-full" />

        <div className="relative z-10 w-full">
          <div className="text-center mb-12">
            <h1 className="text-4xl font-extrabold tracking-tight mb-3">
              Investor <span className="text-teal-400">Dashboard</span>
            </h1>
            <p className="text-gray-400">
              Track your generated ZK proofs, RWA tokens, and fiat payments.
            </p>
          </div>

          {!publicKey ? (
            <div className="bg-gray-900/40 border border-gray-800 rounded-3xl p-12 text-center max-w-lg mx-auto backdrop-blur-md">
              <div className="text-4xl mb-4">🔐</div>
              <h2 className="text-xl font-bold mb-2">Connect Wallet</h2>
              <p className="text-gray-400 text-sm">Please connect your Solana wallet to view your profile.</p>
            </div>
          ) : loading ? (
            <div className="text-center py-20 text-teal-400 animate-pulse">
              Loading profile data...
            </div>
          ) : (
            <div className="space-y-8 max-w-5xl mx-auto">
              {/* Top Stats */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="bg-gray-900/40 border border-gray-800 rounded-2xl p-6 backdrop-blur-md text-center">
                  <div className="text-teal-400 text-sm font-bold tracking-widest font-mono mb-2 uppercase">Proofs Generated</div>
                  <div className="text-4xl font-extrabold">{data?.stats?.proofs_generated ?? 0}</div>
                </div>
                <div className="bg-gray-900/40 border border-purple-500/30 rounded-2xl p-6 backdrop-blur-md text-center shadow-[0_0_30px_rgba(168,85,247,0.1)]">
                  <div className="text-purple-400 text-sm font-bold tracking-widest font-mono mb-2 uppercase">Tokens Owned</div>
                  <div className="text-4xl font-extrabold">{data?.stats?.tokens_owned ?? 0}</div>
                </div>
                <div className="bg-gray-900/40 border border-gray-800 rounded-2xl p-6 backdrop-blur-md text-center">
                  <div className="text-blue-400 text-sm font-bold tracking-widest font-mono mb-2 uppercase">Payments Done</div>
                  <div className="text-4xl font-extrabold">{data?.stats?.payments_done ?? 0}</div>
                </div>
              </div>

              {/* History Table */}
              <div className="bg-gray-900/40 border border-gray-800 rounded-2xl overflow-hidden backdrop-blur-md">
                <div className="px-6 py-4 border-b border-gray-800 bg-black/40">
                  <h3 className="font-bold text-lg">Transaction History</h3>
                </div>
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-sm text-gray-400">
                    <thead className="text-xs uppercase bg-black/40 font-mono text-gray-500">
                      <tr>
                        <th className="px-6 py-4">Payment ID</th>
                        <th className="px-6 py-4">Status</th>
                        <th className="px-6 py-4">Proof Hash</th>
                        <th className="px-6 py-4">Mint Address</th>
                      </tr>
                    </thead>
                    <tbody>
                      {data?.transactions?.length > 0 ? (
                        data.transactions.map((item: any) => (
                          <tr key={item.payment_id} className="border-b border-gray-800/50 hover:bg-white/5 transition-colors">
                            <td className="px-6 py-4 font-mono text-xs">{item.payment_id}</td>
                            <td className="px-6 py-4">
                              <span className={`px-2 py-1 rounded text-xs font-bold ${
                                item.status === 'COMPLETE' ? 'bg-green-500/10 text-green-400' :
                                item.status === 'refunded' ? 'bg-red-500/10 text-red-400' :
                                'bg-yellow-500/10 text-yellow-400'
                              }`}>
                                {item.status}
                              </span>
                            </td>
                            <td className="px-6 py-4 font-mono text-xs max-w-[200px] truncate" title={item.proof_hash}>
                              {item.proof_hash || '-'}
                            </td>
                            <td className="px-6 py-4 font-mono text-xs">
                              {item.mint_address ? (
                                <a href={`https://explorer.solana.com/address/${item.mint_address}?cluster=devnet`} target="_blank" rel="noreferrer" className="text-teal-400 hover:underline">
                                  {item.mint_address.slice(0,8)}...{item.mint_address.slice(-8)}
                                </a>
                              ) : '-'}
                            </td>
                          </tr>
                        ))
                      ) : (
                        <tr>
                          <td colSpan={4} className="px-6 py-8 text-center text-gray-500">
                            No transactions found for this wallet.
                          </td>
                        </tr>
                      )}
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
