import { Connection, PublicKey } from "@solana/web3.js";
import * as dotenv from "dotenv";

// Check multiple potential .env locations
dotenv.config();
dotenv.config({ path: "./apps/web/.env" });

const RPC_FAST_URL = process.env.RPC_ENDPOINT;
const SOLANA_PUBLIC_URL = "https://api.devnet.solana.com";

const calls = 10;
const PROGRAM_ID = new PublicKey(
    process.env.NEXT_PUBLIC_Z_RWA_PROGRAM_ID && process.env.NEXT_PUBLIC_Z_RWA_PROGRAM_ID !== "YOUR_PROGRAM_ID_HERE" 
    ? process.env.NEXT_PUBLIC_Z_RWA_PROGRAM_ID 
    : "3SN3zAmuW5HWgJy5mcWjvy8vwDZRLosEajqydbuxiEZC"
);

async function measureLatency(connection: Connection, name: string): Promise<{avg: number, min: number, max: number, accountLatency: number | string}> {
    const latencies: number[] = [];
    console.log(`\nTesting ${name}...`);
    
    for (let i = 0; i < calls; i++) {
        try {
            const start = Date.now();
            await connection.getLatestBlockhash("confirmed");
            const latency = Date.now() - start;
            latencies.push(latency);
            process.stdout.write(".");
        } catch (e: any) {
            console.log(`\nFAILED on call ${i+1}: ${e.message}`);
        }
    }
    
    console.log();
    
    let avg = 0, min = 0, max = 0;
    if (latencies.length > 0) {
        avg = Math.round(latencies.reduce((a, b) => a + b, 0) / latencies.length);
        min = Math.min(...latencies);
        max = Math.max(...latencies);
    }
    
    // Test getAccountInfo
    let accountLatency: number | string = "FAILED";
    try {
        const start = Date.now();
        await connection.getAccountInfo(PROGRAM_ID);
        accountLatency = Date.now() - start;
    } catch (e: any) {
        console.log(`\ngetAccountInfo FAILED: ${e.message}`);
    }

    return { avg, min, max, accountLatency };
}

async function main() {
    if (!RPC_FAST_URL) {
        console.error("FAILED: RPC_ENDPOINT environment variable is not set. Please check your .env file.");
        return;
    }

    console.log("Starting RPC Benchmark...");
    
    const rpcFastConn = new Connection(RPC_FAST_URL, "confirmed");
    const pubConn = new Connection(SOLANA_PUBLIC_URL, "confirmed");
    
    const fastStats = await measureLatency(rpcFastConn, "RPC Fast");
    const pubStats = await measureLatency(pubConn, "Solana Public");
    
    console.log("\n=================== BENCHMARK RESULTS ===================");
    console.log(`Endpoint           | Avg Latency | Min    | Max`);
    console.log(`-------------------|-------------|--------|--------`);
    console.log(`RPC Fast           | ${fastStats.avg}ms       | ${fastStats.min}ms    | ${fastStats.max}ms`);
    console.log(`Solana Public      | ${pubStats.avg}ms       | ${pubStats.min}ms    | ${pubStats.max}ms`);
    console.log(`---------------------------------------------------------`);
    
    if (pubStats.avg > 0 && fastStats.avg > 0 && pubStats.avg > fastStats.avg) {
        const speedup = Math.round(((pubStats.avg - fastStats.avg) / pubStats.avg) * 100);
        console.log(`Speed advantage:   RPC Fast is ${speedup}% faster`);
    } else if (pubStats.avg > 0 && fastStats.avg > 0) {
        console.log(`Speed advantage:   Solana Public was faster in this run.`);
    } else {
        console.log(`Speed advantage:   Cannot calculate due to failures.`);
    }

    console.log("\n=================== getAccountInfo() test ===================");
    console.log(`RPC Fast       : ${fastStats.accountLatency}ms`);
    console.log(`Solana Public  : ${pubStats.accountLatency}ms`);
}

main().catch(console.error);
