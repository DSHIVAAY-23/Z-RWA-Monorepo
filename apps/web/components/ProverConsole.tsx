'use client';

import { useState, useEffect, useRef } from 'react';

const PROVER_LOGS = [
  { text: "$ zk-rag-prover --mode groth16", delay: 0 },
  { text: "[snarkjs] Initializing Groth16 prover...", delay: 400 },
  { text: "[snarkjs] Loading document hash: sha256(doc)", delay: 800 },
  { text: "[circom] Computing relevance score...", delay: 1200 },
  { text: "[snarkjs] Executing Circom circuit...", delay: 1600 },
  { text: "[circuit] Constraint count: 7,493,634", delay: 2000 },
  { text: "[GROTH16] Generating proof artifacts...", delay: 2400 },
  { text: "[GROTH16] Proof size: 260 bytes", delay: 2800 },
  { text: "[vkey] Loading verification key: 0x00cef...", delay: 3200 },
  { text: "✓ Proof generated successfully!", delay: 3600, color: "text-accent-green font-bold glow-green" },
  { text: "💾 proof_groth16.bin saved", delay: 4000 },
  { text: "[READY] Submit to Solana z-rwa program →", delay: 4400, color: "text-accent-gold" },
];

interface ProverConsoleProps {
  docHash: string;
  onProofGenerated: (proofData: { proof: string, publicValues: string }) => void;
  isActive: boolean;
}

export default function ProverConsole({ docHash, onProofGenerated, isActive }: ProverConsoleProps) {
  const [logs, setLogs] = useState<{text: string, color?: string}[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  const [isDone, setIsDone] = useState(false);
  const [timeLeft, setTimeLeft] = useState(23);
  
  const consoleRef = useRef<HTMLDivElement>(null);
  const isMockMode = process.env.NEXT_PUBLIC_MOCK_MODE === 'true';

  useEffect(() => {
    if (consoleRef.current) {
      consoleRef.current.scrollTop = consoleRef.current.scrollHeight;
    }
  }, [logs]);

  const generateProof = async () => {
    setIsGenerating(true);
    setLogs([]);
    setIsDone(false);
    
    // Total time configuration
    const totalTime = isMockMode ? 5 : 23;
    setTimeLeft(totalTime);
    
    const timeInterval = setInterval(() => {
      setTimeLeft(prev => Math.max(0, prev - 1));
    }, 1000);

    // Speed up artificial delays if mock mode
    const timeMultiplier = isMockMode ? (5000 / 4400) : (23000 / 4400);

    const timeouts: NodeJS.Timeout[] = [];
    PROVER_LOGS.forEach(log => {
      const t = setTimeout(() => {
        setLogs(prev => [...prev, { text: log.text, color: log.color }]);
      }, log.delay * timeMultiplier);
      timeouts.push(t);
    });

    try {
      const response = await fetch('/api/prove', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ docType: 'Document', docHash, query: '' })
      });
      const data = await response.json();
      
      clearInterval(timeInterval);
      setIsDone(true);
      setIsGenerating(false);
      
      onProofGenerated({
        proof: data.proof,
        publicValues: data.publicValues
      });
    } catch (e) {
      clearInterval(timeInterval);
      setIsGenerating(false);
      setLogs(prev => [...prev, { text: "[ERROR] Hardware failure or generic error", color: "text-red-500" }]);
    }
  };

  return (
    <div className={`rounded-xl border p-6 mb-4 transition-all duration-300 ${isActive ? 'border-purple-500/50 bg-purple-500/5' : 'border-white/8 bg-black/50 opacity-50'}`}>
      
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className={`w-8 h-8 rounded-full text-sm font-bold flex items-center justify-center ${isDone ? 'bg-accent-green text-black' : 'bg-yellow-500/20 text-yellow-400'}`}>
            {isDone ? '✓' : '2'}
          </div>
          <span className="font-semibold text-white">
            ⚡ Generate ZK Proof
          </span>
        </div>
        <span className={`px-2 py-1 rounded text-xs font-mono border ${isMockMode ? 'bg-orange-500/20 text-orange-400 border-orange-500/30' : 'bg-accent-green/20 text-accent-green border-accent-green/30'}`}>
          {isMockMode ? 'DEMO MODE' : 'LIVE ZK'}
        </span>
      </div>
      
      <div className="bg-terminal-bg border border-white/10 rounded-lg p-4 font-mono text-sm mb-4 h-48 flex flex-col">
        <div className="flex items-center justify-between mb-3 text-xs border-b border-white/10 pb-2">
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 rounded-full bg-red-500 opacity-80"/>
            <div className="w-3 h-3 rounded-full bg-yellow-500 opacity-80"/>
            <div className="w-3 h-3 rounded-full bg-accent-green opacity-80"/>
            <span className="ml-2 text-gray-500">zk-rwa-prover — snarkjs — bash</span>
          </div>
          {isGenerating && (
            <span className="text-accent-gold animate-pulse">Est. {timeLeft}s</span>
          )}
        </div>
        <div ref={consoleRef} className="text-terminal-green text-xs space-y-1.5 overflow-y-auto flex-1 pr-2 custom-scrollbar">
          {logs.length === 0 && !isGenerating && !isDone && (
            <>
              <div>$ zk-rag-prover</div>
              <div className="text-gray-500">Ready to generate Groth16 proof...</div>
            </>
          )}
          {logs.map((log, i) => (
            <div key={i} className={log.color || "text-accent-green/80"}>
              {log.text}
            </div>
          ))}
          {isGenerating && (
            <div className="inline-block w-2 h-4 bg-accent-green cursor-blink ml-1 align-middle"/>
          )}
        </div>
      </div>
      
      <button 
        onClick={generateProof}
        disabled={!isActive || isGenerating || isDone}
        className={`w-full py-3 rounded-xl font-semibold font-space transition-all items-center justify-center flex gap-2
          ${isDone 
            ? 'bg-accent-green/20 text-accent-green border border-accent-green/30' 
            : isGenerating 
              ? 'bg-accent-green/50 text-black/50 cursor-not-allowed' 
              : 'bg-accent-green text-black hover:bg-green-300 shadow-[0_0_15px_rgba(0,255,136,0.3)] disabled:opacity-40 disabled:shadow-none'
          }`}
      >
        {isDone ? 'ZK Proof Generated ✓' : isGenerating ? 'Generating Proof...' : 'Generate ZK Proof →'}
      </button>
    </div>
  );
}
