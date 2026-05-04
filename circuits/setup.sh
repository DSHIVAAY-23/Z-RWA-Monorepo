#!/bin/bash
set -e

echo "Compiling circuit..."
mkdir -p build
circom compliance.circom --r1cs --wasm --sym -o build/

echo "Downloading Powers of Tau (ptau12)..."
curl -o build/pot12_final.ptau https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_12.ptau


echo "Running Groth16 setup..."
npx snarkjs groth16 setup build/compliance.r1cs build/pot12_final.ptau build/compliance_0.zkey

echo "Contribute randomness..."
echo "zrwa entropy $(date)" | npx snarkjs zkey contribute build/compliance_0.zkey build/compliance_final.zkey --name="Z-RWA" -v

echo "Export verification key..."
npx snarkjs zkey export verificationkey build/compliance_final.zkey build/verification_key.json

echo "Copying to public for client-side proving if needed..."
mkdir -p ../apps/web/public/circuits
cp build/compliance_js/compliance.wasm ../apps/web/public/circuits/
cp build/compliance_final.zkey ../apps/web/public/circuits/
cp build/verification_key.json ../apps/web/public/circuits/

echo "Setup complete."
