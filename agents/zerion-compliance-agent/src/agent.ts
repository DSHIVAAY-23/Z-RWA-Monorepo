import { config } from "dotenv";
config();

import { evaluatePolicy, DEFAULT_RWA_POLICY } from "./policy";
import { TransferRequest, AgentAction } from "./types";

// Mock pending transfer queue
// In production: read from Solana program account (Z-RWA transfer queue)
const MOCK_TRANSFER_QUEUE: TransferRequest[] = [
  {
    id: "req-001",
    fromWallet: process.env.COMPLIANT_WALLET || "DHEi2Q72C5mdAvnak3wFhTk7jMHsZ76MGhNeTg4yenXw",
    toWallet: "FhuXW2JHUyTNFF8eXW1EYsfuWcx3RfzdXHuDPvN7A7Xc",
    tokenMint: "So11111111111111111111111111111111111111112",
    amount: 100,
    requestedAt: new Date(),
    status: "pending",
  },
  {
    id: "req-002", 
    fromWallet: "UNVERIFIED_WALLET_ADDRESS_HERE",
    toWallet: "FhuXW2JHUyTNFF8eXW1EYsfuWcx3RfzdXHuDPvN7A7Xc",
    tokenMint: "So11111111111111111111111111111111111111112",
    amount: 500,
    requestedAt: new Date(),
    status: "pending",
  },
  {
    id: "req-003",
    fromWallet: process.env.COMPLIANT_WALLET || "DHEi2Q72C5mdAvnak3wFhTk7jMHsZ76MGhNeTg4yenXw",
    toWallet: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
    tokenMint: "So11111111111111111111111111111111111111112",
    amount: 15000, // exceeds $10k limit
    requestedAt: new Date(),
    status: "pending",
  },
];

const actionLog: AgentAction[] = [];

async function processTransferQueue() {
  console.log("\n=== Z-RWA Compliance Agent ===");
  console.log(`Policy: ${DEFAULT_RWA_POLICY.name}`);
  console.log(`Chain: ${DEFAULT_RWA_POLICY.allowedChains.join(", ")}`);
  console.log(`Max spend/tx: $${DEFAULT_RWA_POLICY.maxSpendUsdPerTx}`);
  console.log(`ZK Compliance required: ${DEFAULT_RWA_POLICY.requireZKCompliance}`);
  console.log(`Policy expires: ${DEFAULT_RWA_POLICY.expiresAt.toISOString()}`);
  console.log(`\nProcessing ${MOCK_TRANSFER_QUEUE.length} pending requests...\n`);

  for (const request of MOCK_TRANSFER_QUEUE) {
    console.log(`\n--- Request ${request.id} ---`);
    console.log(`From: ${request.fromWallet.slice(0, 8)}...`);
    console.log(`Amount: $${request.amount}`);

    const decision = await evaluatePolicy(request, DEFAULT_RWA_POLICY);

    if (decision.approved) {
      console.log(`✓ APPROVED — ${decision.reason}`);
      
      // Execute via Zerion CLI
      await executeViaZerion(request, decision.proofHash);
      
      actionLog.push({
        type: "transfer",
        walletAddress: request.fromWallet,
        amount: request.amount,
        reason: decision.reason,
        complianceProofHash: decision.proofHash,
        timestamp: new Date(),
      });
    } else {
      console.log(`✗ REJECTED — ${decision.reason}`);
      console.log(`  Blocked by: ${decision.blockedBy}`);
      
      actionLog.push({
        type: "flag",
        walletAddress: request.fromWallet,
        amount: request.amount,
        reason: `Blocked by ${decision.blockedBy}: ${decision.reason}`,
        timestamp: new Date(),
      });
    }
  }

  printSummary();
}

async function executeViaZerion(
  request: TransferRequest,
  proofHash: string
) {
  // In a real integration: call Zerion CLI programmatically
  // For demo: simulate the Zerion API call
  
  console.log(`  Routing through Zerion API...`);
  console.log(`  Proof hash included: ${proofHash.slice(0, 16)}...`);
  
  // Real Zerion swap call would look like:
  // const zerionResponse = await fetch("https://api.zerion.io/v1/swap", {
  //   method: "POST",
  //   headers: { "Authorization": `Bearer ${process.env.ZERION_API_KEY}` },
  //   body: JSON.stringify({
  //     from_chain: "solana",
  //     from_token: request.tokenMint,
  //     amount: request.amount,
  //     // metadata: proofHash included in tx memo
  //   })
  // });
  
  console.log(`  ✓ Transaction submitted via Zerion`);
  console.log(`  Explorer: https://explorer.solana.com/?cluster=devnet`);
}

function printSummary() {
  const approved = actionLog.filter(a => a.type === "transfer").length;
  const rejected = actionLog.filter(a => a.type === "flag").length;
  
  console.log("\n=== Agent Run Summary ===");
  console.log(`Total requests: ${actionLog.length}`);
  console.log(`Approved (ZK compliant): ${approved}`);
  console.log(`Rejected (non-compliant): ${rejected}`);
  console.log(`Compliance rate: ${Math.round(approved / actionLog.length * 100)}%`);
  console.log("\nThe agent is the compliance process.");
  console.log("No human approval required. No central authority.");
  console.log("A transfer either has a valid ZK proof or it doesn't.\n");
  
  // Exit the process so `npm run demo` finishes gracefully
  process.exit(0);
}

// Run once immediately
processTransferQueue();
