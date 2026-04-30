# Z-RWA Protocol: ZK-Powered Compliance for Institutional DeFi

Z-RWA is a privacy-preserving compliance layer designed for India's DPDP (Digital Personal Data Protection) Act. It enables institutional investors to prove their identity and compliance status on-chain without revealing sensitive personal information like Aadhaar or PAN.

## 🚀 Achievements

### 1. Zero-Knowledge Compliance Engine
- **Real Circom Circuit**: Implemented a Groth16-based circuit (`circuits/compliance.circom`) that verifies age and KYC scores without revealing raw values.
- **SnarkJS Integration**: Full backend proof generation pipeline in Next.js, producing 260-byte lightweight proofs optimized for blockchain verification.
- **Trusted Setup**: Successfully performed the Powers of Tau ceremony and generated verification keys.

### 2. Solana On-Chain Verification
- **Anchor Verifier Program**: Built and deployed a custom Rust program (`z_rwa_verifier`) that integrates the `groth16-solana` crate.
- **Groth16 Verifier**: Natively verifies SnarkJS-generated proofs on Solana Devnet within sub-second execution time.
- **Proof-to-Instruction Mapping**: Automated conversion of SnarkJS proof structures to Solana-compatible byte arrays.
### 3. Token2022 Compliant Assets
- **Asset Issuance**: Integrated the **Token2022** standard to mint "Compliance Tokens" upon successful ZK verification.
- **Backend Authority**: Secured minting authority via a backend keypair, ensuring tokens are only issued for validated proofs.
- **Real-Time Stats**: Implemented a proof log system that tracks and displays live network metrics on the dashboard.

---

## 🛠️ Tech Stack
- **ZK**: Circom 2.0, SnarkJS (Groth16)
- **Blockchain**: Solana (Devnet), Anchor Framework
- **Standard**: Token2022 (SPL)
- **Frontend**: Next.js 14, Tailwind CSS, Solana Wallet Adapter
- **Backend**: Node.js, Web3.js

---

## 🏃 How to Run & Verify

### 1. Prerequisites
- Node.js v18+
- Solana CLI & Anchor CLI
- A Solana wallet (e.g., Phantom) with Devnet SOL

### 2. Frontend & Backend Setup
```bash
# Clone the repository
git clone https://github.com/DSHIVAAY-23/Z-RWA-Monorepo
cd Z-RWA-Monorepo

# Install dependencies
npm install

# Build for production (REQUIRED for ZK performance)
cd apps/web
npm run build

# Start the optimized server
npm run start
```

### 3. Frontend Walkthrough
1. **Connect Wallet**: Use the "Connect Wallet" button to bind your Solana Devnet address.
2. **Upload Document**: Select "Aadhaar Card" or "PAN Card" and upload a sample image. The client-side OCR will verify the document pattern and generate a secure hash.
3. **Generate ZK Proof**: Click **"Generate Proof via SP1"**. In production mode, this computes the 7.4M constraints in ~1-2 seconds.
4. **Submit On-Chain**: Click **"Submit Proof & Mint Token"**. This verifies the 260-byte Groth16 proof on Solana Devnet.
5. **Receive Asset**: Once verified, the backend mints 1 RWA Compliance Token (Token2022) directly to your wallet.


---

## 🔗 Deployment Details (Devnet)
- **Verifier Program ID**: `GL8vm2SxWV7yHQbwoZegM7SkbJbEbEDn6A9m9W2XjeQe`
- **RWA Compliance Mint (Token2022)**: `8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa`
- **Backend Authority**: `GsPrDLXoqVbcWwofYpRZFJg4h5dzHEjyNfPyzPrcUKGd`

---

## 📂 Project Structure
- `circuits/`: Circom source code and setup scripts.
- `apps/web/`: Next.js application, API routes, and frontend.
- `Z-RWA/programs/z_rwa_verifier/`: Anchor program for on-chain verification.
- `scripts/`: Utility scripts for minting and setup.

---
*Built with ❤️ for the Colosseum Frontier 2026 Hackathon.*
