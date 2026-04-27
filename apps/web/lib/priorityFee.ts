const QUICKNODE_RPC = process.env.QUICKNODE_RPC_URL || "https://api.devnet.solana.com";

export async function getOptimalPriorityFee(): Promise<number> {
  try {
    const response = await fetch(QUICKNODE_RPC, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "qn_estimatePriorityFees",
        params: {
          last: 100, // last 100 blocks
        },
      }),
    });
    
    const data = await response.json();
    if (data.result?.high?.priorityFeeEstimate) {
      return data.result.high.priorityFeeEstimate;
    }
    return 10000; // fallback 10k microlamports
  } catch (error) {
    console.error("Failed to estimate QuickNode priority fees, using fallback:", error);
    return 10000;
  }
}
