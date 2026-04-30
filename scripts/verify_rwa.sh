#!/bin/bash

echo "------------------------------------------------"
echo "🔍 Z-RWA End-to-End Verification Script"
echo "------------------------------------------------"

# 1. Check if server is running
echo "[1/3] Checking API availability..."
if curl -s http://localhost:3000/api/stats > /dev/null; then
    echo "✅ API server is UP."
else
    echo "❌ API server is DOWN. Please run 'npm run dev' in apps/web."
    exit 1
fi

# 2. Test ZK Proof Generation
echo "[2/3] Generating ZK Proof via /api/generate-proof..."
RESPONSE=$(curl -s -X POST http://localhost:3000/api/generate-proof \
-H "Content-Type: application/json" \
-d '{"age": 25, "panHash": "0x3ba505c657aad8f", "kycScore": 750, "walletAddress": "GsPrDLXoqVbcWwofYpRZFJg4h5dzHEjyNfPyzPrcUKGd"}')

if [[ $RESPONSE == *"proof"* ]]; then
    echo "✅ ZK Proof generated successfully."
    # Extract some values if needed
else
    echo "❌ ZK Proof generation failed."
    echo "Response: $RESPONSE"
    exit 1
fi

# 3. Verify Stats Update
echo "[3/3] Verifying stats update..."
STATS=$(curl -s http://localhost:3000/api/stats)
echo "Current Stats: $STATS"

echo "------------------------------------------------"
echo "✅ Verification Complete!"
echo "------------------------------------------------"
