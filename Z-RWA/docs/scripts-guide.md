## Scripts usage guide (SolanaSTO)

This guide shows how to run the TypeScript scripts to initialize and interact with the on-chain programs in this repo.

### Requirements
- Node 18+, yarn or npm
- Solana CLI and Anchor installed
- A Solana keypair at `~/.config/solana/id.json` (or set `SOLANA_KEYPAIR=/absolute/path/to/keypair.json`)

### Clusters and RPC selection
- Localnet (default): scripts use `http://127.0.0.1:8899` by default.
  - Override with `SOLANA_URL` if needed.
- Devnet: set an env var before running any script:
  - `export SOLANA_URL=https://api.devnet.solana.com`

### Start local validator (localnet)
Run in a separate terminal:
```
solana-test-validator --reset \
  --ledger /data/data/Contract-deployment/SolanaSTO/test-ledger \
  --rpc-port 8899 \
  --limit-ledger-size 1000000
```

Verify health:
```
curl -s http://127.0.0.1:8899 -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'
```

### Build and deploy programs
- Localnet:
```
anchor build
anchor deploy
```

- Devnet (ensure you have SOL airdropped):
```
export SOLANA_URL=https://api.devnet.solana.com
anchor build
anchor deploy
```

Program IDs used by scripts (localnet):
- base-token-program: `7iaDbVGbVJdhZcKXWQmMw783nBqfDyx6K8V4yF6Kv8iq`
- fund-contract: `H9txrMrTfGU6LXYWBzMHzmzuQWbNYcW1vHFLRcxCYiKn`
- token-program: `33pHXYQbe41JJSA7oXor6h7JFY74eqH25xtSjBysmTYo`
- interop-core: `5dyQmihDcQqCBerQg82J19QxMByjpxbwdtmCuPkT9ePD`
- interop-multisig: `2uuBHq3teujBvfE3AnRm4LZFYk7sHUiC36Z9MdkgcJ2N`
- treasury-bond: `53cKZWTPTwLjXD7E6NHCQzn5Gs9KL81u5xqicHEJwNgE`

Make sure `scripts/constant.ts` and the IDLs in `target/idl/*.json` match your deployed program IDs for the cluster you’re using.

### End-to-end entry script
Run all initialization in sequence (idempotent; safe to rerun):
```
npx ts-node scripts/index.ts
```
What it does (by default):
- Ensures wallet has SOL on devnet (best-effort)
- Initializes token, fund, base programs (skips if already initialized)
- If you uncomment core/multisig in `scripts/index.ts`, those will init as well

### Compute PDAs
Print PDAs for all programs on the current cluster:
```
npx ts-node scripts/compute_pdas_and_prepare.ts
```
Use the printed PDAs in your custom calls if you write standalone scripts.

### Running individual actions (base-token-program)
You can call exported helpers directly without editing files:
```
# Maintainers info
node -e "require('ts-node/register/transpile-only');(async()=>{await require('./scripts/base-token-program').fetchBaseMaintainers();})()"

# Create a test token (uses constants TEST/TEST_TOKEN)
node -e "require('ts-node/register/transpile-only');(async()=>{await require('./scripts/base-token-program').createToken();})()"

# Request mint to your ATA
node -e "require('ts-node/register/transpile-only');(async()=>{await require('./scripts/base-token-program').requestOrders();})()"

# Read balances (supply and your ATA balance)
node -e "require('ts-node/register/transpile-only');(async()=>{await require('./scripts/base-token-program').fetchBalances();})()"
```

Alternatively, add these at the end of `scripts/index.ts` and run the entry script:
- `await base.getBaseKeys();`
- `await base.fetchBaseMaintainers();`
- `await base.createToken();`
- `await base.requestOrders();`
- `await base.fetchBalances();`

### Fund wallet (devnet only)
If you see insufficient funds on devnet, airdrop:
```
solana airdrop 2 -u https://api.devnet.solana.com
```

### Troubleshooting
- Attempt to load a program that does not exist
  - You’re pointing at a cluster where the program ID isn’t deployed. Deploy to that cluster, or set `SOLANA_URL` to the cluster where it is deployed.

- DeclaredProgramIdMismatch
  - The on-chain program’s `declare_id!` doesn’t match the deployed address. Update `declare_id!` in Rust, rebuild, redeploy.

- Allocate: account ... already in use
  - You re-ran an init that creates a PDA that already exists. Scripts here make inits idempotent; if you wrote your own call, first try fetching the PDA and skip init when it exists.

- failed to get recent blockhash / fetch failed
  - Local validator is down. Start it with the command above and retry after health is ok.

### Where things are
- Script entry: `scripts/index.ts`
- Per-program helpers: `scripts/*-program.ts`, `scripts/fund-contract.ts`, `scripts/interop-*.ts`
- Program IDs/constants: `scripts/constant.ts`
- Provider/IDL wiring: `scripts/solanaService.ts`

That’s it. Use `index.ts` for full flows, or call individual helpers to test specific functions.

### End-to-end localnet flow (copy-paste)
Use this block if you just want the exact commands to bring everything up locally, mint supply, and verify balances.

1) Start the validator in a separate terminal:
```
solana-test-validator --reset \
  --ledger /data/data/Contract-deployment/SolanaSTO/test-ledger \
  --rpc-port 8899 \
  --limit-ledger-size 1000000
```

2) Build and deploy, then run the main script:
```
cd /data/data/Contract-deployment/SolanaSTO
anchor build
anchor deploy
npx ts-node scripts/index.ts
```

3) Mint supply and check balances for base-token-program (defaults to 100; edit amount inside `requestOrders` if you want more):
```
node -e "require('ts-node/register/transpile-only');(async()=>{await require('./scripts/base-token-program').fetchBaseMaintainers();await require('./scripts/base-token-program').getBaseKeys();await require('./scripts/base-token-program').createToken();await require('./scripts/base-token-program').requestOrders();await require('./scripts/base-token-program').fetchBalances();})()"
```

Notes:
- If you see an error about fetching recent blockhash, ensure your local validator is running and responsive.
- If you see “Attempt to load a program that does not exist,” deploy to the cluster you’re targeting or point `SOLANA_URL` to the correct cluster.


