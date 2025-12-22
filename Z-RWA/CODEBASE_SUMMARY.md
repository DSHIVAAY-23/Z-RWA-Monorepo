# SOLANA CODEBASE TECHNICAL SUMMARY

## 1. Directory Tree & Architecture
The codebase is organized into a workspace containing multiple Anchor programs.

```
/programs
├── base-token-program  (ID: 7iaDbVGbVJdhZcKXWQmMw783nBqfDyx6K8V4yF6Kv8iq)
├── fund-contract       (ID: H9txrMrTfGU6LXYWBzMHzmzuQWbNYcW1vHFLRcxCYiKn)
├── interop-core        (ID: 5dyQmihDcQqCBerQg82J19QxMByjpxbwdtmCuPkT9ePD)
├── interop-multisig    (ID: 2uuBHq3teujBvfE3AnRm4LZFYk7sHUiC36Z9MdkgcJ2N)
├── token-program       (ID: 33pHXYQbe41JJSA7oXor6h7JFY74eqH25xtSjBysmTYo)
└── treasury-bond       (ID: 53cKZWTPTwLjXD7E6NHCQzn5Gs9KL81u5xqicHEJwNgE)
```

## 2. Program Overview & Responsibilities

### Base Token Program (`7iaDb...`)
**Primary Responsibility:** Manages the lifecycle of Real World Asset (RWA) tokens. It is an implementation of a controlled token system using Token2022.
- **Core Instructions:**
  - `create_token`: Initializes a new Token2022 mint with extensions.
  - `mint_token`: Mints tokens to a user, checking admin/issuer authority and freeze status.
  - `burn_token / burn_token_from`: Burns tokens.
  - `freeze_user_account / unfreeze`: Compliance controls to freeze assets.
  - `request_orders`: Enables other programs (like Interop) to trigger mint/burn operations via CPI.

### Fund Contract (`H9txr...`)
**Primary Responsibility:** Manages investment funds, dealing with stablecoin deposits and token issuance (minting/burning). It acts as a high-level manager that orchestrates token movements.
- **Core Instructions:**
  - `create_fund`: Sets up a new fund configuration.
  - `distribute_and_burn`: Handles redemption/exit; swaps investor's tokens (burns them) for stablecoins (transfers them).
  - `share_dividends`: Distributes rewards/dividends.
  - **Interaction:** deeply integrated with `token-program` (not base-token-program) for asset management.

### Interop Core (`5dyQ...`)
**Primary Responsibility:** Handles cross-chain interactions. It receives instructions (proof of burn/lock on another chain) and executes corresponding actions (minting) on Solana.
- **Core Instructions:**
  - `mint_tokens`: Called when assets are bridged in. Triggers `request_orders` on `base-token-program`.
  - `execute_instruction`: General purpose cross-chain instruction executor.
  - `manage_roles`: Validation handling.

### Interop Multisig (`2uuBH...`)
**Primary Responsibility:** Provides multisignature governance *specifically* for the interop layer, likely validating cross-chain payloads.
- **Core Instructions:**
  - `cast_votes`: Validators verify tx hashes.
  - `execute_transactions`: Executes payload if threshold is met.

### Token Program (`33pHXY...`)
**Primary Responsibility:** Similar to Base Token Program but appears to be the specific instance used by `fund-contract`.
- **Note:** Shares almost identical usage patterns with Base Token Program but deployed as a separate ID, possibly for separation of concerns between "RWA/Fund Assets" and "Base Assets".

## 3. The PDA Map

### Base Token Program & Token Program
| Account | Seeds | Purpose |
| :--- | :--- | :--- |
| **Maintainers** | `[b"maintainers"]` | Stores admin and sub-admin lists. |
| **Config** | `[b"config", token_name_bytes]` | Stores metadata like issuer, agents, and limits. |
| **Mint** | `[b"mint", token_name_bytes]` | The actual SPL Token2022 Mint account. Authority is this PDA. |

### Fund Contract
| Account | Seeds | Purpose |
| :--- | :--- | :--- |
| **Global Config** | `[b"global-config", token_name_bytes]` | Stores fund state, fund manager config. |
| **Agent** | `[b"agent", token_name_bytes]` | Stores the authorized agent for fund operations. |

### Interop Core
| Account | Seeds | Purpose |
| :--- | :--- | :--- |
| **Maintainers** | `[b"maintainers"]` | Stores interop admins. |
| **Executer** | `[b"executer"]` | Stores the multisig address/state for execution. |
| **Request** | `[order_id_bytes]` | Stores status of a specific order to prevent replay. |

## 4. CPI Handshake (Program Communication)

It is critical to understand the two distinct flows:

**Flow A: Fund Redemption (Fund Contract -> Token Program)**
1. **Trigger:** `distribute_and_burn` is called on `fund-contract`.
2. **Action 1 (Stablecoin):** `fund-contract` calls `Token2022::TransferChecked` to send stablecoins from Agent -> Investor.
3. **Action 2 (Burn):** `fund-contract` calls `token-program::cpi::burn_token_from`.
   - **Signer:** Fund contract signs for `Global Config` PDA.
   - **Authority:** The Fund Contract itself acts as the delegate/authority to burn the investor's tokens.

**Flow B: Cross-Chain Minting (Interop Core -> Base Token Program)**
1. **Trigger:** `mint_tokens` is called on `interop-core` (after multisig verification).
2. **Action:** `interop-core` calls `base-token-program::cpi::request_orders`.
3. **Mechanism:** `request_orders` on Base Token Program receives the call.
   - It verifies if the caller (Interop Core) is a `sub_admin` in `Maintainers` list.
   - If authorized, it executes `Token2022::mint_to` via a CPI signed by its own `Mint` PDA.

## 5. Token2022 Implementation & Extensions

The project heavily utilizes Token2022 to enforce compliance and ownership at the protocol level.

**Extensions Used:**
- **Mint Authority:** Controlled by the Program PDA (`[b"mint", name]`).
- **Metadata Pointer:** Pointing to self (The Mint Account stores its own metadata).
- **Group Member Pointer:** For grouping collections of tokens.
- **Close Authority:** Retained by Payer/Admin (prevents accidental closing).
- **Permanent Delegate:** Set to the Mint Account PDA.
  - **Significance:** This allows the program to force transfer or burn tokens even without user signature (essential for RWA compliance/seizure).
- **Transfer Hook:** (Commented out in code, but intended for future restriction logic).

**Initialization Snippet:**
```rust
// create_token.rs
#[account(
    init,
    seeds = [b"mint", params.name.as_bytes()],
    mint::token_program = token_program, // Enforces Token2022
    mint::extensions::permanent_delegate::delegate = mint_account, // Critical for RWA
    // ...
)]
pub mint_account: Box<InterfaceAccount<'info, Mint>>,
```

## 6. On-Chain Evidence (IDs for Verification)

To verify the deployment, use `solana program show <ID>`.

| Program Name | Program ID |
| :--- | :--- |
| **Base Token Program** | `7iaDbVGbVJdhZcKXWQmMw783nBqfDyx6K8V4yF6Kv8iq` |
| **Fund Contract** | `H9txrMrTfGU6LXYWBzMHzmzuQWbNYcW1vHFLRcxCYiKn` |
| **Interop Core** | `5dyQmihDcQqCBerQg82J19QxMByjpxbwdtmCuPkT9ePD` |
| **Interop Multisig** | `2uuBHq3teujBvfE3AnRm4LZFYk7sHUiC36Z9MdkgcJ2N` |
| **Token Program** | `33pHXYQbe41JJSA7oXor6h7JFY74eqH25xtSjBysmTYo` |

**Verification Command Template:**
```bash
solana program show 7iaDbVGbVJdhZcKXWQmMw783nBqfDyx6K8V4yF6Kv8iq
```

## 7. Logic Summary

### Mint Logic (Base Token / Token Program)
The `mint_token` instruction is gated. It does not allow public minting.
1. **Authorization:** Checks if the caller is in the `sub_admins` list, or is the designated `issuer` or `tokenization_agent`.
2. **Freeze Check:** Ensures the destination account is not frozen.
3. **Execution:** The Program derives the `Mint` PDA signer seeds (`[b"mint", name, bump]`) and calls `token_2022::mint_to`.

### Burn Logic (via Fund Contract)
Burn is usually associated with redemption.
1. **Authorization:** Caller must be the `agent` or `fund_manager`.
2. **Setup:** Stablecoins are transferred to the investor *first*.
3. **Execution:** The Fund Contract, acting as an authority, calls the Token Program to burn the specific amount of RWA tokens from the investor's wallet. This atomic swap ensures delivery-vs-payment (DvP).

### Multisig Logic (Interop)
1. **Vote:** Validators call `cast_votes` with a `tx_hash` (representing an external chain event).
2. **Threshold:** The state tracks votes. No automatic execution in `cast_votes`.
3. **Execute:** `execute_transactions` is called. It checks if `vote_count >= threshold`. If yes, it decodes the payload and triggers the Interop Core to action (e.g., mint).
