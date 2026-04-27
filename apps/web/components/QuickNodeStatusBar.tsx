"use client";
import { useEffect, useState } from "react";
import { getNetworkStats } from "../lib/quicknode";

export function QuickNodeStatusBar() {
  const [stats, setStats] = useState({ slot: 0, tps: 0, fee: 0 });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchStats() {
      try {
        const [network, feeRes] = await Promise.all([
          getNetworkStats(),
          fetch("/api/priority-fee").catch(() => null)
        ]);

        let fee = 1000;
        if (feeRes && feeRes.ok) {
          const feeData = await feeRes.json();
          fee = feeData.fee ?? 1000;
        }

        setStats({ 
          slot: network.slot, 
          tps: network.tps,
          fee: fee
        });
      } catch (err) {
        console.error("Failed to fetch QuickNode stats:", err);
      } finally {
        setLoading(false);
      }
    }
    
    fetchStats();
    const interval = setInterval(fetchStats, 10000); // refresh every 10s
    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{
      background: "var(--color-background-secondary)",
      borderBottom: "0.5px solid var(--color-border-tertiary)",
      padding: "4px 16px",
      display: "flex",
      gap: "24px",
      alignItems: "center",
      fontSize: "11px",
      fontFamily: "monospace",
    }} className="dark:bg-[#0c0c0c] bg-gray-50 dark:border-gray-900 border-gray-200">
      {/* QuickNode badge */}
      <span style={{ 
        background: "#0F6E56", 
        color: "#E1F5EE",
        padding: "2px 8px",
        borderRadius: "4px",
        fontWeight: 600,
        fontSize: "10px"
      }}>
        ⚡ QUICKNODE
      </span>
      
      <span className="text-gray-500">
        Slot: <span className="text-gray-900 dark:text-gray-100 font-semibold ml-1">
          {loading ? "..." : stats.slot.toLocaleString()}
        </span>
      </span>
      
      <span className="text-gray-500 hidden sm:inline">
        TPS: <span className="text-gray-900 dark:text-gray-100 font-semibold ml-1">
          {loading ? "..." : stats.tps.toLocaleString()}
        </span>
      </span>
      
      <span className="text-gray-500 hidden sm:inline">
        Priority fee: <span className="text-gray-900 dark:text-gray-100 font-semibold ml-1">
          {loading ? "..." : `${stats.fee.toLocaleString()} µ◎`}
        </span>
      </span>
      
      <span style={{ 
        marginLeft: "auto",
        fontSize: "10px"
      }} className="text-gray-400 font-semibold uppercase tracking-wider">
        Solana Devnet · live
      </span>
    </div>
  );
}
