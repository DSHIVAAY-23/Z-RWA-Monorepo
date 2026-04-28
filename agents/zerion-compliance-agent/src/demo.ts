// Quick demo for judges — runs in under 60 seconds
// Shows: 3 requests, 2 pass, 1 fail, clear output

import { config } from "dotenv";
config();

console.log("╔═══════════════════════════════════════════╗");
console.log("║   Z-RWA × Zerion — Compliance Agent Demo  ║");
console.log("╠═══════════════════════════════════════════╣");
console.log("║ Policy: ZK Compliance Required            ║");
console.log("║ Chain Lock: Solana only                   ║");
console.log("║ Max Spend: $10,000 per transaction        ║");
console.log("║ Expiry: Dec 31, 2026                      ║");
console.log("╚═══════════════════════════════════════════╝\n");

// Import and run the agent
// Using require to ensure env variables log correctly before running agent
require("./agent");
