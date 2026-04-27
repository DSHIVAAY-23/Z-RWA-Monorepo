// lib/quicknode.ts
// QuickNode is the primary RPC provider for Z-RWA.
//
// Why QuickNode?
// Groth16 proof payloads for 7.4M constraint circuits produce ~260 bytes of
// proof data per transaction. Standard public devnet RPC endpoints reject these
// large payloads under load and apply aggressive rate limits that cause proof
// submissions to fail. QuickNode provides a reliable, high-throughput dedicated
// endpoint for ZK proof submission at production scale.

import { Connection } from "@solana/web3.js";

const QUICKNODE_ENDPOINT =
  process.env.NEXT_PUBLIC_QUICKNODE_RPC_URL ||
  "https://frequent-alpha-pool.solana-devnet.quiknode.pro/5f06a41cf6e077af5ca7ac464fbf1caed5c84d42/";

// Primary connection — all Z-RWA transactions route through QuickNode
export const quicknodeConnection = new Connection(QUICKNODE_ENDPOINT, {
  commitment: "confirmed",
  confirmTransactionInitialTimeout: 60000,
});

/**
 * Fetches the optimal priority fee using QuickNode's qn_estimatePriorityFees API.
 * ZK proof transactions are large; setting priority fees helps them land faster
 * during network congestion.
 * Returns microlamports (used in ComputeBudgetProgram.setComputeUnitPrice).
 */
export async function getOptimalPriorityFee(): Promise<number> {
  try {
    const response = await fetch(QUICKNODE_ENDPOINT, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "qn_estimatePriorityFees",
        params: { last: 100 },
      }),
    });
    const data = await response.json();
    // Use the "high" tier for ZK proof transactions — large payloads need priority
    const high =
      data?.result?.per_compute_unit?.high ??
      data?.result?.high ??
      null;
    return typeof high === "number" && high > 0 ? Math.round(high) : 1000;
  } catch {
    // Fallback: 1000 microlamports — safe minimum priority fee
    return 1000;
  }
}

export interface NetworkStats {
  slot: number;
  tps: number;
  endpoint: string;
}

/**
 * Returns real-time network stats fetched from QuickNode:
 * - Current slot
 * - Estimated TPS (from recent performance samples)
 */
export async function getNetworkStats(): Promise<NetworkStats> {
  try {
    const [slot, perfSamples] = await Promise.all([
      quicknodeConnection.getSlot(),
      quicknodeConnection.getRecentPerformanceSamples(1),
    ]);

    const tps =
      perfSamples[0]
        ? Math.round(
            perfSamples[0].numTransactions / perfSamples[0].samplePeriodSecs
          )
        : 0;

    return { slot, tps, endpoint: "QuickNode" };
  } catch {
    return { slot: 0, tps: 0, endpoint: "QuickNode" };
  }
}
