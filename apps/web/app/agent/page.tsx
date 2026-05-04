"use client";

import { useEffect, useState } from "react";
import ZNavbar from "../../components/ZNavbar";

interface LogEntry {
  time: string;
  type: "info" | "success" | "error" | "process";
  text: string;
}

const INITIAL_LOGS: LogEntry[] = [
  { time: new Date().toLocaleTimeString(), type: "info", text: "Initializing Zerion Compliance Agent..." },
  { time: new Date().toLocaleTimeString(), type: "info", text: "Loading Policy: Z-RWA Compliance Policy v1" },
  { time: new Date().toLocaleTimeString(), type: "info", text: "Chain Lock: solana | Max Spend: $10,000/tx" },
  { time: new Date().toLocaleTimeString(), type: "info", text: "Connection to Z-RWA Oracle established." },
  { time: new Date().toLocaleTimeString(), type: "process", text: "Waiting for pending RWA transfer requests..." },
];

export default function AgentDashboard() {
  const [logs, setLogs] = useState<LogEntry[]>(INITIAL_LOGS);
  const [running, setRunning] = useState(false);

  // Simulate the agent running
  const runAgentSequence = () => {
    if (running) return;
    setRunning(true);
    setLogs(INITIAL_LOGS);

    const sequence = [
      { t: 1000, type: "process", text: "Found 3 pending requests in queue. Processing..." },
      
      { t: 2500, type: "info", text: "--- Request req-001 ---" },
      { t: 3000, type: "info", text: "From: DHEi2Q72... | Amount: $100" },
      { t: 4000, type: "process", text: "Checking ZK compliance for DHEi2Q72..." },
      { t: 5500, type: "success", text: "✓ APPROVED — ZK proof valid. Proof hash: 0x9bc7b8162b8970..." },
      { t: 6500, type: "process", text: "Routing through Zerion API..." },
      { t: 7500, type: "success", text: "✓ Transaction submitted via Zerion. Explorer link generated." },

      { t: 9000, type: "info", text: "--- Request req-002 ---" },
      { t: 9500, type: "info", text: "From: UNVERIFI... | Amount: $500" },
      { t: 10500, type: "process", text: "Checking ZK compliance for UNVERIFI..." },
      { t: 12000, type: "error", text: "✗ REJECTED — Wallet UNVERIFI... has no valid ZK compliance proof" },
      { t: 12500, type: "error", text: "Blocked by: ZK_COMPLIANCE_FAILED" },

      { t: 14000, type: "info", text: "--- Request req-003 ---" },
      { t: 14500, type: "info", text: "From: DHEi2Q72... | Amount: $15000" },
      { t: 15500, type: "process", text: "Evaluating policy constraints..." },
      { t: 16000, type: "error", text: "✗ REJECTED — Amount $15000 exceeds per-tx limit $10000" },
      { t: 16500, type: "error", text: "Blocked by: SPEND_LIMIT_EXCEEDED" },

      { t: 18000, type: "success", text: "=== Agent Run Summary ===" },
      { t: 18500, type: "info", text: "Total requests: 3 | Approved: 1 | Rejected: 2" },
      { t: 19000, type: "process", text: "Returning to idle monitoring state..." },
    ];

    sequence.forEach((step, index) => {
      setTimeout(() => {
        setLogs((prev) => [
          ...prev, 
          { time: new Date().toLocaleTimeString(), type: step.type as any, text: step.text }
        ]);
        if (index === sequence.length - 1) setRunning(false);
      }, step.t);
    });
  };

  return (
    <div className="min-h-screen bg-[var(--background)] transition-colors duration-200 font-sans text-[var(--foreground)]">
      <ZNavbar />

      <main className="mx-auto max-w-5xl px-6 py-12 space-y-10">
        
        {/* Header */}
        <section className="flex flex-col md:flex-row justify-between items-start md:items-end gap-6">
          <div className="space-y-3">
            <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-blue-500/30 bg-blue-500/10 text-blue-400 text-xs font-mono">
              <span className="w-2 h-2 rounded-full bg-blue-400 animate-pulse"></span>
              Zerion Autonomous Execution
            </div>
            <h1 className="text-4xl md:text-5xl font-bold tracking-tight text-[var(--foreground)] font-space">
              Compliance <span className="bg-gradient-to-r from-blue-400 to-indigo-400 bg-clip-text text-transparent">Agent</span>
            </h1>
            <p className="text-gray-400 max-w-xl text-lg">
              A fully autonomous on-chain agent executing real RWA transfers via the Zerion API, strictly gated by Zero-Knowledge proofs.
            </p>
          </div>
          
          <button 
            onClick={runAgentSequence}
            disabled={running}
            className="px-6 py-3 rounded-xl font-bold text-sm bg-gradient-to-r from-blue-600 to-indigo-600 text-white hover:from-blue-500 hover:to-indigo-500 transition-all shadow-[0_0_20px_rgba(59,130,246,0.3)] disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            {running ? (
              <>
                <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24" fill="none">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                Processing Queue...
              </>
            ) : "Trigger Execution Cycle"}
          </button>
        </section>

        <div className="grid md:grid-cols-3 gap-6">
          
          {/* Left Column: Policy Config */}
          <div className="md:col-span-1 space-y-6">
            
            <div className="rounded-2xl border border-gray-800 bg-gray-900/60 p-6 space-y-6">
              <h3 className="text-xl font-bold font-space text-white border-b border-gray-800 pb-3">Active Policy</h3>
              
              <div className="space-y-4 font-mono text-sm">
                <div className="flex justify-between items-center">
                  <span className="text-gray-500">Require ZK Proof</span>
                  <span className="text-green-400 font-bold bg-green-500/10 px-2 py-1 rounded">TRUE</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-gray-500">Chain Lock</span>
                  <span className="text-white">Solana</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-gray-500">Max Spend / TX</span>
                  <span className="text-blue-400 font-bold">$10,000</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-gray-500">Max Proof Age</span>
                  <span className="text-white">30 Days</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-gray-500">Blocked Actions</span>
                  <span className="text-red-400">Bridging</span>
                </div>
              </div>
            </div>

            <div className="rounded-2xl border border-gray-800 bg-gray-900/60 p-6 space-y-4 line-clamp-none">
              <h3 className="text-lg font-bold font-space text-white">How it works</h3>
              <p className="text-gray-400 text-sm leading-relaxed">
                Traditional RWA systems rely on centralized compliance teams to approve transfers. 
              </p>
              <p className="text-gray-400 text-sm leading-relaxed">
                This agent replaces human compliance officers. It autonomously queries the Z-RWA oracle to verify the cryptographic ZK proof of the sender. Valid transactions are autonomously routed through the Zerion Swap API.
              </p>
            </div>
          </div>

          {/* Right Column: Terminal */}
          <div className="md:col-span-2 rounded-2xl border border-gray-800 bg-[#0d1117] overflow-hidden flex flex-col shadow-2xl">
            <div className="border-b border-gray-800 bg-gray-900/50 p-4 flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full bg-red-500"></div>
                <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
                <div className="w-3 h-3 rounded-full bg-green-500"></div>
              </div>
              <div className="font-mono text-xs text-gray-500">zerion-compliance-node ~ bash</div>
            </div>
            
            <div className="p-6 font-mono text-sm md:text-base space-y-2 h-[500px] overflow-y-auto">
              {logs.map((log, i) => (
                <div key={i} className="flex items-start gap-4 animate-in fade-in slide-in-from-bottom-2 duration-300">
                  <span className="text-gray-600 shrink-0 select-none">[{log.time}]</span>
                  <span className={`
                    ${log.type === "info" ? "text-blue-300" : ""}
                    ${log.type === "success" ? "text-green-400" : ""}
                    ${log.type === "error" ? "text-red-400" : ""}
                    ${log.type === "process" ? "text-gray-300" : ""}
                  `}>
                    {log.type === "error" ? <span className="font-bold mr-2 text-red-500">✖</span> : null}
                    {log.type === "success" ? <span className="font-bold mr-2 text-green-500">✔</span> : null}
                    {log.text}
                  </span>
                </div>
              ))}
              {running && (
                <div className="flex items-center gap-4 mt-2">
                  <span className="text-gray-600">[{new Date().toLocaleTimeString()}]</span>
                  <span className="text-yellow-400 animate-pulse">_</span>
                </div>
              )}
            </div>
          </div>

        </div>
      </main>
    </div>
  );
}
