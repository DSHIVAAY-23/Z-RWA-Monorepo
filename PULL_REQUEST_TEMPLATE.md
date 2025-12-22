# Pull Request: ZK-RAG x Z-RWA Integration (CoinDCX Grant Milestone)

## Description
This PR integrates the Zero-Knowledge Retrieval Augmented Generation (ZK-RAG) prover with the `z-rwa` Solana program. This enables privacy-preserving RWA compliance checks where users prove document relevance off-chain and only submit a ZK proof on-chain.

## Changes
- **New Program**: `z-rwa` (Anchor) acting as the RWA Gatekeeper.
- **Prover Logic**: Updated ZK-RAG to export SP1 Groth16 proofs + Document Hash binding.
- **Verification**: Implemented on-chain `verify_and_mint` using `sp1-solana`.

## Performance Benchmarks
- **Proof System**: SP1 Groth16 (v3.0.0)
- **Constraint Count**: 7,493,634
- **Proof Generation Time**: ~23.1s
- **Verification Cost**: ~200k-300k CU (Precompile)

## Security
- **VKey Binding**: The contract enforces proofs match the `ZK_RAG` circuit VKey: `0x00cef2f0dedae3382b36f085503bb1a86d98102bca1f64362bdaa1634276df9f`.
- **Data Sovereignty**: Partial/Redacted data never leaves the user device.

## Testing
- [x] Mock Mode (`npm run test:mock`)
- [x] Release Proof Generation (`npm run prove:release`)
- [x] Devnet Integration (`anchor test`)
