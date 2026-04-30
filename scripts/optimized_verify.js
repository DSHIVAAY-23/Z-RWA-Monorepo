const snarkjs = require('snarkjs');
const fs = require('fs');
const path = require('path');
const http = require('http');

// ==========================================
// OPTIMIZATION 1: Pre-load Verification Key
// ==========================================
// Reading the file system on every verification request adds massive I/O overhead.
// By loading the vKey into memory at startup, verification becomes purely CPU-bound.
const vkeyPath = path.join(__dirname, '../apps/web/public/circuits/verification_key.json');
let vKeyCache = null;

try {
    vKeyCache = JSON.parse(fs.readFileSync(vkeyPath, 'utf-8'));
    console.log("✅ [Optimization] Verification Key loaded into memory.");
} catch (e) {
    console.error("❌ Failed to load vKey:", e.message);
    process.exit(1);
}

/**
 * Optimized Verification Function
 */
async function fastVerify(publicSignals, proof) {
    // OPTIMIZATION 2: snarkjs.groth16.verify is already heavily optimized in Node.js
    // leveraging WASM/C++ under the hood when available.
    return await snarkjs.groth16.verify(vKeyCache, publicSignals, proof);
}

// ==========================================
// Benchmark Utility
// ==========================================
async function runBenchmark() {
    console.log("\n--- Starting Proof Verification Benchmark ---");
    
    const wasmPath = path.join(__dirname, '../apps/web/public/circuits/compliance.wasm');
    const zkeyPath = path.join(__dirname, '../apps/web/public/circuits/compliance_final.zkey');

    const input = { age: 25, panHash: 123456789, kycScore: 800, minAge: 18, minKycScore: 700 };

    console.log("Generating sample proof (this takes a moment)...");
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmPath, zkeyPath);
    
    console.log("\nRunning 100 verifications in memory...");
    const start = performance.now();
    
    let successCount = 0;
    for(let i = 0; i < 100; i++) {
        const isValid = await fastVerify(publicSignals, proof);
        if (isValid) successCount++;
    }
    
    const end = performance.now();
    const totalTimeMs = end - start;
    const avgTimeMs = totalTimeMs / 100;
    
    console.log(`\n--- Benchmark Results ---`);
    console.log(`Total Verifications : 100`);
    console.log(`Successful          : ${successCount}/100`);
    console.log(`Total Time          : ${totalTimeMs.toFixed(2)} ms`);
    console.log(`Average Time/Proof  : ${avgTimeMs.toFixed(2)} ms`);
    
    if (avgTimeMs < 50) {
        console.log(`🚀 Lightning Fast! Proof verification takes less than 50ms natively.`);
    }

    // ==========================================
    // OPTIMIZATION 3: Micro-Server
    // ==========================================
    // We can run a dedicated lightweight verification microservice.
    console.log("\nStarting Optimized Verification API on port 4000...");
    const server = http.createServer(async (req, res) => {
        if (req.method === 'POST' && req.url === '/verify') {
            let body = '';
            req.on('data', chunk => body += chunk.toString());
            req.on('end', async () => {
                try {
                    const { proof, publicSignals } = JSON.parse(body);
                    const startVerify = performance.now();
                    
                    const isValid = await fastVerify(publicSignals, proof);
                    
                    const verifyTime = performance.now() - startVerify;
                    
                    res.writeHead(200, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ 
                        valid: isValid, 
                        verificationTimeMs: verifyTime.toFixed(2) 
                    }));
                } catch (e) {
                    res.writeHead(400, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ error: e.message }));
                }
            });
        } else {
            res.writeHead(404);
            res.end();
        }
    });

    server.listen(4000, () => {
        console.log("✅ Verification Microservice running at http://localhost:4000/verify");
        console.log("Use POST with JSON { proof, publicSignals }");
        
        // Terminate after a few seconds so the command finishes in Antigravity
        setTimeout(() => {
            console.log("\nShutting down server for completion.");
            server.close();
            process.exit(0);
        }, 3000);
    });
}

runBenchmark().catch(console.error);
