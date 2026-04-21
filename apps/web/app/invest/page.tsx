"use client";

import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

// Simple client-side hasher (Substituting Poseidon with SHA-256 for browser demo purposes)
async function hashValue(val: string): Promise<string> {
    const encoder = new TextEncoder();
    const data = encoder.encode(val);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data as any);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

export default function InvestPage() {
    const { publicKey } = useWallet();
    const [step, setStep] = useState(1);
    const [paymentId, setPaymentId] = useState<string | null>(null);

    // Step 1 State
    const [aadhaar, setAadhaar] = useState('');
    const [pan, setPan] = useState('');
    const [walletAddress, setWalletAddress] = useState('');
    const [aadhaarHash, setAadhaarHash] = useState('');
    const [panHash, setPanHash] = useState('');

    // Step 2 State
    const [amount, setAmount] = useState(1000);
    const [isCheckingOut, setIsCheckingOut] = useState(false);

    // Step 3 State
    const [status, setStatus] = useState<string | null>(null);
    const [txData, setTxData] = useState<any>({});

    useEffect(() => {
        if (publicKey && !walletAddress) {
            setWalletAddress(publicKey.toBase58());
        }
    }, [publicKey]);

    useEffect(() => {
        // Detect paymentId from URL (returning from Dodo)
        const params = new URLSearchParams(window.location.search);
        const pid = params.get('paymentId');
        if (pid) {
            setPaymentId(pid);
            setStep(3);
        }
    }, []);

    // Poll status in Step 3
    useEffect(() => {
        if (step === 3 && paymentId && status !== 'complete' && status !== 'failed') {
            const interval = setInterval(async () => {
                try {
                    const res = await fetch(`/api/payment-status/${paymentId}`);
                    if (res.ok) {
                        const data = await res.json();
                        setStatus(data.status);
                        if (data.status === 'complete') {
                            setTxData(data);
                            clearInterval(interval);
                        } else if (data.status === 'not_found') {
                            // wait...
                        }
                    }
                } catch (e) {
                    // Ignore transient errors
                }
            }, 3000);
            return () => clearInterval(interval);
        }
    }, [step, paymentId, status]);

    const formatAadhaar = (val: string) => {
        return val.replace(/\\D/g, '').replace(/(.{4})/g, '$1-').slice(0, 14).replace(/\\-$/, '');
    };

    const handleStep1Submit = async (e: React.FormEvent) => {
        e.preventDefault();
        const aHash = await hashValue(aadhaar.replace(/\\-/g, ''));
        const pHash = await hashValue(pan);
        setAadhaarHash(aHash);
        setPanHash(pHash);
        setStep(2);
    };

    const handleCheckout = async () => {
        setIsCheckingOut(true);
        try {
            const res = await fetch('/api/create-checkout', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ amount, walletAddress, aadhaarHash, panHash })
            });
            const data = await res.json();
            if (data.checkoutUrl) {
                window.location.href = data.checkoutUrl;
            } else {
                alert("Checkout error");
                setIsCheckingOut(false);
            }
        } catch (e) {
            alert("Failed to initialize checkout");
            setIsCheckingOut(false);
        }
    };

    return (
        <div className="min-h-screen bg-black text-white p-6 md:p-12 font-sans selection:bg-purple-900">
            <div className="max-w-2xl mx-auto">
               <div className="flex justify-end mb-4">
                   <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700 !transition-colors !rounded-lg" />
               </div>

                <div className="text-center mb-10">
                    <h1 className="text-4xl font-extrabold tracking-tight text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-indigo-500 mb-2">
                        Invest in RWA Tokens
                    </h1>
                    <p className="text-gray-400">Compliant. Private. Instant.</p>
                </div>

                {/* Step Indicator */}
                <div className="flex items-center justify-between mb-8 text-sm">
                    <div className={`flex flex-col items-center flex-1 ${step >= 1 ? 'text-purple-400' : 'text-gray-600'}`}>
                        <div className={`w-8 h-8 rounded-full flex items-center justify-center mb-2 ${step >= 1 ? 'bg-purple-600/20 border-2 border-purple-500 text-white' : 'bg-gray-800'}`}>1</div>
                        <span className="hidden sm:block">Verify Identity</span>
                    </div>
                    <div className={`h-1 flex-1 mx-2 rounded ${step >= 2 ? 'bg-purple-500' : 'bg-gray-800'}`} />
                    <div className={`flex flex-col items-center flex-1 ${step >= 2 ? 'text-purple-400' : 'text-gray-600'}`}>
                        <div className={`w-8 h-8 rounded-full flex items-center justify-center mb-2 ${step >= 2 ? 'bg-purple-600/20 border-2 border-purple-500 text-white' : 'bg-gray-800'}`}>2</div>
                        <span className="hidden sm:block">Pay in INR</span>
                    </div>
                    <div className={`h-1 flex-1 mx-2 rounded ${step >= 3 ? 'bg-purple-500' : 'bg-gray-800'}`} />
                    <div className={`flex flex-col items-center flex-1 ${step >= 3 ? 'text-purple-400' : 'text-gray-600'}`}>
                        <div className={`w-8 h-8 rounded-full flex items-center justify-center mb-2 ${step >= 3 ? 'bg-purple-600/20 border-2 border-purple-500 text-white' : 'bg-gray-800'}`}>3</div>
                        <span className="hidden sm:block">Receive Token</span>
                    </div>
                </div>

                {/* Main Panel */}
                <div className="bg-gray-900/50 backdrop-blur-xl border border-gray-800 rounded-2xl p-6 shadow-2xl relative overflow-hidden">
                    {/* Decorative gradient blob */}
                    <div className="absolute -top-20 -right-20 w-40 h-40 bg-purple-600 rounded-full blur-3xl opacity-20 pointer-events-none" />
                    
                    {step === 1 && (
                        <form onSubmit={handleStep1Submit} className="space-y-5 relative z-10">
                            <div>
                                <label className="block text-sm font-medium text-gray-300 mb-1">Aadhaar Number</label>
                                <input 
                                    className="w-full bg-black border border-gray-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent font-mono tracking-widest placeholder-gray-600"
                                    type="password"
                                    required
                                    placeholder="XXXX-XXXX-XXXX"
                                    value={aadhaar}
                                    onChange={(e) => setAadhaar(formatAadhaar(e.target.value))}
                                    pattern="[0-9]{4}-[0-9]{4}-[0-9]{4}"
                                    maxLength={14}
                                />
                            </div>
                            <div>
                                <label className="block text-sm font-medium text-gray-300 mb-1">PAN Number</label>
                                <input 
                                    className="w-full bg-black border border-gray-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent font-mono uppercase placeholder-gray-600"
                                    type="text"
                                    required
                                    placeholder="ABCDE1234F"
                                    value={pan}
                                    onChange={(e) => setPan(e.target.value.toUpperCase())}
                                    pattern="[A-Z]{5}[0-9]{4}[A-Z]{1}"
                                    maxLength={10}
                                />
                            </div>
                            <div>
                                <label className="block text-sm font-medium text-gray-300 mb-1">Solana Wallet Address</label>
                                <input 
                                    className="w-full bg-black border border-gray-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent font-mono text-sm placeholder-gray-600"
                                    type="text"
                                    required
                                    placeholder="Connect wallet or enter address"
                                    value={walletAddress}
                                    onChange={(e) => setWalletAddress(e.target.value)}
                                />
                            </div>
                            
                            <p className="text-xs text-gray-500 flex items-center gap-2 mt-4">
                                <span className="opacity-80">🔒</span> Your Aadhaar/PAN is hashed locally and never transmitted.
                            </p>
                            
                            <button type="submit" className="w-full mt-6 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white font-bold py-3 px-6 rounded-lg transition-all transform active:scale-[0.98] shadow-[0_0_20px_rgba(139,92,246,0.3)]">
                                Continue to Payment
                            </button>
                        </form>
                    )}

                    {step === 2 && (
                        <div className="space-y-6 relative z-10 text-center py-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-400 mb-2">Investment Amount (INR)</label>
                                <div className="relative max-w-sm mx-auto">
                                    <span className="absolute left-4 top-1/2 -translate-y-1/2 text-2xl text-gray-500 font-light">₹</span>
                                    <input 
                                        className="w-full bg-black border border-gray-700 rounded-lg pl-10 pr-4 py-4 text-3xl text-white font-light focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                                        type="number"
                                        min={1000}
                                        max={100000}
                                        value={amount}
                                        onChange={(e) => setAmount(Number(e.target.value))}
                                    />
                                </div>
                            </div>

                            <div className="bg-purple-900/10 border border-purple-500/20 rounded-xl p-4 max-w-sm mx-auto text-purple-200">
                                You will receive: <span className="font-bold text-white text-lg">{(amount / 100).toFixed(2)} RWA</span> tokens
                            </div>

                            <div className="pt-4 max-w-sm mx-auto">
                                <button 
                                    onClick={handleCheckout} 
                                    disabled={isCheckingOut || amount < 1000}
                                    className="w-full bg-white text-black font-bold py-4 px-6 rounded-lg transition-all hover:bg-gray-100 disabled:opacity-50 flex items-center justify-center gap-2"
                                >
                                    {isCheckingOut ? (
                                        <span className="animate-spin h-5 w-5 border-2 border-black border-t-transparent rounded-full" />
                                    ) : 'Pay with Dodo'}
                                </button>
                                <p className="text-xs text-gray-500 mt-4 flex justify-center gap-3">
                                    <span>UPI</span> &bull; <span>Credit/Debit Card</span> &bull; <span>Net Banking</span>
                                </p>
                            </div>
                        </div>
                    )}

                    {step === 3 && (
                        <div className="space-y-6 relative z-10 py-6">
                            <div className="text-center space-y-4">
                                
                                {status === 'failed' ? (
                                    <>
                                       <div className="w-16 h-16 bg-red-500/20 text-red-500 rounded-full flex items-center justify-center mx-auto text-2xl mb-4 border border-red-500/30">✕</div>
                                       <h3 className="text-xl font-bold">Payment Failed</h3>
                                       <p className="text-gray-400">Your transaction could not be completed.</p>
                                    </>
                                ) : (
                                    <>
                                        <div className="w-16 h-16 bg-gradient-to-tr from-purple-600 to-indigo-500 rounded-full flex items-center justify-center mx-auto text-2xl mb-4 shadow-[0_0_30px_rgba(139,92,246,0.5)]">
                                            {status === 'complete' ? '✓' : <span className="animate-pulse">...</span>}
                                        </div>
                                        <h3 className="text-2xl font-bold">
                                            {status === 'complete' ? 'Investment Complete!' : 'Processing...'}
                                        </h3>
                                        
                                        <div className="text-sm font-medium mt-6 max-w-xs mx-auto space-y-3 text-left">
                                            <div className="flex items-center gap-3 text-gray-400">
                                               {['processing','complete'].includes(status || '') ? <span className="text-green-500">✓</span> : <span className="text-purple-400 animate-spin">⚪</span>} 
                                               <span className={status ? 'text-gray-200' : ''}>Payment confirmed</span>
                                            </div>
                                            <div className="flex items-center gap-3 text-gray-400">
                                                {status === 'complete' ? <span className="text-green-500">✓</span> : (status === 'processing' ? <span className="text-purple-400 animate-spin">⚪</span> : <span className="opacity-0">✓</span>)} 
                                                <span className={['processing','complete'].includes(status || '') ? 'text-gray-200' : ''}>Generating ZK proof...</span>
                                            </div>
                                            <div className="flex items-center gap-3 text-gray-400">
                                                {status === 'complete' ? <span className="text-green-500">✓</span> : <span className="opacity-0">✓</span>} 
                                                <span className={status === 'complete' ? 'text-gray-200' : ''}>Minting token...</span>
                                            </div>
                                        </div>
                                    </>
                                )}
                            </div>

                            {status === 'complete' && txData.proofHash && (
                                <div className="mt-8 bg-black/50 p-4 border border-gray-800 rounded-lg text-sm space-y-4">
                                    <div>
                                        <div className="text-gray-500 font-mono text-xs mb-1">RWA TOKEN ADDRESS</div>
                                        <a href={`https://explorer.solana.com/address/${txData.tokenAddress}?cluster=devnet`} target="_blank" className="text-purple-400 hover:text-purple-300 font-mono break-all inline-flex items-center gap-1">
                                            {txData.tokenAddress} <span className="text-xs">↗</span>
                                        </a>
                                    </div>
                                    <div>
                                        <div className="text-gray-500 font-mono text-xs mb-1">TRANSACTION</div>
                                        <a href={`https://explorer.solana.com/tx/${txData.txSignature}?cluster=devnet`} target="_blank" className="text-purple-400 hover:text-purple-300 font-mono break-all inline-flex items-center gap-1">
                                            {txData.txSignature.slice(0, 32)}... <span className="text-xs">↗</span>
                                        </a>
                                    </div>
                                    <div>
                                        <div className="text-gray-500 font-mono text-xs mb-1 flex items-center justify-between">
                                            <span>ZK PROOF HASH</span>
                                            <button onClick={() => navigator.clipboard.writeText(txData.proofHash)} className="hover:text-white transition-colors">Copy</button>
                                        </div>
                                        <div className="text-gray-300 font-mono break-all text-xs opacity-70">
                                            {txData.proofHash}
                                        </div>
                                    </div>
                                    <div className="pt-2">
                                        <button onClick={() => alert("Check phantom extension to view balance!")} className="w-full border border-purple-500 text-purple-400 hover:bg-purple-900/30 py-2 rounded font-medium transition-colors">
                                            View in Wallet
                                        </button>
                                    </div>
                                </div>
                            )}
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}
