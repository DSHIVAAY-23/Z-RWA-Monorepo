'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';

// ── Types ────────────────────────────────────────────────────────────────────
interface Stats {
  proofs_generated: number;
  wallets_verified: number;
  tokens_minted: number;
}

// ── Constants ─────────────────────────────────────────────────────────────────
const VERIFIER_PROGRAM  = 'GL8vm2SxWV7yHQbwoZegM7SkbJbEbEDn6A9m9W2XjeQe';
const MINT_ADDRESS      = '8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa';
const AUTHORITY_ADDRESS = 'GsPrDLXoqVbcWwofYpRZFJg4h5dzHEjyNfPyzPrcUKGd';

const explorerUrl = (address: string) =>
  `https://explorer.solana.com/address/${address}?cluster=devnet`;

// ── Deployment row ─────────────────────────────────────────────────────────────
function DeployRow({ label, address }: { label: string; address: string }) {
  const [copied, setCopied] = useState(false);
  const copy = () => {
    navigator.clipboard.writeText(address);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };
  return (
    <div className="flex flex-col sm:flex-row sm:items-center gap-1 sm:gap-3 py-3 border-b border-gray-800 last:border-0">
      <span className="text-gray-400 text-xs uppercase tracking-widest w-40 shrink-0">{label}</span>
      <div className="flex items-center gap-2 overflow-hidden">
        <a
          href={explorerUrl(address)}
          target="_blank"
          rel="noopener noreferrer"
          className="font-mono text-xs text-purple-400 hover:text-purple-300 underline underline-offset-2 truncate"
        >
          {address}
        </a>
        <button
          onClick={copy}
          title="Copy address"
          className="shrink-0 text-gray-600 hover:text-gray-300 transition-colors"
        >
          {copied ? (
            <svg className="w-3.5 h-3.5 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
          ) : (
            <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
          )}
        </button>
      </div>
    </div>
  );
}

// ── Stat counter card ──────────────────────────────────────────────────────────
function StatCard({ value, label, color }: { value: number | null; label: string; color: string }) {
  return (
    <div className="flex flex-col items-center justify-center p-8 rounded-2xl border border-gray-800 bg-gray-950/60">
      <div className={`text-5xl font-bold font-mono mb-2 ${color}`}>
        {value === null ? '—' : value.toLocaleString()}
      </div>
      <div className="text-xs text-gray-500 uppercase tracking-widest font-mono">{label}</div>
    </div>
  );
}

// ── Main page ──────────────────────────────────────────────────────────────────
export default function SubmissionPage() {
  const [stats, setStats] = useState<Stats | null>(null);

  useEffect(() => {
    fetch('/api/stats')
      .then((r) => r.json())
      .then((d) => setStats(d))
      .catch(() => {});
  }, []);

  return (
    <main className="min-h-screen bg-[#030712] text-white">
      {/* ── HEADER ── */}
      <header className="relative border-b border-gray-800/60 bg-gray-950/80 backdrop-blur-md">
        <div className="max-w-5xl mx-auto px-6 py-8 flex flex-col items-center gap-3 text-center">
          {/* Logo */}
          <div className="flex items-center gap-2 mb-1">
            <div className="w-9 h-9 rounded-lg bg-gradient-to-br from-purple-600 to-green-500 flex items-center justify-center font-bold text-sm">
              Z
            </div>
            <span className="text-xl font-bold tracking-tight">Z-RWA</span>
          </div>

          {/* Hackathon badge */}
          <span className="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-purple-500/40 bg-purple-500/10 text-purple-300 text-xs font-mono uppercase tracking-widest">
            <span className="w-1.5 h-1.5 rounded-full bg-purple-400 animate-pulse" />
            Colosseum Frontier 2026 — Hackathon Submission
          </span>

          {/* Tagline */}
          <p className="text-gray-300 text-base md:text-lg max-w-2xl leading-relaxed">
            Privacy-preserving KYC for Indian RWA — ZK proof on Solana,{' '}
            <span className="text-green-400 font-semibold">identity never leaves your device</span>
          </p>
        </div>
      </header>

      <div className="max-w-5xl mx-auto px-6 py-14 space-y-14">

        {/* ── PROBLEM ── */}
        <section>
          <div className="rounded-2xl border border-gray-800 bg-gray-900/50 p-8 md:p-10">
            <div className="flex items-center gap-3 mb-5">
              <span className="text-2xl">⚠️</span>
              <h2 className="text-xl font-bold text-white">The Problem</h2>
            </div>
            <p className="text-gray-300 leading-relaxed text-base md:text-lg">
              India&apos;s <span className="text-purple-400 font-semibold">DPDP Act</span> mandates institutions prove compliance without
              storing raw Aadhaar/PAN data. Existing solutions force a binary choice:{' '}
              <span className="text-red-400">full privacy (no verification)</span> or{' '}
              <span className="text-red-400">full disclosure (no privacy)</span>.{' '}
              <span className="text-green-400 font-semibold">Z-RWA eliminates this tradeoff entirely.</span>
            </p>
          </div>
        </section>

        {/* ── HOW IT WORKS ── */}
        <section>
          <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <span className="text-2xl">⚙️</span> How It Works
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            {[
              {
                step: '01',
                title: 'Upload Document',
                body: 'Client-side OCR extracts identity fields. Raw data never leaves browser.',
                icon: '📄',
                color: 'border-blue-500/30 bg-blue-500/5',
                accent: 'text-blue-400',
              },
              {
                step: '02',
                title: 'Generate ZK Proof',
                body: 'Groth16 circuit verifies age ≥ 18 and KYC score ≥ threshold. 7.4M constraints processed in ~1-2s via WASM.',
                icon: '🔐',
                color: 'border-purple-500/30 bg-purple-500/5',
                accent: 'text-purple-400',
              },
              {
                step: '03',
                title: 'On-Chain Verify',
                body: 'Custom Anchor program (z_rwa_verifier) verifies proof on Solana Devnet. No raw data touches the chain.',
                icon: '✅',
                color: 'border-green-500/30 bg-green-500/5',
                accent: 'text-green-400',
              },
              {
                step: '04',
                title: 'Receive Token',
                body: 'Token2022 compliance credential minted to wallet. Transferable proof of verified status.',
                icon: '🏅',
                color: 'border-yellow-500/30 bg-yellow-500/5',
                accent: 'text-yellow-400',
              },
            ].map(({ step, title, body, icon, color, accent }) => (
              <div key={step} className={`rounded-2xl border p-6 flex flex-col gap-3 ${color}`}>
                <div className="flex items-center justify-between">
                  <span className={`font-mono text-xs font-bold uppercase tracking-widest ${accent}`}>Step {step}</span>
                  <span className="text-2xl">{icon}</span>
                </div>
                <h3 className="font-semibold text-white text-sm">{title}</h3>
                <p className="text-gray-400 text-xs leading-relaxed">{body}</p>
              </div>
            ))}
          </div>
        </section>

        {/* ── TECHNICAL ACHIEVEMENTS ── */}
        <section>
          <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <span className="text-2xl">🏆</span> Technical Achievements
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
            {[
              {
                icon: '⚡',
                title: 'Real Groth16 Proofs',
                body: 'Production SnarkJS pipeline with VKey caching. Zero mock data.',
                accent: 'from-purple-600/20 to-purple-800/10 border-purple-500/30',
              },
              {
                icon: '🔢',
                title: '7.4M Constraints',
                body: 'Cryptographic soundness without sacrificing speed. ~1-2s proof generation via WASM optimisation.',
                accent: 'from-green-600/20 to-green-800/10 border-green-500/30',
              },
              {
                icon: '🤖',
                title: 'Autonomous Agent',
                body: 'Zerion-powered compliance agent gates RWA transfers via ZK proof verification.',
                accent: 'from-blue-600/20 to-blue-800/10 border-blue-500/30',
              },
            ].map(({ icon, title, body, accent }) => (
              <div key={title} className={`rounded-2xl border bg-gradient-to-br p-7 flex flex-col gap-3 ${accent}`}>
                <span className="text-3xl">{icon}</span>
                <h3 className="font-bold text-white">{title}</h3>
                <p className="text-gray-400 text-sm leading-relaxed">{body}</p>
              </div>
            ))}
          </div>
        </section>

        {/* ── DEPLOYMENT ── */}
        <section>
          <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <span className="text-2xl">🚀</span> Deployment Details
          </h2>
          <div className="rounded-2xl border border-gray-800 bg-gray-950/60 p-6 md:p-8">
            <DeployRow label="Verifier Program" address={VERIFIER_PROGRAM} />
            <DeployRow label="RWA Compliance Mint" address={MINT_ADDRESS} />
            <DeployRow label="Backend Authority" address={AUTHORITY_ADDRESS} />
            <div className="flex flex-col sm:flex-row sm:items-center gap-1 sm:gap-3 pt-3">
              <span className="text-gray-400 text-xs uppercase tracking-widest w-40 shrink-0">Network</span>
              <span className="font-mono text-xs text-green-400 font-semibold">Solana Devnet</span>
            </div>
          </div>
        </section>

        {/* ── LIVE STATS ── */}
        <section>
          <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <span className="text-2xl">📊</span> Live Protocol Stats
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
            <StatCard value={stats?.proofs_generated ?? null} label="Proofs Generated" color="text-purple-400" />
            <StatCard value={stats?.wallets_verified ?? null} label="Wallets Verified" color="text-green-400" />
            <StatCard value={stats?.tokens_minted ?? null} label="Tokens Minted" color="text-blue-400" />
          </div>
          <p className="text-center text-xs text-gray-600 mt-3 font-mono">
            Live data from /api/stats — updates with every real proof generated
          </p>
        </section>

        {/* ── CTA ── */}
        <section className="flex flex-col sm:flex-row gap-4 justify-center pt-2">
          <Link
            href="/"
            className="inline-flex items-center justify-center gap-2 px-7 py-3 rounded-xl bg-purple-600 hover:bg-purple-500 text-white font-semibold transition-colors text-sm"
          >
            🎯 View Live Demo
          </Link>
          <a
            href="https://github.com/DSHIVAAY-23/Z-RWA-Monorepo"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center justify-center gap-2 px-7 py-3 rounded-xl border border-gray-700 hover:border-gray-500 bg-gray-900 hover:bg-gray-800 text-white font-semibold transition-colors text-sm"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z" />
            </svg>
            GitHub Repo
          </a>
          <Link
            href="/agent"
            className="inline-flex items-center justify-center gap-2 px-7 py-3 rounded-xl border border-green-600/50 hover:border-green-500 bg-green-600/10 hover:bg-green-600/20 text-green-400 font-semibold transition-colors text-sm"
          >
            🤖 Try Agent UI
          </Link>
        </section>

      </div>

      {/* ── FOOTER ── */}
      <footer className="border-t border-gray-800/60 mt-10">
        <div className="max-w-5xl mx-auto px-6 py-6 text-center">
          <p className="text-gray-600 text-sm font-mono">
            Built for Colosseum Frontier 2026 |{' '}
            <span className="text-purple-500">Z-RWA Protocol</span> |{' '}
            India 🇮🇳
          </p>
        </div>
      </footer>
    </main>
  );
}
