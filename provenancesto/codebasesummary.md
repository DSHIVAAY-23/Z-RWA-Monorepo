# Codebase Summary: Provenance Smart Contracts

## Executive Overview

This codebase is a **comprehensive smart contract deployment system** for the [Provenance Blockchain](https://provenance.io/), a purpose-built, eco-friendly proof-of-stake blockchain for modernizing financial services. The project integrates the Provenance blockchain core with a suite of custom CosmWasm smart contracts designed for various financial and operational use cases.

The repository combines:
- **Provenance Blockchain Core**: A Go-based blockchain implementation using Cosmos SDK and CometBFT consensus
- **Smart Contracts**: 11+ production-ready Rust-based CosmWasm contracts
- **Build Infrastructure**: Docker and Makefile-based deployment automation

---

## Project Structure

```
Contract-deployment/provenancesto/
├── build/                      # Build artifacts and compiled blockchain
│   ├── Makefile               # Build orchestration
│   ├── blockchain_process.pid # Process management
│   ├── provnode/              # Provenance blockchain source
│   │   └── provenance/        # Core blockchain implementation (Go)
│   └── run/                   # Runtime artifacts
│       └── provenanced/       # Blockchain runtime directory
├── contracts/                 # Smart contract implementations
│   ├── axelar-router/         # EVM/Cosmos cross-chain bridge
│   ├── custom-marker/         # Custom token/marker implementation
│   ├── cw20-marker/           # CW20 token standard with restrictions
│   ├── cw20-token-contract/   # Standard CW20 token implementation
│   ├── factory/               # Factory contract for creating markers
│   ├── fund/                  # Capital call/fund management
│   ├── interop-core/          # Core interoperability contract
│   ├── interop-multisig/      # Multi-signature interop contract
│   ├── jpmc/                  # JPMC-specific contracts
│   │   ├── axelar-router/
│   │   ├── cw20-marker/
│   │   ├── gmp-sample/        # General Message Passing sample
│   │   ├── marker-factory/
│   │   └── scripts/
│   ├── price-feed/            # BandChain oracle price feed integration
│   └── treasury-bond/         # Treasury bond management
├── README.md                  # Project documentation
└── .gitignore                 # Git configuration
```

---

## Technology Stack

### Blockchain Core
- **Language**: Go 1.23+
- **Framework**: Cosmos SDK (v0.47+)
- **Consensus**: CometBFT (formerly Tendermint)
- **Purpose**: Financial services-focused blockchain with custom modules
- **Native Modules**: Name registry, Marker (token), Metadata, Attributes, IBC support

### Smart Contracts
- **Language**: Rust
- **Framework**: CosmWasm
- **Build**: Cargo package manager
- **Optimization**: Docker-based WASM optimization (cosmwasm/rust-optimizer)
- **Standards**: CW20 (token standard), CW721 (NFT standard)
- **Testing**: Unit tests with `cargo test`

### Build & Deployment
- **Orchestration**: GNU Make
- **Containerization**: Docker
- **CLI**: Provenance Chain CLI (`provenanced`)
- **Configuration**: TOML (Cargo.toml), JSON (genesis configs)

---

## Core Components

### 1. Provenance Blockchain Node (`build/provnode/provenance/`)

The foundation of the system - a full Cosmos-based blockchain implementation.

**Key Modules**:
- **Name Module**: Hierarchical domain name service (similar to DNS for blockchain)
- **Marker Module**: Custom token/asset creation and management
- **Metadata Module**: Scope-based record management and provenance tracking
- **Attribute Module**: Flexible attribute system for accounts and data
- **IBC Module**: Inter-blockchain communication for cross-chain operations
- **WASM Module**: Smart contract execution engine (CosmWasm)

**Features**:
- Financial-grade security and compliance
- Proof-of-stake consensus
- Gas-based transaction pricing
- Message-fee mechanism
- Forced-transfer capability for institutional use
- Custom genesis configuration

**Build Artifacts**:
```
build/provnode/provenance/
├── app/              # Application logic
├── cmd/              # Command-line tools
├── x/                # Custom modules
├── proto/            # Protocol buffer definitions
├── protoBindings/    # Generated proto bindings
├── testutil/         # Test utilities
└── networks/         # Network configurations (dev, local, ibc)
```

---

### 2. Smart Contracts

#### **A. Core Financial Contracts**

##### `custom-marker/` - Custom Marker Implementation
**Purpose**: Tokenize and manage fungible assets on Provenance Blockchain using the Marker module.

**Key Features**:
- Initialize with admin account
- Create and manage custom markers (tokens)
- Contract-level upgradability via migration
- Integration with Provenance's marker infrastructure

**Technology**: Rust + CosmWasm, Provenance-specific bindings
**Status**: Production (Devnet ID: 65, Testnet ID: 608)

---

##### `cw20-marker/` - CW20 Token with Restrictions
**Purpose**: Implement CW20 token standard with institutional-grade restrictions.

**Key Features**:
1. **Supply Management**
   - Maximum supply cap enforcement
   - Mint/burn operations
   
2. **Account-Level Controls**
   - Maximum holder balance limits
   - Frozen balance per account (partial lock)
   - Account-level freeze functionality
   
3. **Compliance**
   - Country code-based authorization (e.g., "91" for India)
   - Geo-restricted trading
   
4. **Shareholder Management**
   - Automatic shareholder registration on token receipt
   - Automatic removal when balance reaches 0

**Use Case**: Compliant tokenization for international markets with regional restrictions
**Status**: Production (Devnet ID: 16, Testnet ID: 651)

---

##### `cw20-token-contract/` - Standard CW20 Implementation
**Purpose**: Basic CW20 token standard implementation with enhanced security.

**Key Features**:
- Standard CW20 interface compliance
- Mint/burn/transfer operations
- Allowance management
- Marketing info support

**Extends**: cw20_base library for proven implementation
**Status**: Production (Contract ID: 532)

---

##### `treasury-bond/` - Treasury Bond Management
**Purpose**: Capital call and fund management for treasury bonds.

**Key Features**:
- Bond creation and lifecycle management
- Capital call functionality
- Fund distribution mechanisms
- Admin-controlled contract upgradability

**Use Case**: Institutional treasury management and bond issuance
**Similar to**: Fund contract with treasury-specific logic
**Status**: Production

---

##### `fund/` - Capital Call Fund Contract
**Purpose**: Manage capital calls and fund operations on Provenance Blockchain.

**Key Features**:
- Capital call creation and tracking
- Fund lifecycle management
- Multi-participant contribution handling
- Investor management

**Integration Points**:
- Marker contract for token operations
- Oracle contract for pricing/data

**Deployment Addresses**:
- Marker Contract: `tp1jphsuxnqvqdy9sl8qh886760szuupxv327zjrgqkner3jvwcel2srnmj8l`
- Oracle Contract: `tp15d2kxfntk3u8wtr42nsrgrtqf6jxf8lsn9qpj69nzkxh8ykhwfsq863kuz`

---

#### **B. Factory & Creation Contracts**

##### `factory/` - Contract Factory
**Purpose**: Factory pattern for creating and managing token/marker instances.

**Key Features**:
- Create new marker instances
- Manage multiple contracts
- Batch operations
- Sub-contract instantiation

**Uses**: Provenance-specific helper functions:
- `activate_marker`, `burn_marker_supply`, `cancel_marker`
- `create_forced_transfer_marker`, `destroy_marker`, `finalize_marker`
- `grant_marker_access`, `mint_marker_supply`, `transfer_marker_coins`
- `withdraw_coins`

**Status**: Production (Contract ID: 522)

---

#### **C. Cross-Chain & Interoperability**

##### `axelar-router/` - EVM ↔ Cosmos Bridge
**Purpose**: Send and receive message payloads between EVM (Ethereum-compatible) and Cosmos ecosystems.

**Key Features**:
- EVM contract integration via Axelar
- Message encoding/decoding (ethabi)
- IBC packet handling
- Cross-chain state synchronization

**Technology Stack**:
- `ethabi` for EVM encoding/decoding
- `osmosis_std_derive` for Osmosis integration
- IBC capabilities

**Deployed On**: Osmosis-5 testnet
**Status**: Production (Testnet address: `osmo12uds53qp285w68vzq9wx5kjjfj0evvsy7k0e2srx9mchj29h7urq3rtuuc`)

---

##### `price-feed/` - BandChain Oracle Integration
**Purpose**: Integrate with BandChain oracle for off-chain data (price feeds, reference data).

**Key Features**:
- Oracle script request/response
- IBC communication with BandChain
- Fee limit and gas management
- Data caching and validation

**Instantiation Parameters**:
```rust
InstantiateMsg {
    client_id: String,           // Unique oracle request ID
    oracle_script_id: Uint64,    // BandChain oracle script ID
    ask_count: Uint64,           // Number of validators to respond
    min_count: Uint64,           // Minimum validators required
    fee_limit: Vec<Coin>,        // uband payment limits
    prepare_gas: Uint64,         // Raw request preparation gas
    execute_gas: Uint64,         // Execution gas reserve
}
```

**Use Case**: Real-time price feeds, reference data, external data integration
**Status**: Development (not production-ready)

---

##### `interop-core/` - Core Interoperability
**Purpose**: Central interoperability hub for cross-chain communication and data synchronization.

**Key Features**:
- Ethereum address conversion (checksummed addresses)
- Hexadecimal data encoding/decoding
- Message routing and validation
- Error handling for cross-chain operations

**Dependencies**:
- `ethabi` for EVM type conversions
- `rustc_hex` for hex operations
- Custom error types for interop failures

**Deployment Addresses**:
- Dev: `tp1hfcpqqxl0e9g6terx5qw0nvqrfty9thequ6c8czc9k7vytyd98ys9pj40a`
- UAT: `tp1nme8093h2xt93ywyjethuw37twnzlfsdndhx9kfayrg79q9py2csrcjapa`
- Prod: `tp1wumgvldlcfh5g8fl823qdt5pqu7qf06ysvf4qtetqp003zjvzpzsgku3du`

---

##### `interop-multisig/` - Multi-Signature Interop
**Purpose**: Multi-signature authorization for interoperability operations.

**Key Features**:
- Multi-signature verification
- Threshold-based approval
- Message validation and routing
- Participant management

**Use Case**: Institutional-grade cross-chain transactions requiring multiple approvals
**Status**: Production (Devnet ID: 654)

---

#### **D. JPMC-Specific Contracts** (`contracts/jpmc/`)

JPMC (JP Morgan Chase) integration suite with specialized contracts:

##### `jpmc/gmp-sample/` - General Message Passing Sample
**Purpose**: Demonstration of Axelar's General Message Passing (GMP) protocol.

**Features**:
- Cross-chain message passing
- Payload serialization/deserialization
- Example implementations

**Deployment**:
- Devnet: `tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8` (ID: 7)
- Testnet: `tp1gdrv7296qj3f5yuxvtrqc43dp6nc4dwdw4f9hhpwe9n759jd0lushvxcu8` (ID: 546)

---

##### `jpmc/marker-factory/` - JPMC Marker Factory
**Purpose**: Factory for creating JPMC-specific markers and tokens.

**Status**: Production
**Deployment**:
- Devnet: `tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla` (ID: 88)
- Testnet: `tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070` (ID: 604)

---

##### `jpmc/cw20-marker/` and `jpmc/axelar-router/`
**Purpose**: JPMC-customized versions of standard CW20 and Axelar contracts with institutional requirements.

---

### 3. Build Infrastructure

#### Root Makefile (`build/Makefile`)

**Key Targets**:

```makefile
make start          # Clone Provenance, compile, initialize blockchain
make store          # Store WASM contract on-chain
make init           # Instantiate contract with parameters
make build-store    # Build and store contract
```

**Key Configuration**:
```makefile
BLOCKCHAIN_PID_FILE := blockchain_process.pid
REPO_URL = https://github.com/provenance-io/provenance
CONTRACT_REPO_URL = https://github.com/Oasis-Pro-Inc/provenancesto
DENOM ?= nhash                    # Native token
CHAIN_ID ?= testing               # Chain identifier
MIN_FLOOR_PRICE ?= 1905           # Minimum gas price
```

**Initialization Sequence**:
1. Clone Provenance blockchain
2. Compile Go code (`make install`)
3. Initialize blockchain with custom denom (nhash)
4. Create test accounts (validator, bob)
5. Configure root names (pio, pb, io, provenance)
6. Fund accounts with genesis tokens
7. Add markers with permissions
8. Set up message fees
9. Configure custom floor prices
10. Enable forced transfers for institutional accounts
11. Generate genesis transactions
12. Start blockchain

---

## Key Technologies & Libraries

### Rust/CosmWasm Dependencies

**Common Across Contracts**:
- `cosmwasm_schema`: JSON schema generation for contract interfaces
- `cosmwasm_std`: CosmWasm standard library (execution environment)
- `cw_storage_plus`: Enhanced storage abstractions
- `cw2`: Contract versioning
- `serde`/`serde_json`: Serialization
- `thiserror`: Error handling macros

**Provenance-Specific**:
- `provwasm_std`: Provenance blockchain bindings
  - Marker operations (mint, burn, transfer, access control)
  - Metadata management
  - Name service integration
  - Querying capabilities

**EVM/Cross-Chain**:
- `ethabi`: Ethereum ABI encoding/decoding
- `osmosis_std_derive`: Osmosis chain integration
- `rustc_hex`: Hexadecimal encoding/decoding

**Utility**:
- `bincode`: Binary serialization
- `cw20`: Standard token implementation
- `cw20_base`: Base token contract implementation
- `cosmwasm_schema`: QueryResponses trait
- `cw_serde`: Serde derive macros for CosmWasm

---

## Development Workflow

### 1. Contract Development
```bash
cd contracts/<contract-name>
cargo build --release           # Build contract
cargo test                      # Run unit tests
cargo schema                    # Generate JSON schemas
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.13  # Optimize WASM
```

### 2. Blockchain Setup
```bash
cd build
make start                      # Initialize and start blockchain
provenanced -t --home build/run/provenanced tx wasm store ...  # Store contract
provenanced -t --home build/run/provenanced tx wasm instantiate ...  # Deploy
```

### 3. Contract Interaction
Contracts are interacted with via:
- `ExecuteMsg`: State-changing operations
- `QueryMsg`: Read-only queries
- `InstantiateMsg`: Contract initialization
- JSON parameter files in `parameters/` directory
- CLI commands via `provenanced` binary

---

## Contract Deployment Details

### Network Status

**Active Networks**:
1. **Testnet**: Primary testing network
   - Node: `https://rpc.test.provenance.io:443`
   - Chain ID: `pio-testnet-1`
   - Gas prices: `4500nhash`

2. **Devnet**: Local/development network
   - Used in Makefile with `testing` chain ID
   - Custom denom support
   - Local RPC endpoint

3. **Mainnet**: Production network (separate repository)

### Contract Deployment Matrix

| Contract | Type | Devnet ID | Devnet Address | Testnet ID | Testnet Address | Status |
|----------|------|-----------|----------------|-----------|-----------------|--------|
| axelar-router | Bridge | - | - | 608 | tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk | Production |
| custom-marker | Token | 65 | tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx | - | - | Production |
| cw20-marker | Token | 16 | tp1wkwy0xh89ksdgj9hr347dyd2dw7zesmtrue6kfzyml4vdtz6e5wsvaczas | 651 | tp10m7er24gc7u0fl26qpm4d487d90vug2gw2s4kq9r5zw00nd4hymqgmrpa3 | Production |
| cw20-token-contract | Token | - | - | 532 | tp19kwsg0vpaa20pf5xkzpyfkvthgm6vk6ztlrmcdhjnxrn620agzsqwqnqaz | Production |
| factory | Factory | - | - | 522 | tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3 | Production |
| interop-core | Bridge | - | - | 658 | (Multi-env) | Production |
| interop-multisig | Multisig | 654 | tp1chaxg0l0vy4j48x3hr77663u6e5kx3akc74nadra0mqjfjks9r9sqnzrzx | - | - | Production |
| jpmc/gmp-sample | Bridge | 7 | tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8 | 546 | tp1gdrv7296qj3f5yuxvtrqc43dp6nc4dwdw4f9hhpwe9n759jd0lushvxcu8 | Production |
| jpmc/marker-factory | Factory | 88 | tp1v67pppdudcpdddkn8wlgwh7fzwrqjqw7juwxe63fmtnwf758s5fsa7qrla | 604 | tp1ve5ydzcdrpww2s6mdncw8qvqaemgp9cp9zacn7jcter9hz8anscqf2t070 | Production |
| price-feed | Oracle | - | - | - | - | Development |
| treasury-bond | Finance | - | - | - | - | Production |
| fund | Finance | - | - | - | - | Production |

---

## Architecture & Design Patterns

### 1. Token Management Hierarchy
```
Factory Contract
    ↓
Creates ↓ instances of
    ↓
├── CW20-Marker (restricted tokens)
├── Custom-Marker (institutional tokens)
└── Treasury-Bond/Fund (specialized instruments)
```

### 2. Cross-Chain Communication
```
Provenance Blockchain
    ↓
Axelar Router / GMP Sample
    ↓
EVM Chains (Ethereum, Polygon, etc.)
```

### 3. Oracle & Pricing Integration
```
Price-Feed Contract (Provenance)
    ↓ (IBC)
    ↓
BandChain Oracle
    ↓
External Data Providers
```

### 4. Multi-Signature Authorization
```
Interop-Multisig Contract
    ↓
Validates threshold signatures
    ↓
Routes to Interop-Core
```

---

## Key Features & Capabilities

### Financial Features
- ✅ Token creation and lifecycle management
- ✅ Supply caps and restrictions
- ✅ Account-level balance limits
- ✅ Frozen balance management
- ✅ Geographic/country restrictions
- ✅ Capital call management
- ✅ Treasury bond issuance
- ✅ Multi-party fund operations

### Technical Features
- ✅ Smart contract upgradeability via migration
- ✅ Cross-chain message passing (EVM ↔ Cosmos)
- ✅ Oracle integration (BandChain)
- ✅ Multi-signature authorization
- ✅ IBC communication
- ✅ Institutional forced-transfer capability
- ✅ Comprehensive error handling
- ✅ Message fees and gas pricing

### Compliance Features
- ✅ Country/region-based authorization
- ✅ Account freezing
- ✅ Institutional audit trails
- ✅ Admin-controlled operations
- ✅ Access control on markers

---

## Code Organization Patterns

### Per-Contract Structure
```
contracts/<name>/
├── Cargo.toml              # Dependencies
├── Makefile               # Build targets
├── README.md              # Documentation
├── rustfmt.toml           # Code formatting
├── src/
│   ├── lib.rs             # Public API
│   ├── contract.rs        # Entry points (instantiate, execute, query)
│   ├── msg.rs             # Message definitions
│   ├── state.rs           # Storage state
│   ├── error.rs           # Custom errors
│   ├── handler.rs         # Message handlers
│   ├── helper.rs          # Utility functions
│   ├── enums.rs           # Enumerations
│   ├── structs.rs         # Data structures
│   ├── types.rs           # Type definitions
│   ├── ibc.rs             # IBC packet handling
│   └── tests.rs           # Unit tests
├── schema/                # Generated JSON schemas
├── parameters/            # Sample instantiation parameters (JSON)
└── target/               # Build artifacts
```

### Entry Point Pattern
All contracts follow standard CosmWasm entry points:
```rust
#[entry_point]
pub fn instantiate(deps, env, info, msg) -> Result<Response, Error>

#[entry_point]
pub fn execute(deps, env, info, msg) -> Result<Response, Error>

#[entry_point]
pub fn query(deps, env, msg) -> Result<Binary, Error>

#[entry_point]
pub fn migrate(deps, env, msg) -> Result<Response, Error>
```

---

## Dependencies & Versioning

### Key Version Constraints
- **Go**: 1.23 (strict requirement for Provenance)
- **Rust**: Stable (as of contract compilation)
- **CosmWasm**: Latest compatible version
- **Cosmos SDK**: v0.47+
- **CometBFT**: Latest compatible
- **Docker**: cosmwasm/rust-optimizer:0.12.13 (WASM optimization)

### Critical Provenance Dependencies
- `provwasm_std`: Blockchain bindings
- `cw_storage_plus`: Storage abstractions
- `cw2`: Versioning support
- `cw20`/`cw20_base`: Token standards

---

## Testing Strategy

### Unit Testing
- Located in `tests/` modules or test files
- Executed via `cargo test`
- Test categories:
  - Message validation
  - State transitions
  - Error conditions
  - Access control

### Integration Testing
- Manual testing via CLI commands
- Parameter files in `parameters/` directory
- Testnet deployment verification
- Cross-contract interactions

### Network Testing
- Devnet: Local testing with custom configuration
- Testnet: Public testing network
- Mainnet: Production network

---

## Security Considerations

### Built-In Security
1. **Admin Control**: Critical operations restricted to contract admin
2. **Access Control**: Marker-level permissions
3. **Error Handling**: Comprehensive error types with `thiserror`
4. **Type Safety**: Rust's compile-time type checking
5. **Storage Safety**: `cw_storage_plus` abstractions

### Risk Areas
1. **Cross-Chain Bridges**: Requires trusted oracle/Axelar verification
2. **Price Feeds**: Depends on BandChain oracle reliability
3. **Multi-Sig Threshold**: Configuration critical for security
4. **Forced Transfers**: Institutional feature requiring careful gating

### Audit Recommendations
- Code review of critical paths (transfers, minting)
- Oracle reliability verification
- Access control matrix validation
- Cross-chain message verification

---

## Deployment & Operations

### Prerequisites
1. Go 1.23 installed
2. Rust toolchain with `wasm32-unknown-unknown` target
3. Docker for WASM optimization
4. Provenance blockchain binaries (from `make start`)

### Deployment Steps

1. **Compile Blockchain**
   ```bash
   cd build
   make start  # Clones and compiles Provenance
   ```

2. **Build Contracts**
   ```bash
   cd contracts/<name>
   docker run --rm -v "$(pwd)":/code ... cosmwasm/rust-optimizer:0.12.13
   ```

3. **Store Contracts**
   ```bash
   provenanced tx wasm store <wasm-file> --from validator
   ```

4. **Instantiate Contracts**
   ```bash
   provenanced tx wasm instantiate <code-id> '<init-msg>' --label "name"
   ```

### Monitoring
- Process ID tracking via `blockchain_process.pid`
- Block explorer monitoring (Testnet: https://test.provenance.io)
- Contract state queries via CLI
- Transaction tracking and verification

---

## File Structure Summary

| Path | Purpose |
|------|---------|
| `build/Makefile` | Main orchestration, blockchain setup |
| `build/provnode/provenance/` | Provenance blockchain core (Go) |
| `build/run/provenanced/` | Runtime directory, config, keys |
| `contracts/*/` | 11+ CosmWasm contracts (Rust) |
| `contracts/*/src/lib.rs` | Contract entry points |
| `contracts/*/src/msg.rs` | Message definitions (Execute, Query, Instantiate) |
| `contracts/*/parameters/` | Sample/test instantiation messages |
| `contracts/jpmc/` | JPMC-specific contract variants |

---

## Summary

This is a **sophisticated blockchain deployment system** combining:

1. **Provenance Blockchain**: A purpose-built financial services blockchain
2. **Smart Contracts Suite**: 11+ production-ready CosmWasm contracts
3. **Cross-Chain Bridge**: EVM ↔ Cosmos interoperability
4. **Oracle Integration**: Price feeds and external data
5. **Institutional Features**: Multi-sig, compliance controls, forced transfers
6. **Build Infrastructure**: Automated deployment and testing

**Target Use Cases**:
- Tokenization of financial instruments
- Cross-chain asset management
- Capital calls and fund operations
- Treasury bond issuance
- Institutional trading with geographic restrictions
- Real-time price data integration

**Maturity Level**: Production-ready with ongoing development for new features (price-feed refinements, additional cross-chain bridges)

---

## Additional Resources

- **Provenance Documentation**: https://provenance.io
- **Cosmos SDK**: https://docs.cosmos.network
- **CosmWasm**: https://docs.cosmwasm.com
- **Axelar GMP**: https://docs.axelar.dev
- **BandChain Oracle**: https://docs.bandchain.org
- **CW20 Standard**: CW20 token specification

---

*Last Updated: December 23, 2025*
*Repository Source: https://github.com/Oasis-Pro-Inc/provenancesto*
