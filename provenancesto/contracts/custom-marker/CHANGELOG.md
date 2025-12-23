## Custom Marker Contract — Consolidated Changes and Features

### Status
- Artifacts: `artifacts/custom_marker.wasm` (checksums in `artifacts/checksums.txt`)
- Schemas: JSON schemas under `schema/` and `schema/raw/`
- Scripts: Localnet and Testnet helper scripts under `scripts/`

### Core Lifecycle
- Instantiate: Sets the caller as Admin. Event: `provwasm.contracts.custom_marker.init`
- Migrate: Upgrades contract code by Admin. Event: `provwasm.contracts.custom_marker.migrate`

### Marker Creation and Supply Ops
- Create restricted marker with active status, whitelist countries, token limit, roles (issuer, tokenization agent, transfer agent). Event: `provwasm.contracts.custom_marker.create`
- Withdraw from marker to account. Events: withdraw amount/denom/recipient
- Mint to marker (increase supply). Events: mint amount/denom
- Burn from marker (decrease supply). Events: burn amount/denom

### Transfers and Freezes
- Transfer from contract to recipient with whitelist and freeze checks. Event: `provwasm.contracts.custom_marker.transfer`
- Force transfer between addresses with checks. Event: `provwasm.contracts.custom_marker.force_transfer`
- Freeze/Unfreeze accounts (batch via update_type add/remove). Events: freeze/unfreeze
- Partial freeze/unfreeze balances per address (batch). Events: partial_freeze/partial_unfreeze

### Whitelisting and Config
- Whitelist addresses per denom by country code: set/unset (batch). Events: whitelist.set / whitelist.unset
- Update authorized country codes per denom: add/remove. Events: add_country_code / remove_country_code
- Update token holding limit per denom. Event: update_token_limit

### Role Management
- Manage roles via `manage_roles` (batch):
  - Issuer: add/remove. Events: add_issuer / remove_issuer
  - Transfer Agent: add/remove. Events: add_transfer_agent / remove_transfer_agent
  - Tokenization Agent: add/remove. Events: add_tokenization_agent / remove_tokenization_agent
  - Sub Admins: add/remove (admin-only). Events: add_sub_admins / remove_sub_admins
  - Admin: update admin address. Event: update_admin
  - Agents: grant/ungrant marker accesses (admin/burn/deposit/delete/mint/transfer/withdraw/freeze/unfreeze/force_transfer). Events: grant_access / ungrant_access

### Marker State Ops
- Cancel marker (requires delete access). Event: cancel
- Destroy marker (requires delete access). Event: destroy

### Queries
- get_admin, get_sub_admins
- get_by_address, get_by_denom (returns Marker struct)
- get_authorized_countries, get_country_code_by_address
- get_freezed_accounts, get_frozen_balance, get_frozen_tokens
- get_denom_config (token_limit, country_codes)
- get_balance, get_ciculating_supply

### Parameters Templates
- JSON templates under `parameters/` for all execute operations, including:
  - Role updates: `addIssuer.json`, `removeIssuer.json`, `addTransferAgent.json`, `removeTransferAgent.json`, `addTokenizationAgent.json`, `removeTokenizationAgent.json`, `addSubAdmin.json`, `removeSubAdmin.json`, `grantAccess.json`, `ungrantAccessFromAgent.json`
  - Marker ops: `createMarker.json`, `withdraw.json`, `mint.json`, `burn.json`, `mintTo.json`, `burnFrom.json`, `send.json`, `forceTransfer.json`, `cancel.json`, `destroy.json`
  - Freeze/whitelist/config: `freeze.json`, `unfreeze.json`, `addToPartialFreeze.json`, `subFromFrozenList.json`, `setWhitelist.json`, `unsetWhitelist.json`, `addCountryCode.json`, `removeCountryCode.json`, `updateTokenLimit.json`
  - Queries: multiple `get*.json` files (admin, config, balances, etc.)

### Scripts
- Localnet scripts under `scripts/localnet/`:
  - auth: `auth.sh`
  - init & instantiate: `init/init.sh`, `init/instantiate_custom_marker.sh`, `init/name.sh`
  - deploy/migrate: `deploy/deploy_custom_marker.sh`, `deploy/deploy.sh`, `migrate/migrate.sh`
  - execute: 30+ helper scripts for each execute entrypoint
  - query: 10+ helper scripts for each query
  - balance/utilities under `balance/`
- Testnet scripts under `scripts/testnet/` mirroring localnet (auth, deploy, migrate, execute, query)

### Schema Files
- Top-level: `schema/custom-marker.json`
- Raw request/response schemas in `schema/raw/` for instantiate, execute, query, and specific responses

### Deployment Notes (from README)
- Example deployment references:
  - contract_id: 686
  - contract_address: `tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3`

### Notes
- All execute and query shapes are documented with examples in `README.md`.
- Events are emitted for each major operation to aid indexing and auditability.



