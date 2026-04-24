"use client";

import { useState, useEffect, useCallback } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import ZNavbar from "../../components/ZNavbar";
import Link from "next/link";

// ── Types ────────────────────────────────────────────────────────────────────
interface VerifyResult {
  compliant: boolean;
  wallet: string;
  proof_hash: string | null;
  verified_at: string | null;
  expires_at: string | null;
  network: string;
  standard: string;
  message?: string;
}

type CheckState = "idle" | "loading" | "compliant" | "not_verified" | "error";

// ── Helpers ──────────────────────────────────────────────────────────────────
function isValidSolanaAddress(addr: string): boolean {
  if (!addr || addr.length < 32 || addr.length > 44) return false;
  return /^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(addr);
}

function timeAgo(iso: string): string {
  const diff = Date.now() - new Date(iso).getTime();
  const mins = Math.floor(diff / 60_000);
  const hours = Math.floor(mins / 60);
  const days = Math.floor(hours / 24);
  if (days > 0) return `${days} day${days === 1 ? "" : "s"} ago`;
  if (hours > 0) return `${hours} hour${hours === 1 ? "" : "s"} ago`;
  if (mins > 0) return `${mins} minute${mins === 1 ? "" : "s"} ago`;
  return "just now";
}

function timeFromNow(iso: string): string {
  const diff = new Date(iso).getTime() - Date.now();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));
  if (days > 0) return `${days} days from now`;
  const hours = Math.floor(diff / (1000 * 60 * 60));
  if (hours > 0) return `${hours} hours from now`;
  return "expiring soon";
}

function truncate(addr: string): string {
  return addr.length > 12 ? `${addr.slice(0, 6)}...${addr.slice(-4)}` : addr;
}

// ── Main Component ───────────────────────────────────────────────────────────
export default function CheckPage() {
  const { publicKey, connected } = useWallet();
  const [mounted, setMounted] = useState(false);
  const [input, setInput] = useState("");
  const [inputError, setInputError] = useState("");
  const [state, setState] = useState<CheckState>("idle");
  const [result, setResult] = useState<VerifyResult | null>(null);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Auto-fill with connected wallet
  useEffect(() => {
    if (connected && publicKey) {
      setInput(publicKey.toBase58());
    }
  }, [connected, publicKey]);

  const handleCheck = useCallback(async () => {
    const trimmed = input.trim();

    if (!isValidSolanaAddress(trimmed)) {
      setInputError("Invalid Solana address");
      return;
    }
    setInputError("");
    setState("loading");
    setResult(null);

    try {
      const res = await fetch(`/api/verify/${trimmed}`);
      const data: VerifyResult = await res.json();

      if (!res.ok) {
        setState("error");
        return;
      }

      setResult(data);
      setState(data.compliant ? "compliant" : "not_verified");
    } catch {
      setState("error");
    }
  }, [input]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") handleCheck();
  };

  const tweetUrl = result
    ? `https://twitter.com/intent/tweet?text=My+wallet+is+ZK-verified+on+Z-RWA+%E2%9C%93+No+identity+data+on-chain.+Check+yours+at+zrwa.vercel.app%2Fcheck+%23ZKProof+%23Solana+%23RWA`
    : "";

  const badgeMarkdown = result
    ? `![Z-RWA Verified](https://z-rwa.vercel.app/api/badge/${result.wallet})`
    : "";

  const copyBadge = async () => {
    await navigator.clipboard.writeText(badgeMarkdown);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-[var(--background)] transition-colors duration-200 font-sans text-[var(--foreground)]">
      <ZNavbar />

      <main className="mx-auto max-w-3xl px-6 py-16 space-y-16">
        {/* ── Header ──────────────────────────────────────────────────────── */}
        <section className="text-center space-y-4">
          <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-purple-500/30 bg-purple-500/10 text-purple-400 text-xs font-mono mb-2">
            Powered by Z-RWA · Solana Devnet
          </div>
          <h1 className="text-4xl md:text-5xl font-bold tracking-tight text-[var(--foreground)] font-space">
            Z-RWA{" "}
            <span className="bg-gradient-to-r from-purple-400 to-teal-400 bg-clip-text text-transparent">
              Compliance Checker
            </span>
          </h1>
          <p className="text-gray-400 text-lg leading-relaxed max-w-xl mx-auto">
            Check if any Solana wallet has a valid ZK compliance proof
          </p>
        </section>

        {/* ── Search Section ───────────────────────────────────────────────── */}
        <section className="rounded-2xl border border-gray-800 bg-gray-900/60 backdrop-blur-xl p-8 space-y-4">
          <div className="space-y-2">
            <input
              id="wallet-input"
              type="text"
              value={input}
              onChange={(e) => {
                setInput(e.target.value);
                if (inputError) setInputError("");
              }}
              onKeyDown={handleKeyDown}
              placeholder="Enter Solana wallet address (e.g. 7xKX...)"
              className={`w-full bg-gray-950 border rounded-xl px-4 py-4 text-white font-mono text-sm outline-none transition-all
                ${inputError
                  ? "border-red-500 focus:border-red-400"
                  : "border-gray-700 focus:border-purple-500/70"
                }`}
            />
            {inputError && (
              <p className="text-red-400 text-xs font-mono pl-1">{inputError}</p>
            )}
          </div>

          <div className="flex flex-col sm:flex-row gap-3">
            <button
              id="check-compliance-btn"
              onClick={handleCheck}
              disabled={state === "loading"}
              className="flex-1 py-3.5 rounded-xl font-bold text-sm bg-gradient-to-r from-purple-600 to-teal-500 text-white hover:from-purple-500 hover:to-teal-400 transition-all shadow-[0_0_20px_rgba(139,92,246,0.3)] hover:shadow-[0_0_30px_rgba(139,92,246,0.5)] disabled:opacity-50 disabled:cursor-wait"
            >
              {state === "loading" ? (
                <span className="flex items-center justify-center gap-2">
                  <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24" fill="none">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                  </svg>
                  Checking...
                </span>
              ) : (
                "Check Compliance →"
              )}
            </button>

            {mounted && (
              <div className="[&>button]:!bg-gray-800 [&>button]:!text-sm [&>button]:!font-semibold [&>button]:!text-white [&>button]:!border [&>button]:!border-gray-600 [&>button:hover]:!border-purple-400 [&>button]:!rounded-xl [&>button]:!h-[50px] [&>button]:!px-4 [&>button]:!transition-all">
                <WalletMultiButton />
              </div>
            )}
          </div>
        </section>

        {/* ── Result States ────────────────────────────────────────────────── */}

        {/* Loading skeleton */}
        {state === "loading" && (
          <div className="rounded-2xl border border-gray-800 bg-gray-900/60 p-8 animate-pulse space-y-4">
            <div className="h-6 bg-gray-700 rounded-lg w-1/3" />
            <div className="h-4 bg-gray-800 rounded-lg w-2/3" />
            <div className="h-4 bg-gray-800 rounded-lg w-1/2" />
            <div className="h-4 bg-gray-800 rounded-lg w-3/4" />
          </div>
        )}

        {/* Compliant */}
        {state === "compliant" && result && (
          <div className="rounded-2xl border border-green-500/40 bg-green-500/5 shadow-[0_0_30px_rgba(0,204,102,0.15)] p-8 space-y-6">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-green-500/20 border border-green-500/40 flex items-center justify-center text-green-400 text-xl">
                ✓
              </div>
              <div>
                <div className="text-green-400 font-bold text-lg font-space tracking-wide uppercase">
                  Compliant Wallet
                </div>
                <div className="text-green-600 text-xs font-mono">Z-RWA-v1 · Solana Devnet</div>
              </div>
            </div>

            <div className="bg-gray-950/80 rounded-xl border border-gray-800 divide-y divide-gray-800 font-mono text-sm">
              {[
                ["Wallet", truncate(result.wallet)],
                ["Verified", result.verified_at ? timeAgo(result.verified_at) : "—"],
                ["Proof Hash", result.proof_hash ? `${result.proof_hash.slice(0, 18)}...` : "—"],
                ["Expires", result.expires_at ? timeFromNow(result.expires_at) : "—"],
              ].map(([label, value]) => (
                <div key={label} className="flex justify-between px-5 py-3">
                  <span className="text-gray-500">{label}</span>
                  <span className="text-white">{value}</span>
                </div>
              ))}
            </div>

            <div className="flex flex-col sm:flex-row gap-3">
              <a
                href={tweetUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 py-2.5 rounded-xl text-center text-sm font-semibold bg-[#1DA1F2]/10 border border-[#1DA1F2]/30 text-[#1DA1F2] hover:bg-[#1DA1F2]/20 transition-all"
              >
                🐦 Share on Twitter
              </a>
              <button
                onClick={copyBadge}
                className="flex-1 py-2.5 rounded-xl text-sm font-semibold bg-gray-800 border border-gray-700 text-gray-300 hover:border-purple-400 hover:text-white transition-all"
              >
                {copied ? "✓ Copied!" : "Copy Badge Code"}
              </button>
            </div>
          </div>
        )}

        {/* Not verified */}
        {state === "not_verified" && result && (
          <div className="rounded-2xl border border-red-500/30 bg-red-500/5 p-8 space-y-6">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-red-500/20 border border-red-500/40 flex items-center justify-center text-red-400 text-xl">
                ✗
              </div>
              <div>
                <div className="text-red-400 font-bold text-lg font-space tracking-wide uppercase">
                  Not Verified
                </div>
                <div className="text-red-600 text-xs font-mono">No compliance proof found</div>
              </div>
            </div>

            <div className="bg-gray-950/80 rounded-xl border border-gray-800 divide-y divide-gray-800 font-mono text-sm">
              {[
                ["Wallet", truncate(result.wallet)],
                ["Status", "No compliance proof found"],
                ["Network", "Solana Devnet"],
              ].map(([label, value]) => (
                <div key={label} className="flex justify-between px-5 py-3">
                  <span className="text-gray-500">{label}</span>
                  <span className="text-white">{value}</span>
                </div>
              ))}
            </div>

            <Link
              href="/"
              className="block w-full py-3 rounded-xl text-center text-sm font-bold bg-gradient-to-r from-purple-600 to-teal-500 text-white hover:from-purple-500 hover:to-teal-400 transition-all shadow-[0_0_20px_rgba(139,92,246,0.3)]"
            >
              → Get Verified Now
            </Link>
          </div>
        )}

        {/* Error */}
        {state === "error" && (
          <div className="rounded-2xl border border-red-500/30 bg-red-500/5 p-6 text-center text-red-400 font-mono text-sm">
            Failed to check compliance. Please try again.
          </div>
        )}

        {/* ── How It Works ─────────────────────────────────────────────────── */}
        <section className="space-y-8">
          <h2 className="text-2xl font-bold text-center font-space text-[var(--foreground)]">
            How It Works
          </h2>
          <div className="grid md:grid-cols-3 gap-4">
            {[
              {
                step: "1",
                title: "Enter Wallet",
                desc: "Paste any Solana wallet address — or connect your wallet to auto-fill.",
                color: "purple",
              },
              {
                step: "2",
                title: "We Check",
                desc: "Query on-chain ZK proof state via Solana RPC. No personal data required.",
                color: "teal",
              },
              {
                step: "3",
                title: "Instant Result",
                desc: "See compliance status in under 2 seconds. Share or embed your badge.",
                color: "green",
              },
            ].map(({ step, title, desc, color }) => (
              <div
                key={step}
                className={`rounded-xl border p-6 space-y-3
                  ${color === "purple"
                    ? "border-purple-500/20 bg-purple-500/5"
                    : color === "teal"
                    ? "border-teal-500/20 bg-teal-500/5"
                    : "border-green-500/20 bg-green-500/5"
                  }`}
              >
                <div
                  className={`w-8 h-8 rounded-full text-sm font-bold flex items-center justify-center
                    ${color === "purple"
                      ? "bg-purple-500 text-white"
                      : color === "teal"
                      ? "bg-teal-500 text-white"
                      : "bg-green-500 text-black"
                    }`}
                >
                  {step}
                </div>
                <div className="font-semibold text-[var(--foreground)] font-space">{title}</div>
                <p className="text-gray-400 text-sm leading-relaxed">{desc}</p>
              </div>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}
