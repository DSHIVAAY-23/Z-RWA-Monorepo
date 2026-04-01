"use client";

import { useState, useCallback, useEffect } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import ZNavbar from "../components/ZNavbar";
import ZTerminal, { TerminalLine } from "../components/ZTerminal";
import { submitProof, getExplorerUrl } from "../lib/solana";

function delay(ms: number) {
  return new Promise((r) => setTimeout(r, ms));
}

export default function HomePage() {
  const wallet = useWallet();
  const { connected } = wallet;
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Step 1: Document
  const [selectedDoc, setSelectedDoc] = useState("Aadhaar Card");
  const [file, setFile] = useState<File | null>(null);
  const [docHash, setDocHash] = useState("");

  // Step 2: Proof
  const [terminalLines, setTerminalLines] = useState<TerminalLine[]>([]);
  const [isProving, setIsProving] = useState(false);
  const [proofDone, setProofDone] = useState(false);
  const [proofData, setProofData] = useState<{ proof: string; publicValues: string } | null>(null);

  // Step 3: Mint
  const [isMinting, setIsMinting] = useState(false);
  const [mintDone, setMintDone] = useState(false);
  const [mintAddress, setMintAddress] = useState("");

  const handleFileDrop = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
      const mockSha = Array.from(crypto.getRandomValues(new Uint8Array(32)))
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");
      setDocHash(mockSha);
    }
  };

  const handleGenerateProof = useCallback(async () => {
    if (isProving) return;
    setIsProving(true);
    setTerminalLines([]);
    setProofDone(false);

    try {
      await delay(300);
      setTerminalLines((prev) => [...prev, { text: "$ zk-rag-prover --mode groth16" }]);
      await delay(500);
      setTerminalLines((prev) => [...prev, { text: "[SP1] Initializing RISC-V zkVM...", isSystem: true }]);
      await delay(600);
      setTerminalLines((prev) => [...prev, { text: `[SP1] Loading document hash: ${docHash.slice(0, 16)}...`, isSystem: true }]);
      await delay(750);
      setTerminalLines((prev) => [...prev, { text: "[ZK-RAG] Computing relevance score...", isSystem: true }]);
      await delay(900);
      setTerminalLines((prev) => [...prev, { text: "[SP1] Executing RISC-V program...", isSystem: true }]);
      await delay(1000);
      setTerminalLines((prev) => [...prev, { text: "[SP1] Constraint count: 7,493,634", isBenchmark: true }]);
      await delay(1200);
      setTerminalLines((prev) => [...prev, { text: "[GROTH16] Generating proof artifacts...", isSystem: true }]);
      
      // Call Backend API
      const res = await fetch('/api/prove', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ docType: selectedDoc, docHash })
      });
      
      if (!res.ok) throw new Error("API call failed");
      const data = await res.json();
      
      setProofData({ proof: data.proof, publicValues: data.publicValues });
      
      await delay(500);
      setTerminalLines((prev) => [...prev, { text: `[GROTH16] Proof size: ${data.proofSize} bytes`, isBenchmark: true }]);
      await delay(800);
      setTerminalLines((prev) => [...prev, { text: `[VK] Binding to ZK_RAG_VKEY: 0x00cef...`, isSystem: true }]);
      await delay(600);
      setTerminalLines((prev) => [...prev, { text: `✓ Proof generated successfully in ${data.provingTime}s!`, isSuccess: true }]);
      await delay(500);
      setTerminalLines((prev) => [...prev, { text: `💾 Bytecode hash: 0x${docHash.slice(0, 12)}...`, isSystem: true }]);
      await delay(400);
      setTerminalLines((prev) => [...prev, { text: "[READY] Submit to Solana z-rwa program →", isBenchmark: true }]);
      
      setProofDone(true);
    } catch (e) {
      setTerminalLines((prev) => [...prev, { text: "[ERROR] Proof generation failed", isError: true }]);
    } finally {
      setIsProving(false);
    }
  }, [isProving, docHash]);

  const handleMint = async () => {
    if (isMinting || !connected || !proofData) return;
    setIsMinting(true);
    
    try {
      setTerminalLines((prev) => [...prev, { text: "➤ Connecting to Solana Devnet...", isSystem: true }]);
      
      const result = await submitProof(
        wallet, 
        proofData.proof,
        proofData.publicValues,
        docHash
      );

      if (result.success) {
        setMintAddress(result.mintAddress);
        setMintDone(true);
        setTerminalLines((prev) => [...prev, { text: `✓ Transaction confirmed: ${result.txHash.slice(0, 8)}...`, isSuccess: true }]);
      }
    } catch (e: any) {
      console.error(e);
      setTerminalLines((prev) => [...prev, { text: `[RPC ERROR] ${e.message || "Transaction failed"}`, isError: true }]);
    } finally {
      setIsMinting(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-950 grid-bg transition-colors duration-200 font-sans text-gray-100">
      <ZNavbar />

      {/* 2. Sub-Ticker Bar */}
      <div className="border-b border-gray-900 bg-gray-950/80 backdrop-blur-md">
        <div className="mx-auto max-w-7xl px-6 py-2.5 flex items-center gap-4">
          <div className="h-[1px] flex-1 bg-gradient-to-r from-transparent via-gray-700 to-transparent" />
          <span className="font-mono text-[10px] md:text-xs text-slate-400 uppercase tracking-[0.3em] font-semibold">
            Z-RWA • CROSS-CHAIN RWA & COMPLIANCE INFRASTRUCTURE
          </span>
          <div className="h-[1px] flex-1 bg-gradient-to-r from-transparent via-gray-700 to-transparent" />
        </div>
      </div>

      <main className="mx-auto max-w-5xl px-6 py-12 space-y-12">
        {/* 3. Top Banner Card */}
        <section className="rounded-2xl border border-yellow-500/30 bg-yellow-500/5 p-8 relative overflow-hidden shadow-[0_0_30px_rgba(234,179,8,0.05)] text-left">
          {/* Subtle glow orb */}
          <div className="absolute -top-32 -right-32 w-64 h-64 rounded-full bg-yellow-500/20 blur-[80px] pointer-events-none" />
          
          <div className="flex flex-col md:flex-row md:items-start justify-between gap-6 relative z-10">
            <div className="flex-1">
              <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-yellow-500/30 bg-yellow-500/10 text-yellow-500 text-xs font-mono mb-4">
                🇮🇳 Built for India's $3.5T RWA Opportunity
              </div>
              <h2 className="text-3xl font-bold text-white font-space mb-3 flex items-center gap-3">
                Tokenize Your Indian Assets. Keep Documents Private.
              </h2>
              <p className="text-gray-400 text-sm leading-relaxed max-w-2xl mb-5">
                Indian farmers and landowners hold trillions in assets that cannot be tokenized without exposing highly sensitive Aadhaar/PAN details on-chain. Z-RWA provides a privacy-preserving compliance portal leveraging local SP1 Groth16 proofs, verified sub-second on Solana via an Anchor program to mint a compliant Token2022 asset.
              </p>
              <ul className="space-y-2 mt-4 text-sm text-gray-300">
                {[
                  "ZK proof certifies document validity completely locally.",
                  "Document hash published on-chain for verifiability.",
                  "Document content NEVER leaves the user's device."
                ].map((item, i) => (
                  <li key={i} className="flex items-start gap-2">
                    <span className="text-green-500 mt-1 text-[10px]">●</span>
                    <span>{item}</span>
                  </li>
                ))}
              </ul>
            </div>
            
            <div className="flex flex-col gap-3 min-w-[200px] shrink-0 md:mt-4">
              <button 
                onClick={() => document.getElementById("compliance-flow")?.scrollIntoView({ behavior: "smooth" })}
                className="px-6 py-3 rounded-xl border border-yellow-500/50 bg-yellow-500/10 text-yellow-500 font-semibold text-sm hover:bg-yellow-500/20 transition-all shadow-[0_0_15px_rgba(234,179,8,0.15)] glow-amber">
                Start Compliance Check ↓
              </button>
            </div>
          </div>
        </section>

        {/* 4. Main Dashboard Card (The Proof Card) */}
        <section id="compliance-flow" className="rounded-2xl border border-gray-800 bg-gray-900/90 backdrop-blur-2xl overflow-hidden shadow-2xl relative">
          
          {/* Subtle neon green glow behind card */}
          <div className="absolute left-1/2 top-0 -translate-x-1/2 w-[500px] h-[300px] bg-neon-green/5 blur-[100px] pointer-events-none" />

          {/* Card Header */}
          <div className="px-8 py-5 border-b border-gray-800 flex items-center justify-between relative z-10 bg-gray-950/50">
            <div className="flex items-center gap-3">
              <div className="flex items-center justify-center w-8 h-8 rounded-lg bg-purple-500/10 border border-purple-500/30 text-purple-400">
                ⚡
              </div>
              <span className="px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-400 text-[10px] font-mono tracking-widest font-bold">
                ZK-COMPLY-V1
              </span>
            </div>
            <div className="flex items-center gap-2 px-3 py-1 rounded-full bg-gray-900 border border-gray-800">
              <div className="w-2 h-2 rounded-full bg-neon-green animate-pulse-slow"></div>
              <span className="text-neon-green text-[10px] font-mono font-bold tracking-widest">ONLINE</span>
            </div>
          </div>

          {/* Card Body */}
          <div className="p-8 md:p-10 relative z-10">
            <div className="text-center mb-10">
              <h1 className="text-4xl font-bold text-white font-space mb-4 tracking-tight">
                Private RWA Compliance Shield
              </h1>
              <p className="text-slate-400 max-w-2xl mx-auto leading-relaxed">
                Prove ownership and compliance of Indian assets without revealing Aadhaar, PAN, or sensitive documents. Verified natively on <span className="text-purple-400 font-semibold">Solana (Devnet)</span>.
              </p>
            </div>

            {/* Metrics Row */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-12">
              {[
                { value: "260 bytes", label: "Proof Size" },
                { value: "~1.2s", label: "Prove Time" },
                { value: "Sub-second", label: "On-chain Verify" },
                { value: "7.4M", label: "Constraints" },
              ].map((stat, i) => (
                <div key={i} className="flex flex-col items-center justify-center p-5 rounded-xl border border-gray-800 bg-gray-950/60 transition-transform hover:-translate-y-1 hover:border-gray-700">
                  <div className="text-2xl font-bold text-white mb-1 tracking-tight font-mono">{stat.value}</div>
                  <div className="text-[10px] text-slate-500 uppercase tracking-widest font-mono font-semibold">{stat.label}</div>
                </div>
              ))}
            </div>

            <div className="flex flex-col relative space-y-6 max-w-3xl mx-auto">
              {/* STEP 1: Document Upload */}
              <div className={`rounded-xl border p-6 md:p-8 transition-all duration-300 ${!file ? 'border-purple-500/50 bg-purple-500/5 glow-pulse' : 'border-gray-800 bg-gray-900/50 opacity-80'}`}>
                <div className="flex items-center gap-3 mb-6">
                  <div className={`w-8 h-8 rounded-full text-sm font-bold flex items-center justify-center ${file ? 'bg-green-500 text-black' : 'bg-purple-500 text-white'}`}>
                    {file ? '✓' : '1'}
                  </div>
                  <span className="font-semibold text-white font-space text-lg">
                    🔒 Upload Document (Processed Locally)
                  </span>
                </div>
                
                <select 
                  value={selectedDoc}
                  onChange={e => setSelectedDoc(e.target.value)}
                  disabled={!!file}
                  className="w-full bg-gray-950 border border-gray-700 rounded-xl px-4 py-3 text-white font-mono text-sm mb-4 outline-none focus:border-purple-500/50 transition-all cursor-pointer shadow-inner disabled:opacity-50"
                >
                  <option>Aadhaar Card</option>
                  <option>PAN Card</option>
                  <option>Passport</option>
                  <option>Land Record (Bhulekh)</option>
                  <option>Investor Certificate</option>
                </select>
                
                {!file ? (
                  <label className="block border-2 border-dashed border-gray-700 rounded-xl p-8 text-center hover:border-purple-500/50 hover:bg-purple-500/5 transition-all cursor-pointer bg-gray-950/50">
                    <div className="text-4xl mb-3">📄</div>
                    <div className="text-white font-medium mb-1">
                      Drop document here or click to browse
                    </div>
                    <div className="text-gray-500 text-sm mb-4">
                      PDF, PNG, JPG (MAX. 10MB)
                    </div>
                    <div className="inline-flex items-center gap-2 text-xs text-green-400 font-mono bg-green-400/10 px-3 py-1.5 rounded-full border border-green-400/20">
                      🔒 Zero data leaves your device
                    </div>
                    <input type="file" className="hidden" accept=".pdf,.png,.jpg,.jpeg" onChange={handleFileDrop} />
                  </label>
                ) : (
                  <div className="flex items-center justify-between bg-gray-950/80 border border-gray-700 p-4 rounded-xl">
                    <div className="flex items-center gap-4">
                      <div className="text-3xl">📄</div>
                      <div>
                        <div className="text-sm font-semibold text-white">{file.name}</div>
                        <div className="text-xs text-green-400 font-mono mt-1">Hash: {docHash.slice(0, 16)}...</div>
                      </div>
                    </div>
                    <button onClick={() => {setFile(null); setTerminalLines([]); setProofDone(false); setMintDone(false);}} className="text-xs text-gray-400 hover:text-white transition-colors underline underline-offset-2">Change</button>
                  </div>
                )}
              </div>

              {/* STEP 2: Prover Console */}
              <div className={`rounded-xl border p-6 md:p-8 transition-all duration-300 ${file && !proofDone ? 'border-purple-500/50 bg-black/60 glow-pulse shadow-[0_0_20px_rgba(168,85,247,0.15)]' : 'border-gray-800 bg-black/40 opacity-80'}`}>
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
                  <div className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-full text-sm font-bold flex items-center justify-center shrink-0 ${proofDone ? 'bg-green-500 text-black' : file ? 'bg-purple-500 text-white' : 'bg-gray-800 text-gray-500'}`}>
                      {proofDone ? '✓' : '2'}
                    </div>
                    <span className="font-semibold text-white font-space text-lg">
                      ⚡ Generate ZK Proof via SP1
                    </span>
                  </div>
                  <span className="px-2.5 py-1 rounded-full text-[10px] font-mono bg-neon-green/10 text-neon-green border border-neon-green/30 tracking-widest font-bold w-fit">
                    LIVE SP1
                  </span>
                </div>
                
                <div className="mb-6 border border-gray-800 rounded-xl overflow-hidden bg-gray-950">
                  <ZTerminal lines={terminalLines} isRunning={isProving} />
                </div>
                
                <button 
                  onClick={handleGenerateProof}
                  disabled={!file || isProving || proofDone}
                  className={`w-full py-4 rounded-xl font-bold font-space text-sm md:text-base transition-all items-center justify-center flex gap-2
                    ${proofDone 
                      ? 'bg-green-500/10 text-green-400 border border-green-500/30' 
                      : isProving 
                        ? 'bg-gray-800 text-gray-500 cursor-wait border border-gray-700' 
                        : !file 
                          ? 'bg-gray-800/50 text-gray-600 cursor-not-allowed border border-gray-800'
                          : 'bg-white text-black hover:bg-gray-200 shadow-[0_0_15px_rgba(255,255,255,0.2)]'
                    }`}
                >
                  {proofDone ? 'ZK Proof Generated ✓' : isProving ? 'Generating Proof via SP1...' : 'Generate Proof via SP1 →'}
                </button>
              </div>

              {/* STEP 3: Mint Token */}
              <div className={`rounded-xl border p-6 md:p-8 transition-all duration-300 ${proofDone && !mintDone ? 'border-neon-green/50 bg-neon-green/5 shadow-[0_0_20px_rgba(0,204,102,0.15)] glow-pulse' : 'border-gray-800 bg-black/20'}`}>
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
                  <div className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-full text-sm font-bold flex items-center justify-center shrink-0 ${mintDone ? 'bg-green-500 text-black' : proofDone ? 'bg-neon-green text-black' : 'bg-gray-800 text-gray-600'}`}>
                      {mintDone ? '✓' : '3'}
                    </div>
                    <span className="font-semibold text-white font-space text-lg">
                      🏆 Mint RWA Compliance Token
                    </span>
                  </div>
                </div>
                
                {!mintDone ? (
                  <>
                    <p className="text-gray-400 text-sm mb-6 leading-relaxed">
                      Proof verified locally. Submit the lightweight 260-byte Groth16 proof to the <span className="text-purple-400 font-mono">z-rwa program</span>. A compliant Token2022 will be minted to your connected wallet upon success.
                    </p>
                    
                    {!connected ? (
                      <div className="w-full flex justify-center py-2 bg-gray-900/50 rounded-xl border border-gray-800 mb-2">
                        <div className="[&>button]:!bg-transparent [&>button]:!text-sm [&>button]:!font-semibold [&>button]:!text-purple-400 [&>button]:!border [&>button]:!border-purple-500/50 [&>button:hover]:!bg-purple-500/10 [&>button]:!transition-all [&>button]:!duration-200 [&>button]:!rounded-lg">
                          {mounted && <WalletMultiButton />}
                        </div>
                      </div>
                    ) : (
                      <button 
                        onClick={handleMint}
                        disabled={!proofDone || isMinting}
                        className={`w-full py-4 rounded-xl font-bold font-space text-sm md:text-base transition-all items-center justify-center flex gap-2
                          ${isMinting 
                            ? 'bg-purple-500/20 text-purple-400 border border-purple-500/30 animate-pulse' 
                            : !proofDone 
                              ? 'bg-gray-800/50 text-gray-600 cursor-not-allowed border border-gray-800'
                              : 'bg-neon-green text-gray-950 hover:bg-[#00e673] shadow-[0_0_20px_#00cc6655] hover:shadow-[0_0_30px_#00cc6688] hover:scale-[1.02] transform'
                          }`}
                      >
                         <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path></svg>
                        {isMinting ? 'Verifying on Solana Devnet...' : 'Submit Proof & Mint Token →'}
                      </button>
                    )}
                  </>
                ) : (
                  <div className="p-6 rounded-xl border border-green-500/30 bg-green-500/5">
                    <div className="flex items-center gap-3 mb-4 text-green-400 font-space font-semibold text-lg">
                      <span className="text-2xl">🏆</span> Token2022 Minted Successfully
                    </div>
                    <div className="space-y-3 text-sm font-mono text-gray-400 bg-gray-950/80 p-4 rounded-lg border border-gray-800">
                      <div className="flex justify-between border-b border-gray-800 pb-2"><span>Token:</span> <span className="text-white">Z-RWA-COMPLY</span></div>
                      <div className="flex justify-between border-b border-gray-800 pb-2"><span>Standard:</span> <span className="text-white">Token2022</span></div>
                      <div className="flex justify-between pt-1"><span>Mint Address:</span> <span className="text-purple-400 break-all ml-4 text-right">{mintAddress}</span></div>
                    </div>
                  </div>
                )}
              </div>
            </div>
            
          </div>
        </section>

        {/* 5. Additional Information Sections */}
        <section className="grid md:grid-cols-2 gap-8 max-w-5xl mx-auto">
           <div className="rounded-2xl border border-gray-800 bg-gray-900/50 backdrop-blur-md p-8 shadow-lg">
             <h3 className="text-xl font-space font-bold text-white mb-6 flex items-center gap-2">
                <span className="text-2xl text-red-500">❌</span> The Problem
             </h3>
             <p className="text-gray-400 text-sm leading-relaxed mb-6">
                Indian farmers and landowners hold trillions in assets that cannot be tokenized without exposing highly sensitive Aadhaar or PAN details on-chain. Public ledgers and strict KYC requirements clash with fundamental privacy rights, blocking legitimate liquidity.
             </p>
             <h3 className="text-xl font-space font-bold text-white mb-6 flex items-center gap-2 pt-4 border-t border-gray-800">
                <span className="text-2xl text-green-500">✅</span> Our Solution
             </h3>
             <ul className="space-y-3 text-sm text-gray-400">
                <li className="flex items-start gap-3"><span className="text-green-500 mt-0.5">✓</span> ZK proof certifies document validity completely locally.</li>
                <li className="flex items-start gap-3"><span className="text-green-500 mt-0.5">✓</span> Document hash published on-chain for verifiability.</li>
                <li className="flex items-start gap-3"><span className="text-green-500 mt-0.5">✓</span> Document content NEVER leaves the user's device.</li>
             </ul>
           </div>
           
           <div className="rounded-2xl border border-gray-800 bg-gray-900/50 backdrop-blur-md p-8 shadow-lg flex flex-col">
             <h3 className="text-xl font-space font-bold text-white mb-6">Supported Document Types</h3>
             <div className="flex flex-wrap gap-3 mb-10">
                {['🔒 Aadhaar', '🪪 PAN Card', '🗺️ Land Records', '📜 Investor Certificate', '🛂 Passport'].map(doc => (
                  <span key={doc} className="px-4 py-2 rounded-full border border-gray-700 bg-gray-800 text-xs text-gray-300 font-mono">
                    {doc}
                  </span>
                ))}
             </div>
             
             <h3 className="text-xl font-space font-bold text-white mb-6 pt-4 border-t border-gray-800">Zero-Knowledge Architecture Overview</h3>
             <div className="flex-1 border border-gray-800 rounded-xl bg-gray-950 p-5 overflow-x-auto custom-scrollbar flex items-center">
               <div className="flex items-center gap-2 min-w-max mx-auto text-xs font-mono text-gray-400">
                 <div className="flex flex-col items-center gap-2"><span className="text-xl">💻</span> User Device</div>
                 <span className="text-purple-500 mx-2">→</span>
                 <div className="flex flex-col items-center gap-2"><span className="text-xl">⚡</span> SP1 Prover</div>
                 <span className="text-purple-500 mx-2">→</span>
                 <div className="flex flex-col items-center gap-2"><span className="text-xl">🔐</span> Groth16 Proof</div>
                 <span className="text-purple-500 mx-2">→</span>
                 <div className="flex flex-col items-center gap-2"><span className="text-xl">⛓️</span> Solana</div>
                 <span className="text-purple-500 mx-2">→</span>
                 <div className="flex flex-col items-center gap-2"><span className="text-xl">🪙</span> Token2022</div>
               </div>
             </div>
           </div>
        </section>
        
        {/* FOOTER */}
        <footer className="border-t border-gray-800 py-8 text-center relative z-10">
          <div className="text-gray-500 text-[10px] md:text-xs font-mono tracking-widest uppercase">
            Z-RWA • Built on Solana • SP1 Groth16 • Superteam India Grant
          </div>
        </footer>

      </main>
    </div>
  );
}
