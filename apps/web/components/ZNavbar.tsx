"use client";

import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useState, useEffect } from "react";

export default function ZNavbar() {
  const { connected } = useWallet();
  const [isDark, setIsDark] = useState(true);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  useEffect(() => {
    if (isDark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }, [isDark]);

  return (
    <header className="sticky top-0 z-50 border-b border-gray-800 bg-gray-950/90 backdrop-blur-xl">
      <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
        {/* Left Side (Brand) */}
        <div className="flex items-center gap-4">
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
            <span className="text-xl font-bold tracking-tight text-white font-space">
              Z-RWA <span className="text-green-500">Compliance</span>
            </span>
            <span className="text-[9px] font-mono text-gray-400 uppercase tracking-[0.3em] mt-0.5">
              🟢 v1.0.0-beta • Local Device
            </span>
          </div>
        </div>

        {/* Right Side (Controls) */}
        <div className="flex items-center gap-4">
          {/* Network Badge */}
          <div className="hidden sm:flex items-center gap-2 rounded-full border border-yellow-500/50 bg-yellow-950/40 px-3 py-1.5 shadow-[0_0_10px_rgba(234,179,8,0.2)]">
            <span className="text-[11px] font-mono font-bold text-yellow-500 uppercase tracking-wider">
              🟡 Solana Devnet
            </span>
          </div>

          {/* Theme Toggle */}
          <button 
            onClick={() => setIsDark(!isDark)}
            className="flex h-10 w-10 items-center justify-center rounded-lg border border-gray-800 bg-gray-900 text-gray-400 hover:text-white hover:border-gray-600 transition-colors"
          >
            {isDark ? (
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>
            ) : (
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>
            )}
          </button>

          {/* Wallet Integration Button */}
          <div className="[&>button]:!bg-gray-900 [&>button]:!text-sm [&>button]:!font-semibold [&>button]:!text-white [&>button]:!border [&>button]:!border-purple-500/50 [&>button:hover]:!border-purple-400 [&>button:hover]:!bg-gray-800 [&>button]:!transition-all [&>button]:!duration-200 [&>button]:!rounded-lg [&>button]:!h-10 [&>button]:!px-4">
            {mounted && <WalletMultiButton />}
          </div>
        </div>
      </div>
    </header>
  );
}
