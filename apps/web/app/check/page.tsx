"use client";

import { useState, useEffect, useCallback } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import ZNavbar from "../../components/ZNavbar";
import { getWalletTransactions, getWalletTokenBalances } from "../../lib/goldrush";

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

  // GoldRush state
  const [goldrushBalances, setGoldrushBalances] = useState<any[]>([]);
  const [goldrushTransactions, setGoldrushTransactions] = useState<any[]>([]);
  const [isGoldRushLoading, setIsGoldRushLoading] = useState(false);

  // Dashboard analytics state
  const [dashboardData, setDashboardData] = useState<any>(null);
  const [isDashboardLoading, setIsDashboardLoading] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Auto-fill and auto-check on wallet connect
  useEffect(() => {
    if (connected && publicKey) {
      const addr = publicKey.toBase58();
      setInput(addr);
      handleCheck(addr);
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [connected, publicKey]);

  const handleCheck = useCallback(async (overrideAddr?: string) => {
    const trimmed = (overrideAddr || input).trim();

    if (!isValidSolanaAddress(trimmed)) {
      setInputError("Invalid Solana address");
      return;
    }
    setInputError("");
    setState("loading");
    setResult(null);

    try {
      setIsGoldRushLoading(true);
      setIsDashboardLoading(true);

      const [verifyRes, transactions, balances, dashRes] = await Promise.all([
        fetch(`/api/verify/${trimmed}`),
        getWalletTransactions(trimmed),
        getWalletTokenBalances(trimmed),
        fetch(`/api/dashboard?wallet=${trimmed}`)
      ]);

      const data: VerifyResult = await verifyRes.json();

      if (!verifyRes.ok) {
        setState("error");
        setIsGoldRushLoading(false);
        setIsDashboardLoading(false);
        return;
      }

      const dash = await dashRes.json();
      setResult(data);
      setGoldrushTransactions(transactions);
      setGoldrushBalances(balances);
      setDashboardData(dash);
      setState(data.compliant ? "compliant" : "not_verified");
      setIsGoldRushLoading(false);
      setIsDashboardLoading(false);
    } catch (err) {
      console.error("[Check] Fetch error:", err);
      setState("error");
      setIsGoldRushLoading(false);
      setIsDashboardLoading(false);
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
              onClick={() => handleCheck()}
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

            <a
              href="/#compliance-flow"
              className="block w-full py-3 rounded-xl text-center text-sm font-bold bg-gradient-to-r from-purple-600 to-teal-500 text-white hover:from-purple-500 hover:to-teal-400 transition-all shadow-[0_0_20px_rgba(139,92,246,0.3)]"
            >
              Generate ZK Proof →
            </a>
          </div>
        )}

        {/* Error */}
        {state === "error" && (
          <div className="rounded-2xl border border-red-500/30 bg-red-500/5 p-6 text-center text-red-400 font-mono text-sm">
            Failed to check compliance. Please try again.
          </div>
        )}

        {/* ── GoldRush Data Sections ───────────────────────────────────────── */}
        {(state === "compliant" || state === "not_verified") && (
          <div className="space-y-8">
            {/* Section 0: Wallet Analytics */}
            <section className="rounded-2xl border border-gray-800 bg-gray-900/60 backdrop-blur-xl p-8 space-y-6">
              <div className="flex items-center justify-between">
                <h3 className="text-xl font-bold font-space text-[var(--foreground)]">Wallet Analytics</h3>
                <div className="text-[10px] font-mono text-gray-500 uppercase tracking-widest px-2 py-1 bg-gray-950 rounded border border-gray-800">
                  Live Data
                </div>
              </div>

              {isDashboardLoading ? (
                <div className="animate-pulse space-y-3">
                  <div className="h-12 bg-gray-800 rounded-xl w-full" />
                  <div className="h-12 bg-gray-800 rounded-xl w-full" />
                </div>
              ) : (
                <>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="p-4 bg-gray-950/50 rounded-xl border border-gray-800/50 text-center">
                      <div className="text-xs text-gray-500 font-mono uppercase mb-1">Tokens Owned</div>
                      <div className="text-2xl font-bold text-teal-400">{dashboardData?.stats?.tokens_owned ?? 0}</div>
                      <div className="text-[10px] text-gray-600 font-mono mt-1">Z-RWA-COMPLY</div>
                    </div>
                    <div className="p-4 bg-gray-950/50 rounded-xl border border-gray-800/50 text-center">
                      <div className="text-xs text-gray-500 font-mono uppercase mb-1">Payments Done</div>
                      <div className="text-2xl font-bold text-purple-400">{dashboardData?.stats?.payments_done ?? 0}</div>
                      <div className="text-[10px] text-gray-600 font-mono mt-1">via Dodo</div>
                    </div>
                  </div>

                  {dashboardData?.transactions?.length > 0 && (
                    <div className="overflow-hidden bg-gray-950/50 rounded-xl border border-gray-800/50">
                      <div className="overflow-x-auto">
                        <table className="w-full text-left text-xs font-mono">
                          <thead className="bg-gray-900/50 text-gray-500 uppercase border-b border-gray-800">
                            <tr>
                              <th className="px-4 py-3">Payment ID</th>
                              <th className="px-4 py-3">Amount</th>
                              <th className="px-4 py-3 text-right">Status</th>
                            </tr>
                          </thead>
                          <tbody className="divide-y divide-gray-800/50">
                            {dashboardData.transactions.slice(0, 3).map((tx: any, i: number) => (
                              <tr key={i} className="hover:bg-white/5 transition-colors">
                                <td className="px-4 py-3 text-gray-300 truncate max-w-[140px]">
                                  {tx.payment_id?.slice(0, 16)}...
                                </td>
                                <td className="px-4 py-3 text-teal-400">{tx.amount}</td>
                                <td className="px-4 py-3 text-right">
                                  <span className={`px-2 py-0.5 rounded-full text-[9px] font-bold uppercase ${
                                    tx.status === "COMPLETE"
                                      ? "bg-green-500/10 text-green-400"
                                      : "bg-yellow-500/10 text-yellow-400"
                                  }`}>
                                    {tx.status}
                                  </span>
                                </td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    </div>
                  )}
                </>
              )}
            </section>

            {/* Section 1: Token Balances */}
            <section className="rounded-2xl border border-gray-800 bg-gray-900/60 backdrop-blur-xl p-8 space-y-6">
              <div className="flex items-center justify-between">
                <h3 className="text-xl font-bold font-space text-[var(--foreground)]">Token Balances</h3>
                <div className="text-[10px] font-mono text-gray-500 uppercase tracking-widest px-2 py-1 bg-gray-950 rounded border border-gray-800">
                  Powered by GoldRush API
                </div>
              </div>

              {isGoldRushLoading ? (
                <div className="animate-pulse space-y-3">
                  <div className="h-12 bg-gray-800 rounded-xl w-full" />
                  <div className="h-12 bg-gray-800 rounded-xl w-full" />
                </div>
              ) : (
                <div className="grid gap-3">
                  {goldrushBalances.length > 0 ? (
                    goldrushBalances.map((item: any, i: number) => (
                      <div key={i} className="flex items-center justify-between p-4 bg-gray-950/50 rounded-xl border border-gray-800/50">
                        <div className="flex items-center gap-3">
                          {item.logo_url && (
                            <img src={item.logo_url} alt={item.contract_name} className="w-6 h-6 rounded-full" />
                          )}
                          <div>
                            <div className="text-sm font-bold text-white">{item.contract_ticker_symbol}</div>
                            <div className="text-[10px] text-gray-500 uppercase">{item.contract_name}</div>
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="text-sm font-mono text-teal-400 font-bold">
                            {(Number(item.balance) / Math.pow(10, item.contract_decimals)).toLocaleString(undefined, { maximumFractionDigits: 4 })}
                          </div>
                          <div className="text-[10px] text-gray-500 font-mono">
                            ${Number(item.quote).toLocaleString(undefined, { maximumFractionDigits: 2 })}
                          </div>
                        </div>
                      </div>
                    ))
                  ) : (
                    <div className="text-center py-6 text-gray-500 text-sm font-mono">
                      No tokens found in this wallet.
                    </div>
                  )}
                </div>
              )}
              <div className="text-[9px] text-gray-600 font-mono text-right">
                GoldRush Covalent SDK · Mainnet-Beta
              </div>
            </section>

            {/* Section 2: Transaction History */}
            <section className="rounded-2xl border border-gray-800 bg-gray-900/60 backdrop-blur-xl p-8 space-y-6">
              <div className="flex items-center justify-between">
                <h3 className="text-xl font-bold font-space text-[var(--foreground)]">Recent Activity</h3>
                <div className="text-[10px] font-mono text-gray-500 uppercase tracking-widest px-2 py-1 bg-gray-950 rounded border border-gray-800">
                  Powered by GoldRush API
                </div>
              </div>

              {isGoldRushLoading ? (
                <div className="animate-pulse space-y-3">
                  <div className="h-20 bg-gray-800 rounded-xl w-full" />
                  <div className="h-20 bg-gray-800 rounded-xl w-full" />
                </div>
              ) : (
                <div className="overflow-hidden bg-gray-950/50 rounded-xl border border-gray-800/50">
                  <div className="overflow-x-auto">
                    <table className="w-full text-left text-xs font-mono">
                      <thead className="bg-gray-900/50 text-gray-500 uppercase border-b border-gray-800">
                        <tr>
                          <th className="px-5 py-3">Transaction</th>
                          <th className="px-5 py-3">Date</th>
                          <th className="px-5 py-3 text-right">Status</th>
                        </tr>
                      </thead>
                      <tbody className="divide-y divide-gray-800/50">
                        {goldrushTransactions.length > 0 ? (
                          goldrushTransactions.map((tx: any, i: number) => (
                            <tr key={i} className="hover:bg-white/5 transition-colors">
                              <td className="px-5 py-4">
                                <a 
                                  href={`https://explorer.solana.com/tx/${tx.tx_hash}`} 
                                  target="_blank" 
                                  rel="noreferrer"
                                  className="text-teal-400 hover:underline flex flex-col gap-0.5"
                                >
                                  <span>{tx.tx_hash.slice(0, 8)}...{tx.tx_hash.slice(-8)}</span>
                                  <span className="text-[9px] text-gray-500 uppercase">{tx.log_events?.[0]?.sender_contract_ticker_symbol || 'SOL'} Transfer</span>
                                </a>
                              </td>
                              <td className="px-5 py-4 text-gray-400">
                                {new Date(tx.block_signed_at).toLocaleDateString()}
                              </td>
                              <td className="px-5 py-4 text-right">
                                <span className={`px-2 py-0.5 rounded-full text-[9px] font-bold uppercase ${
                                  tx.successful ? 'bg-green-500/10 text-green-400' : 'bg-red-500/10 text-red-400'
                                }`}>
                                  {tx.successful ? 'Success' : 'Failed'}
                                </span>
                              </td>
                            </tr>
                          ))
                        ) : (
                          <tr>
                            <td colSpan={3} className="px-5 py-10 text-center text-gray-500">
                              No recent transactions found.
                            </td>
                          </tr>
                        )}
                      </tbody>
                    </table>
                  </div>
                </div>
              )}
              <div className="text-[9px] text-gray-600 font-mono text-right">
                GoldRush Covalent SDK · solana-mainnet
              </div>
            </section>
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
