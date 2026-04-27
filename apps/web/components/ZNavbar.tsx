"use client";

import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useState, useEffect } from "react";
import ThemeToggle from "./ThemeToggle";
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
    { href: "/check", label: "Check Wallet", primary: false },
    { href: "/community", label: "Community", primary: false },
  ];

  return (
    <header className="sticky top-0 z-50 border-b border-gray-200 dark:border-gray-800 bg-[var(--background)]/90 backdrop-blur-xl transition-colors duration-200">
      <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
        {/* Left Side (Brand) */}
        <div className="flex items-center gap-6">
          <Link href="/" className="flex items-center gap-4">
            <div className="relative flex h-10 w-10 items-center justify-center">
              {/* Hexagon icon */}
              <svg viewBox="0 0 36 36" fill="none" className="absolute inset-0 h-full w-full">
                <polygon
                  points="18,2 33,10.5 33,25.5 18,34 3,25.5 3,10.5"
                  stroke="#00cc66"
                  strokeWidth="1.5"
                  fill="rgba(0,204,102,0.1)"
                />
              </svg>
              <span className="relative text-sm font-bold text-green-500 font-mono">Z</span>
            </div>
            <div className="flex flex-col">
              <span className="text-xl font-bold tracking-tight text-[var(--foreground)] font-space">
                Z-RWA <span className="text-green-500">Compliance</span>
              </span>
              <span className="text-[9px] font-mono text-gray-500 dark:text-gray-400 uppercase tracking-[0.3em] mt-0.5">
                🟢 v1.0.0-beta • Local Device
              </span>
            </div>
          </Link>

          {/* Nav Links */}
          <nav className="hidden md:flex items-center gap-1">
            {navLinks.map(({ href, label, primary }) => (
              <Link
                key={href}
                href={href}
                className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors
                  ${primary
                    ? "text-purple-400 bg-purple-500/10 border border-purple-500/20 hover:bg-purple-500/20 hover:text-purple-300"
                    : pathname === href
                    ? "text-purple-400 bg-purple-500/10 border border-purple-500/20"
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

          {/* New Cyberpunk Theme Toggle */}
          <ThemeToggle />

          {/* Wallet Integration Button */}
          <div className="[&>button]:!bg-gray-100 dark:[&>button]:!bg-gray-900 [&>button]:!text-sm [&>button]:!font-semibold [&>button]:!text-gray-900 dark:[&>button]:!text-white [&>button]:!border [&>button]:!border-purple-500/30 dark:[&>button]:!border-purple-500/50 [&>button:hover]:!border-purple-400 [&>button:hover]:!bg-gray-200 dark:[&>button:hover]:!bg-gray-800 [&>button]:!transition-all [&>button]:!duration-200 [&>button]:!rounded-lg [&>button]:!h-10 [&>button]:!px-4">
            {mounted && <WalletMultiButton />}
          </div>
        </div>
      </div>
    </header>
  );
}

