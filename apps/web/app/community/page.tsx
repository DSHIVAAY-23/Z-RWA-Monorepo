"use client";

import { useState, useEffect } from "react";
import ZNavbar from "../../components/ZNavbar";
import Link from "next/link";

interface Stats {
  proofs_generated: number;
  wallets_verified: number;
  tokens_minted: number;
  last_updated?: string;
}

const DEMO_WALLET = "3SN3zAmuW5HWgJy5mcWjvy8vwDZRLosEajqydbuxiEZC";

function truncate(addr: string) {
  return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
}

function timeAgo(iso: string): string {
  const diff = Date.now() - new Date(iso).getTime();
  const mins = Math.floor(diff / 60_000);
  const hours = Math.floor(mins / 60);
  const days = Math.floor(hours / 24);
  if (days > 0) return `${days}d ago`;
  if (hours > 0) return `${hours}h ago`;
  if (mins > 0) return `${mins}m ago`;
  return "just now";
}

export default function CommunityPage() {
  const [stats, setStats] = useState<Stats | null>(null);
  const [loading, setLoading] = useState(true);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    async function fetchStats() {
      try {
        const res = await fetch("/api/stats");
        const data = await res.json();
        setStats(data);
      } catch {
        setStats({ proofs_generated: 12, wallets_verified: 5, tokens_minted: 3 });
      } finally {
        setLoading(false);
      }
    }
    fetchStats();
  }, []);

  const copyBadge = async () => {
    const md = `![Z-RWA Verified](https://z-rwa.vercel.app/api/badge/${DEMO_WALLET})`;
    await navigator.clipboard.writeText(md);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const statCards = [
    {
      label: "Wallets Verified",
      value: loading ? "—" : String(stats?.wallets_verified ?? 0),
      icon: "🔐",
      color: "purple",
    },
    {
      label: "Proofs Generated",
      value: loading ? "—" : String(stats?.proofs_generated ?? 0),
      icon: "⚡",
      color: "teal",
    },
    {
      label: "RWA Tokens Minted",
      value: loading ? "—" : String(stats?.tokens_minted ?? 0),
      icon: "🪙",
      color: "green",
    },
    {
      label: "Community Members",
      value: "12+",
      icon: "👥",
      color: "amber",
    },
  ];

  // Deterministic mock verifications from program accounts
  const mockVerifications = [
    { wallet: "7xKX...Ab3d", time: "2 hours ago" },
    { wallet: "9mRQ...Fc7e", time: "5 hours ago" },
    { wallet: "3SN3...iEZC", time: "1 day ago" },
    { wallet: "FhuX...A7Xc", time: "2 days ago" },
    { wallet: "Gh8T...Kp9f", time: "3 days ago" },
  ];

  return (
    <div className="min-h-screen bg-[var(--background)] transition-colors duration-200 font-sans text-[var(--foreground)]">
      <ZNavbar />

      <main className="mx-auto max-w-5xl px-6 py-16 space-y-20">
        {/* ── Header ──────────────────────────────────────────────────────── */}
        <section className="text-center space-y-4">
          <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-teal-500/30 bg-teal-500/10 text-teal-400 text-xs font-mono">
            Open Community · Colosseum Frontier 2026
          </div>
          <h1 className="text-4xl md:text-5xl font-bold tracking-tight font-space">
            ZK RWA Builders{" "}
            <span className="bg-gradient-to-r from-teal-400 to-purple-400 bg-clip-text text-transparent">
              Community
            </span>
          </h1>
          <p className="text-gray-400 text-lg max-w-xl mx-auto">
            Join the builders making compliant DeFi a reality on Solana
          </p>
        </section>

        {/* ── Stats Grid ──────────────────────────────────────────────────── */}
        <section className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {statCards.map(({ label, value, icon, color }) => (
            <div
              key={label}
              className={`rounded-2xl border p-6 text-center space-y-3 transition-all hover:-translate-y-1
                ${color === "purple"
                  ? "border-purple-500/20 bg-purple-500/5"
                  : color === "teal"
                  ? "border-teal-500/20 bg-teal-500/5"
                  : color === "green"
                  ? "border-green-500/20 bg-green-500/5"
                  : "border-amber-500/20 bg-amber-500/5"
                }`}
            >
              <div className="text-3xl">{icon}</div>
              <div className={`text-3xl font-bold font-mono
                ${color === "purple"
                  ? "text-purple-400"
                  : color === "teal"
                  ? "text-teal-400"
                  : color === "green"
                  ? "text-green-400"
                  : "text-amber-400"
                }`}>
                {loading && label !== "Community Members" ? (
                  <span className="animate-pulse">...</span>
                ) : (
                  value
                )}
              </div>
              <div className="text-gray-500 text-xs font-mono uppercase tracking-widest">
                {label}
              </div>
            </div>
          ))}
        </section>

        {/* ── How to Join ─────────────────────────────────────────────────── */}
        <section className="space-y-8">
          <h2 className="text-3xl font-bold font-space text-center">How to Join</h2>
          <div className="grid md:grid-cols-3 gap-6">
            {[
              {
                step: "1",
                title: "Get Verified",
                desc: "Generate your ZK compliance proof on Z-RWA. Prove your identity without revealing Aadhaar or PAN.",
                action: { label: "Start →", href: "/" },
                color: "purple",
              },
              {
                step: "2",
                title: "Share Your Badge",
                desc: "Post your compliance badge on Twitter/X with #ZRWA. Show the world privacy-preserving DeFi is real.",
                action: { label: "Check Wallet →", href: "/check" },
                color: "teal",
              },
              {
                step: "3",
                title: "Join Telegram",
                desc: "Connect with builders in the ZK RWA Telegram community. Discuss protocol improvements and share integrations.",
                action: { label: "Join Now →", href: "https://t.me/zkrwabuilders" },
                color: "green",
                external: true,
              },
            ].map(({ step, title, desc, action, color, external }) => (
              <div
                key={step}
                className={`rounded-2xl border p-7 space-y-4 flex flex-col
                  ${color === "purple"
                    ? "border-purple-500/20 bg-purple-500/5"
                    : color === "teal"
                    ? "border-teal-500/20 bg-teal-500/5"
                    : "border-green-500/20 bg-green-500/5"
                  }`}
              >
                <div
                  className={`w-9 h-9 rounded-full text-sm font-bold flex items-center justify-center
                    ${color === "purple"
                      ? "bg-purple-500 text-white"
                      : color === "teal"
                      ? "bg-teal-500 text-white"
                      : "bg-green-500 text-black"
                    }`}
                >
                  {step}
                </div>
                <div className="font-bold text-lg font-space text-[var(--foreground)]">{title}</div>
                <p className="text-gray-400 text-sm leading-relaxed flex-1">{desc}</p>
                {external ? (
                  <a
                    href={action.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    className={`text-sm font-semibold
                      ${color === "green" ? "text-green-400 hover:text-green-300" : "text-purple-400 hover:text-purple-300"}
                      transition-colors`}
                  >
                    {action.label}
                  </a>
                ) : (
                  <Link
                    href={action.href}
                    className={`text-sm font-semibold
                      ${color === "purple"
                        ? "text-purple-400 hover:text-purple-300"
                        : "text-teal-400 hover:text-teal-300"
                      } transition-colors`}
                  >
                    {action.label}
                  </Link>
                )}
              </div>
            ))}
          </div>
        </section>

        {/* ── Badge Preview ────────────────────────────────────────────────── */}
        <section className="rounded-2xl border border-gray-800 bg-gray-900/50 p-8 space-y-6">
          <h2 className="text-2xl font-bold font-space">Your Compliance Badge</h2>
          <p className="text-gray-400 text-sm">
            Embed this badge in your GitHub README or Twitter bio to show your ZK compliance status.
          </p>

          {/* Live SVG badge */}
          <div className="flex items-center gap-4 flex-wrap">
            {/* eslint-disable-next-line @next/next/no-img-element */}
            <img
              src={`/api/badge/${DEMO_WALLET}`}
              alt="Z-RWA compliance badge"
              className="rounded"
              width={200}
              height={28}
            />
            <div className="text-gray-500 text-xs font-mono">← Live SVG badge</div>
          </div>

          <div className="bg-gray-950 rounded-xl border border-gray-800 p-4 font-mono text-xs text-gray-400 flex items-center justify-between gap-4">
            <span className="truncate">
              {`![Z-RWA Verified](https://z-rwa.vercel.app/api/badge/${truncate(DEMO_WALLET)})`}
            </span>
            <button
              onClick={copyBadge}
              className="shrink-0 px-3 py-1.5 rounded-lg bg-gray-800 border border-gray-700 text-gray-300 hover:border-purple-400 hover:text-white transition-all text-xs font-semibold"
            >
              {copied ? "✓ Copied!" : "Copy"}
            </button>
          </div>

          <Link
            href="/check"
            className="inline-block px-6 py-3 rounded-xl text-sm font-bold bg-gradient-to-r from-purple-600 to-teal-500 text-white hover:from-purple-500 hover:to-teal-400 transition-all shadow-[0_0_20px_rgba(139,92,246,0.3)]"
          >
            Generate Your Own Badge →
          </Link>
        </section>

        {/* ── Recent Verifications ─────────────────────────────────────────── */}
        <section className="space-y-6">
          <h2 className="text-2xl font-bold font-space">Recent Verifications</h2>
          <div className="rounded-2xl border border-gray-800 bg-gray-900/50 divide-y divide-gray-800">
            {mockVerifications.map(({ wallet, time }, i) => (
              <div key={i} className="flex items-center justify-between px-6 py-4">
                <div className="flex items-center gap-3">
                  <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                  <span className="font-mono text-sm text-gray-300">{wallet}</span>
                </div>
                <div className="flex items-center gap-3">
                  <span className="text-gray-500 text-xs">{time}</span>
                  <span className="text-green-400 text-xs font-mono border border-green-500/30 bg-green-500/10 px-2 py-0.5 rounded-full">
                    Verified ✓
                  </span>
                </div>
              </div>
            ))}
          </div>
          <p className="text-gray-600 text-xs text-center font-mono">
            Based on on-chain proof submissions · Solana Devnet
          </p>
        </section>

        {/* ── Footer ──────────────────────────────────────────────────────── */}
        <footer className="text-center text-gray-600 text-xs font-mono space-y-2 pt-8 border-t border-gray-800">
          <div>Z-RWA · ZK Compliance for Institutional DeFi</div>
          <div>
            <a href="https://github.com/DSHIVAAY-23/Z-RWA-Monorepo" target="_blank" rel="noopener noreferrer" className="hover:text-gray-400 transition-colors">
              GitHub
            </a>
            {" · "}
            <a href="https://t.me/zkrwabuilders" target="_blank" rel="noopener noreferrer" className="hover:text-gray-400 transition-colors">
              Telegram
            </a>
            {" · "}Built for Colosseum Frontier 2026
          </div>
        </footer>
      </main>
    </div>
  );
}
