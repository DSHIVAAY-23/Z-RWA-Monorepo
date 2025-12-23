# Cosmwasm SendReceive Smart Contract
This project contains the Cosmwasm smart contract that can send and receive message payloads to/from EVM.

This contract is deployed to osmosis-5 testnet: `osmo12uds53qp285w68vzq9wx5kjjfj0evvsy7k0e2srx9mchj29h7urq3rtuuc`

# Tests
Unit tests can be run with `cargo test`

# Build and deploy
This process assumes you're using osmosisd CLI: https://docs.osmosis.zone/osmosis-core/osmosisd/

1. Build
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.13
  ```

## Testnet Deployment Details

### contract_id
```
608
```

### contract_address
```
tp1wjea0da3kzt7rcddjyvf9gxf7nkvzuc89dkfrdh3ywqnz7kt6z5qxpkvxk
```

## Devnet Deployment Details

### contract_id
```
65
```

### contract_address
```
tp1d8lzewx67da62k4ax5gcz4h90w236gnehfhx65y5ly24zwgdcyuscc48wx
```
