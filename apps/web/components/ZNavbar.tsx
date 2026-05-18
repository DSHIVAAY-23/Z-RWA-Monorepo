"use client";

import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

export default function ZNavbar() {
  const { connected } = useWallet();
  const [mounted, setMounted] = useState(false);
  const pathname = usePathname();

  useEffect(() => {
    setMounted(true);
  }, []);

  const navLinks = [
    { href: "/#compliance-flow", label: "Generate Proof", primary: true },
    { href: "/invest", label: "Invest", primary: false },
    { href: "/check", label: "Check Wallet", primary: false },
    { href: "/community", label: "Community", primary: false },
    { href: "/agent", label: "Agent UI", primary: false },
  ];

  return (
    <header className="sticky top-0 z-50 border-b border-gray-200 dark:border-gray-800 bg-[var(--background)]/90 backdrop-blur-xl transition-colors duration-200">
      <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
        {/* Left Side (Brand) */}
        <div className="flex items-center gap-6">
          <Link href="/" className="flex items-center gap-3">
            <div className="relative flex h-9 w-9 items-center justify-center overflow-hidden rounded-lg">
              <img src="/z-rwa-logo.png" alt="Z-RWA Logo" className="object-cover w-full h-full" />
            </div>
            <span className="text-xl font-bold tracking-tight text-[var(--foreground)] font-space">
              Z-RWA <span className="text-emerald-400">Compliance</span>
            </span>
          </Link>

          {/* Nav Links */}
          <nav className="hidden md:flex items-center gap-1">
            {navLinks.map(({ href, label, primary }) => (
              <Link
                key={href}
                href={href}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors
                  ${primary
                    ? "text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 hover:bg-emerald-500/20 hover:text-emerald-300"
                    : pathname === href
                    ? "text-emerald-400 bg-emerald-500/10 border border-emerald-500/20"
                    : "text-gray-500 dark:text-gray-400 hover:text-[var(--foreground)] hover:bg-gray-100 dark:hover:bg-gray-800"
                  }`}
              >
                {label}
              </Link>
            ))}
          </nav>
        </div>

        {/* Right Side (Controls) */}
        <div className="flex items-center gap-4">
          {/* Network Badge */}
          <div className="hidden sm:flex items-center gap-2 rounded-full border border-yellow-500/30 dark:border-yellow-500/50 bg-yellow-500/5 dark:bg-yellow-950/40 px-3 py-1.5 shadow-[0_0_10px_rgba(234,179,8,0.1)] dark:shadow-[0_0_10px_rgba(234,179,8,0.2)]">
            <span className="text-[11px] font-mono font-bold text-yellow-600 dark:text-yellow-500 uppercase tracking-wider">
              🟡 Solana Devnet
            </span>
          </div>

          {/* Dark Mode Forced (Theme Toggle removed to fix layout issues) */}

          {/* Wallet Integration Button */}
          <div className="[&>button]:!bg-gray-100 dark:[&>button]:!bg-gray-900 [&>button]:!text-sm [&>button]:!font-semibold [&>button]:!text-gray-900 dark:[&>button]:!text-white [&>button]:!border [&>button]:!border-emerald-500/30 dark:[&>button]:!border-emerald-500/50 [&>button:hover]:!border-emerald-400 [&>button:hover]:!bg-gray-200 dark:[&>button:hover]:!bg-gray-800 [&>button]:!transition-all [&>button]:!duration-200 [&>button]:!rounded-lg [&>button]:!h-10 [&>button]:!px-4">
            {mounted && <WalletMultiButton />}
          </div>
        </div>
      </div>
    </header>
  );
}

