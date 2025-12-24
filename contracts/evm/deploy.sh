#!/bin/bash
# Fuse Spark Testnet Deployment Script
# This script deploys the Private Payroll Verifier to Fuse Spark Testnet

set -e  # Exit on error

echo "================================================"
echo "Fuse Payroll Verifier - Deployment Script"
echo "================================================"
echo ""

# Check if PRIVATE_KEY is set
if [ -z "$PRIVATE_KEY" ]; then
    echo "❌ Error: PRIVATE_KEY environment variable not set"
    echo ""
    echo "Please set your private key:"
    echo "  export PRIVATE_KEY=your_private_key_here"
    echo ""
    exit 1
fi

echo "✅ Private key detected"
echo ""

# Navigate to contracts directory
cd "$(dirname "$0")"

echo "📦 Building contracts..."
forge build

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please fix compilation errors."
    exit 1
fi

echo "✅ Build successful"
echo ""

echo "🚀 Deploying to Fuse Spark Testnet..."
echo "   Network: Fuse Spark"
echo "   Chain ID: 123"
echo "   RPC: https://rpc.fusespark.io"
echo ""

# Deploy contracts
forge script script/DeployFuse.s.sol \
    --rpc-url https://rpc.fusespark.io \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --legacy \
    -vvv

if [ $? -eq 0 ]; then
    echo ""
    echo "================================================"
    echo "✅ DEPLOYMENT SUCCESSFUL!"
    echo "================================================"
    echo ""
    echo "Next steps:"
    echo "1. Check the deployment output above for contract addresses"
    echo "2. Save the addresses for your grant documentation"
    echo "3. Verify contracts on Fuse Explorer (optional)"
    echo "4. Test the verifySalary function with sample data"
    echo ""
    echo "For detailed instructions, see README_FUSE.md"
    echo ""
else
    echo ""
    echo "❌ Deployment failed. Please check the error messages above."
    echo ""
    echo "Common issues:"
    echo "  - Insufficient testnet FUSE tokens (get from faucet)"
    echo "  - Network connectivity issues"
    echo "  - Invalid private key"
    echo ""
    exit 1
fi
