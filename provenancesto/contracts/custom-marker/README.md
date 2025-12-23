# Marker Contract 3643

This smart contract is use for tokenizing a Fungible Assets in Provenance Blockchain based on Marker module.

---

## Introduction

The Token project aims to provide sets of modules for creating Fungible Tokens in Provenance blockchain.

## Functions

### Function `instantiate`

This function is use to initialize the smart contract with new address. In this function the caller is set as Admin. Only this caller can call `migrate` function for smart contract upgradation.

#### Events
```
provwasm.contracts.custom_marker.init
```

#### Parameters
```
{}
```

#### Script
```
provenanced tx wasm instantiate <code_id> \
	'{}' \
    --admin <caller> \
    --label <contract-name> \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `migrate`

This function is use to perform smart contract upgradability in case of logic changes or upgradation. Only the smart contract initializer account can call this function.

#### Events
```
provwasm.contracts.custom_marker.migrate
```

#### Parameters
```
{}
```

#### Script
```
provenanced tx wasm migrate <old_contract_address> <new_code_id> \
	'{}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `create`

This function is use to create new restricted marker with active status. In this function call
1. a list of country codes are stored for allowing transactions only from whitelisted countries.
2. Token limit are alloted so that no user can hold tokens above this limit.
3. Issuer address is assigned for mint, burn, force_transfer, freeze and unfreeze operations.
4. Tokenization Agent address is assigned for mint and burn operations.
5. Transfer Agent address is assigned for force_transfer, freeze and unfreeze operations.

#### Fails when
1. caller is not sub_admin
2. supply is lesser than token_limit

#### Events
```
1. provwasm.contracts.custom_marker.create
2. marker_supply
3. marker_denom
4. denom_id
```

#### Parameters
```
{
    "create": {
        "params": {
            "supply": "<initial_supply: string with unsigned 128 bit integer>",
            "denom": "<token_name: string>",
            "denom_config": {
                "token_limit": "<token_limit: string with unsigned 128 bit integer>",
                "country_codes": [
                    <list of unsigned 8 bit integer>
                ]
            },
            "id": "<unique_id: string>",
            "issuer": "<address: string>",
            "tokenization_agent": "<address: string>",
            "transfer_agent": "<address: string>"
        }
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "create": {
        "params": {
            "supply": "<initial_supply: string with unsigned 128 bit integer>",
            "denom": "<token_name: string>",
            "denom_config": {
                "token_limit": "<token_limit: string with unsigned 128 bit integer>",
                "country_codes": [
                    <list of unsigned 8 bit integer>
                ]
            },
            "id": "<unique_id: string>",
            "issuer": "<address: string>",
            "tokenization_agent": "<address: string>",
            "transfer_agent": "<address: string>"
        }
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `withdraw`

This function is use to withdraw tokens from marker address to an account.

#### Fails when
1. caller is not sub_admin
2. amount is zero

#### Events
```
1. provwasm.contracts.custom_marker.withdraw
2. withdraw_amount
3. withdraw_denom
4. withdraw_recipient
```

#### Parameters
```
{
    "withdraw": {
        "denom": "<token_name: string>",
        "amount": "amount: string with unsigned 128 bit integer"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "withdraw": {
        "denom": "<token_name: string>",
        "amount": "amount: string with unsigned 128 bit integer"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `mint`

This function is use to mint tokens to marker address and increases the marker supply.

#### Fails when
1. caller is not sub_admin
2. amount is zero

#### Events
```
1. provwasm.contracts.custom_marker.mint
2. mint_amount
3. mint_denom
```

#### Parameters
```
{
    "mint": {
        "denom": "<token_name: string>",
        "amount": "amount: string with unsigned 128 bit integer"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "mint": {
        "denom": "<token_name: string>",
        "amount": "amount: string with unsigned 128 bit integer"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `burn`

This function is use to burn tokens from marker address and decreases the marker supply.

#### Fails when
1. caller is not sub_admin
2. amount is zero

#### Events
```
1. provwasm.contracts.custom_marker.burn
2. burn_amount
3. burn_denom
```

#### Parameters
```
{
    "burn": {
        "denom": "<token_name: string>",
        "amount": "amount: string with unsigned 128 bit integer"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "burn": {
        "denom": "<token_name: string>",
        "amount": "amount: string with unsigned 128 bit integer"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `cancel`

This function is use to cancel marker. Marker state will be changed to cancelled state, i.e. `MarkerStatus::Cancelled`

#### Fails when
1. caller does not have delete access

#### Events
```
1. provwasm.contracts.custom_marker.cancel
2. marker_denom
```

#### Parameters
```
{
    "cancel": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "cancel": {
        "denom": "<token_name: string>"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `destroy`

This function is use to destroy marker. Marker state will be changed to destroyed state, i.e. `MarkerStatus::Destroyed`

#### Fails when
1. caller does not have delete access

#### Events
```
1. provwasm.contracts.custom_marker.destroy
2. marker_denom
```

#### Parameters
```
{
    "destroy": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "destroy": {
        "denom": "<token_name: string>"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `transfer`

This function is use to transfer coins from contract to recipient address.

#### Fails when
1. recipient is not whitelisted
2. recipient is freezed
3. amount is zero
4. amount exceeds the alloted token_limit

#### Events
```
1. provwasm.contracts.custom_marker.transfer
2. funds
3. to
4. from
```

#### Parameters
```
{
    "transfer": {
        "amount": "<amount: string with unsigned 128 bit integer>",
        "denom": "<token_name: string>",
        "to": "<address: string>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "transfer": {
        "amount": "<amount: string with unsigned 128 bit integer>",
        "denom": "<token_name: string>",
        "to": "<address: string>"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `freeze`

This function is use to freeze and unfreeze account. If the account is freezed than it will not able to perform any transactions. Both freeze and unfreeze can be performed by same entrypoint, based on update_type. For freeze `update_type` will be `UpdateType::Add(Addresses)` and for unfreeze `update_type` will be `UpdateType::Remove(Addresses)`. This function supports batch operations, i.e. multiple addresses can be freezed / unfreezed simultaneously.

#### Fails when
1. caller is not issuer, transfer_agent, sub_admin or having freeze access for freezing
2. caller is not issuer, transfer_agent, sub_admin or having unfreeze access for unfreezing
3. freeze list doesn't contain the address that is going to be removed

#### Events
```
case 1. During addition, i.e., update_type = UpdateType::Add(Addresses)
    then
        1. provwasm.contracts.custom_marker.freeze
        2. update_type
case 2. During removal, i.e., update_type = UpdateType::Remove(Addresses)
    then
        1. provwasm.contracts.custom_marker.unfreeze
        2. update_type
```

#### Parameters
For freeze
```
{
    "freeze": {
        "denom": "<token_name: string>",
        "update_type": [
            {
                "add": "<address: string>"
            },
            {
                "add": "<address: string>"
            }, 
            ..
        ]
    }
}
```

For unfreeze
```
{
    "freeze": {
        "denom": "<token_name: string>",
        "update_type": [
            {
                "remove": "<address: string>"
            },
            {
                "remove": "<address: string>"
            }, 
            ..
        ]
    }
}
```

#### Script
For freeze
```
provenanced tx wasm execute <contract_address> \
	'{
    "freeze": {
        "denom": "<token_name: string>",
        "update_type": [
            {
                "add": "<address: string>"
            },
            {
                "add": "<address: string>"
            }, 
            ..
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

For unfreeze
```
provenanced tx wasm execute <contract_address> \
	'{
    "freeze": {
        "denom": "<token_name: string>",
        "update_type": [
            {
                "remove": "<address: string>"
            },
            {
                "remove": "<address: string>"
            }, 
            ..
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `partial_freeze`

This function is use to freeze and unfreeze partial balance of accounts. When the tokens are partially freezed then entire amount can't be used in any transactions, only unfreezed tokens can take part in the transactions. Both partial freeze and unfreeze can be performed by same entrypoint, based on update_type. For partial freeze `update_type` will be `UpdateType::Add(Uint128)` and for partial unfreeze `update_type` will be `UpdateType::Remove(Uint128)`. This function supports batch operations, i.e. multiple addresses can be freezed / unfreezed simultaneously.

#### Fails when
1. caller is not issuer, transfer_agent, sub_admin or having freeze access for partial freeze
2. caller is not issuer, transfer_agent, sub_admin or having unfreeze access for partial unfreeze
3. partial freeze list doesn't contain the address that is going to be removed
4. the frozen token list is empty

#### Events
```
case 1. During addition, i.e., update_type = UpdateType::Add(Uint128)
    then
        1. provwasm.contracts.custom_marker.partial_freeze
        2. address
case 2. During removal, i.e., update_type = UpdateType::Remove(Uint128)
    then
        1. provwasm.contracts.custom_marker.partial_unfreeze
        2. address
```

#### Parameters
For partial freeze
```
{
    "partial_freeze": {
        "denom": "<token_name: string>",
        "params": [
            {
                "address": "<address: string>",
                "update_type": {
                    "add": "<amount: string with unsigned 128 bit integer>"
                }
            },
            {
                "address": "<address: string>",
                "update_type": {
                    "add": "<amount: string with unsigned 128 bit integer>"
                }
            },
            ...
        ]
    }
}
```

For unfreeze
```
{
    "partial_freeze": {
        "denom": "<token_name: string>",
        "params": [
            {
                "address": "<address: string>",
                "update_type": {
                    "remove": "<amount: string with unsigned 128 bit integer>"
                }
            },
            {
                "address": "<address: string>",
                "update_type": {
                    "remove": "<amount: string with unsigned 128 bit integer>"
                }
            },
            ...
        ]
    }
}
```

#### Script
For partial freeze
```
provenanced tx wasm execute <contract_address> \
	'{
    "partial_freeze": {
        "denom": "<token_name: string>",
        "params": [
            {
                "address": "<address: string>",
                "update_type": {
                    "add": "<amount: string with unsigned 128 bit integer>"
                }
            },
            {
                "address": "<address: string>",
                "update_type": {
                    "add": "<amount: string with unsigned 128 bit integer>"
                }
            },
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

For partial unfreeze
```
provenanced tx wasm execute <contract_address> \
	'{
    "partial_freeze": {
        "denom": "<token_name: string>",
        "params": [
            {
                "address": "<address: string>",
                "update_type": {
                    "remove": "<amount: string with unsigned 128 bit integer>"
                }
            },
            {
                "address": "<address: string>",
                "update_type": {
                    "remove": "<amount: string with unsigned 128 bit integer>"
                }
            },
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `whitelist`

This function is use to whitelist address based on country_code. Only whitelisted addresses can take part in transaction. Both set whitelist and unset whitelist can be performed by same entrypoint, based on update_type. For set whitelist `update_kind` will be `UpdateKind::Set(u8)` and for unset whitelist `update_kind` will be `UpdateKind::Unset(u8)`. This function supports batch operations, i.e. multiple addresses can be whitelisted and removed from whitelist simultaneously.

#### Fails when
1. caller is not sub_admin or tokenization agent
3. the list already contains the address for addition
4. the list doesn't have the address for removal

#### Events
```
case 1. update_kind = UpdateKind::Set(u8)
    then
        1. provwasm.contracts.custom_marker.whitelist.set
        2. address
case 2. update_kind = UpdateKind::Unset(u8)
    then
        1. provwasm.contracts.custom_marker.whitelist.unset
        2. address
```

#### Parameters
For set whitelist
```
{
  "whitelist": {
    "lists": [
      {
        "denom": "<token_name: string>",
        "data": [
          {
            "address": "<address: string>",
            "country_code": {
              "set": <unsigned 8 bit integer>
            }
          },
          {
            "address": "<address: string>",
            "country_code": {
              "set": <unsigned 8 bit integer>
            }
          },
          ...
        ]
      }
    ]
  }
}
```

For unset whitelist
```
{
  "whitelist": {
    "lists": [
      {
        "denom": "<token_name: string>",
        "data": [
          {
            "address": "<address: string>",
            "country_code": {
              "unset": {}
            }
          },
          {
            "address": "<address: string>",
            "country_code": {
              "unset": {}
            }
          },
          ...
        ]
      }
    ]
  }
}
```

#### Script
For set whitelist
```
provenanced tx wasm execute <contract_address> \
	'{
  "whitelist": {
    "lists": [
      {
        "denom": "<token_name: string>",
        "data": [
          {
            "address": "<address: string>",
            "country_code": {
              "set": <unsigned 8 bit integer>
            }
          },
          {
            "address": "<address: string>",
            "country_code": {
              "set": <unsigned 8 bit integer>
            }
          },
          ...
        ]
      }
    ]
  }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

For unset whitelist
```
provenanced tx wasm execute <contract_address> \
	'{
  "whitelist": {
    "lists": [
      {
        "denom": "<token_name: string>",
        "data": [
          {
            "address": "<address: string>",
            "country_code": {
              "unset": {}
            }
          },
          {
            "address": "<address: string>",
            "country_code": {
              "unset": {}
            }
          },
          ...
        ]
      }
    ]
  }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `update_country_code`

This list contains set of country codes that are authorised to transact on this platform. Both add and remove can be performed by this single entry function. To add country_code `update_type` will be `UpdateType::Add(u8)` and for unset whitelist `update_type` will be `UpdateType::Remove(u8)`.

#### Fails when
1. caller is not sub_admin
3. denom config (also known as token config) is not available for particular denom

#### Events
```
case 1. During addition, i.e., update_type = UpdateType::Add(u8)
    then
        1. provwasm.contracts.custom_marker.add_country_code
        2. country_code
case 2. During removal, i.e., update_type = UpdateType::Remove(u8)
    then
        1. provwasm.contracts.custom_marker.remove_country_code
        2. country_code
```

#### Parameters
For addition of new country code
```
{
    "update_country_code": {
        "denom": "<token_name: string>",
        "update_type": {
            "add": <unsigned 8 bit integer>
        }
    }
}
```

For removal of country code
```
{
    "update_country_code": {
        "denom": "<token_name: string>",
        "update_type": {
            "remove": <unsigned 8 bit integer>
        }
    }
}
```

#### Script
For addition of new country code
```
provenanced tx wasm execute <contract_address> \
	'{
    "update_country_code": {
        "denom": "<token_name: string>",
        "update_type": {
            "add": <unsigned 8 bit integer>
        }
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

For removal of country code
```
provenanced tx wasm execute <contract_address> \
	'{
    "update_country_code": {
        "denom": "<token_name: string>",
        "update_type": {
            "remove": <unsigned 8 bit integer>
        }
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `update_token_limit`

This list contains token limit, i.e., the maximum permissible token that an account can hold.

#### Fails when
1. caller is not sub_admin
3. denom config (also known as token config) is not available for particular denom

#### Events
```
1. provwasm.contracts.custom_marker.update_token_limit
2. new_limit
```

#### Parameters
```
{
    "update_token_limit": {
        "denom": "<token_name: string>",
        "limit": "<token_limit: string with unsigned 128 bit integer>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "update_token_limit": {
        "denom": "<token_name: string>",
        "limit": "<token_limit: string with unsigned 128 bit integer>"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `send`

Function to send token marker coins to recipient address.

#### Fails when
1. sender doesn't have transfer access
2. recipient is not whitelisted
3. recipient is freezed
4. amount is zero
5. amount exceeds the alloted token_limit
6. unfreezed balance is lesser than requested amount

#### Events
```
1. provwasm.contracts.custom_marker.send
2. funds
3. to
4. from
```

#### Parameters
```
{
    "send": {
        "denom": "<token_name: string>",
        "to": "<address: string>",
        "amount": "<amount: string with unsigned 128 bit integer>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "send": {
        "denom": "<token_name: string>",
        "to": "<address: string>",
        "amount": "<amount: string with unsigned 128 bit integer>"
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `mint_to`

Function to mint tokens to various addresses. This function supports batch operations, i.e. multiple addresses can be minted simultaneously.

#### Fails when
1. caller is not sub_admin, issuer, tokenization agent or having mint access
2. recipient is not whitelisted
3. recipient is freezed
4. amount is zero
5. amount exceeds the alloted token_limit

#### Events
```
provwasm.contracts.custom_marker.mint_to
```

#### Parameters
```
{
    "mint_to": {
        "mint_to_params": [
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            },
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            }
            ...
        ]
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "mint_to": {
        "mint_to_params": [
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            },
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `burn_from`

Function to burn tokens from various addresses. This function supports batch operations, i.e. multiple addresses can be burned simultaneously. This function call decreases the total supply.

#### Fails when
1. caller is not sub_admin, issuer, tokenization agent or having burn access
2. recipient is not whitelisted
3. recipient is freezed
4. amount is zero

#### Events
```
provwasm.contracts.custom_marker.burn_from
```

#### Parameters
```
{
    "burn_from": {
        "burn_from_params": [
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            },
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            }
            ...
        ]
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "burn_from": {
        "burn_from_params": [
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            },
            {
                "denom": "<token_name: string>",
                "mint_burn_data": [
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    {
                        "address": "<address: string>"",
                        "amount": "<amount: string with unsigned 128 bit integer>"
                    },
                    ...
                ]
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `force_transfer`

Function to transfer tokens forcefully from various addresses. This function supports batch operations, i.e. multiple amounts can be transferred from the accounts simultaneously.

#### Fails when
1. caller is not sub_admin, issuer, transfer agent or having force_transfer access
2. recipient is not whitelisted
3. caller is not whitelisted
4. recipient is freezed
5. amount is zero
6. caller is freezed
7. unfreezed balance is lesser than requested amount

#### Events
```
1. provwasm.contracts.custom_marker.force_transfer
2. funds
3. to
4. from
```

#### Parameters
```
{
    "force_transfer": {
        "denom": "<token_name: string>",
        "params": [
            {    
                "amount": "<amount: string with unsigned 128 bit integer>"
                "to": "<to: string representing recipient address>",
                "from": "<from: string representing sender address>",
            },
            {    
                "amount": "<amount: string with unsigned 128 bit integer>"
                "to": "<to: string representing recipient address>",
                "from": "<from: string representing sender address>",
            }
            ...
        ]
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "force_transfer": {
        "denom": "<token_name: string>",
        "params": [
            {    
                "amount": "<amount: string with unsigned 128 bit integer>"
                "to": "<to: string representing recipient address>",
                "from": "<from: string representing sender address>",
            },
            {    
                "amount": "<amount: string with unsigned 128 bit integer>"
                "to": "<to: string representing recipient address>",
                "from": "<from: string representing sender address>",
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `manage_roles`

Function to manage different roles. This function supports batch operations, i.e. multiple roles can be assigned or unassigned simultaneously. This entry point can be use to modify different roles such as:-
1. Issuer
2. Transfer Agent
3. Tokenization Agent
4. Sub Admins
5. Admin
6. Agents

Based on the `update_type` field addresses can be added or removed:-
1. for addition `update_type` will be `UpdateType::Add(Addresses)`
2. for removal `update_type` will be `UpdateType::Remove(Addresses)`

#### Manage `issuer`
Issuer can be added or removed based on the `update_type`, for addition `update_type` will be `UpdateType::Add(Addresses)` whereas for removal `update_type` will be `UpdateType::Remove(Addresses)`.

##### Function `add_issuer`
Function to add new issuer. Basically replaced the old address with new address other than existing one.

###### Fails when
1. caller is not sub_admin
2. address already stored as Issuer

###### Events
```
provwasm.contracts.custom_marker.add_issuer
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "issuer": {
                    "update_type": {
                        "add": "<address: string representing new issuer address>"
                    }
                }
            },
            {
                "issuer": {
                    "update_type": {
                        "add": "<address: string representing new issuer address>"
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "issuer": {
                    "update_type": {
                        "add": "<address: string representing new issuer address>"
                    }
                }
            },
            {
                "issuer": {
                    "update_type": {
                        "add": "<address: string representing new issuer address>"
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

##### Function `remove_issuer`
Function to remove existing issuer.

###### Fails when
1. caller is not sub_admin
2. address doesn't stored as Issuer

###### Events
```
provwasm.contracts.custom_marker.remove_issuer
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "issuer": {
                    "update_type": {
                        "remove": "<address: string representing existing issuer address>"
                    }
                }
            },
            {
                "issuer": {
                    "update_type": {
                        "remove": "<address: string representing existing issuer address>"
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "issuer": {
                    "update_type": {
                        "remove": "<address: string representing existing issuer address>"
                    }
                }
            },
            {
                "issuer": {
                    "update_type": {
                        "remove": "<address: string representing existing issuer address>"
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

#### Manage `transfer_agent`
Transfer Agent can be added or removed based on the `update_type`, for addition `update_type` will be `UpdateType::Add(Addresses)` whereas for removal `update_type` will be `UpdateType::Remove(Addresses)`.

##### Function `add_transfer_agent`
Function to add new transfer_agent. Basically replaced the old address with new address other than existing one.

###### Fails when
1. caller is not sub_admin
2. address already stored as Transfer Agent

###### Events
```
provwasm.contracts.custom_marker.add_transfer_agent
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "transfer_agent": {
                    "update_type": {
                        "add": "<address: string representing new transfer_agent address>"
                    }
                }
            },
            {
                "transfer_agent": {
                    "update_type": {
                        "add": "<address: string representing new transfer_agent address>"
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "transfer_agent": {
                    "update_type": {
                        "add": "<address: string representing new transfer_agent address>"
                    }
                }
            },
            {
                "transfer_agent": {
                    "update_type": {
                        "add": "<address: string representing new transfer_agent address>"
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

##### Function `remove_transfer_agent`
Function to remove existing transfer_agent.

###### Fails when
1. caller is not sub_admin
2. address doesn't stored as Transfer Agent

###### Events
```
provwasm.contracts.custom_marker.remove_transfer_agent
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "transfer_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing transfer_agent address>"
                    }
                }
            },
            {
                "transfer_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing transfer_agent address>"
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "transfer_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing transfer_agent address>"
                    }
                }
            },
            {
                "transfer_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing transfer_agent address>"
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

#### Manage `tokenization_agent`
Tokenization Agent can be added or removed based on the `update_type`, for addition `update_type` will be `UpdateType::Add(Addresses)` whereas for removal `update_type` will be `UpdateType::Remove(Addresses)`.

##### Function `add_tokenization_agent`
Function to add new tokenization_agent. Basically replaced the old address with new address other than existing one.

###### Fails when
1. caller is not sub_admin
2. address already stored as Tokenization Agent

###### Events
```
provwasm.contracts.custom_marker.add_tokenization_agent
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "tokenization_agent": {
                    "update_type": {
                        "add": "<address: string representing new tokenization_agent address>"
                    }
                }
            },
            {
                "tokenization_agent": {
                    "update_type": {
                        "add": "<address: string representing new tokenization_agent address>"
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "tokenization_agent": {
                    "update_type": {
                        "add": "<address: string representing new tokenization_agent address>"
                    }
                }
            },
            {
                "tokenization_agent": {
                    "update_type": {
                        "add": "<address: string representing new tokenization_agent address>"
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

##### Function `remove_tokenization_agent`
Function to remove existing tokenization_agent.

###### Fails when
1. caller is not sub_admin
2. address doesn't stored as Tokenization Agent

###### Events
```
provwasm.contracts.custom_marker.remove_tokenization_agent
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "tokenization_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing tokenization_agent address>"
                    }
                }
            },
            {
                "tokenization_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing tokenization_agent address>"
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "tokenization_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing tokenization_agent address>"
                    }
                }
            },
            {
                "tokenization_agent": {
                    "update_type": {
                        "remove": "<address: string representing existing tokenization_agent address>"
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

#### Manage `sub_admin`
Sub Admins can be added or removed based on the `update_type`, for addition `update_type` will be `UpdateType::Add(Vec<Addresses>)` whereas for removal `update_type` will be `UpdateType::Remove(Vec<Addresses>)`.

##### Function `add_sub_admins`
Function to add new sub_admins.

###### Fails when
caller is not admin

###### Events
```
provwasm.contracts.custom_marker.add_sub_admins
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string, can be keep as empty>",
        "roles": [
            {
                "sub_admin": {
                    "update_type": {
                        "add": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string, can be keep as empty>",
        "roles": [
            {
                "sub_admin": {
                    "update_type": {
                        "add": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

##### Function `remove_sub_admins`
Function to remove existing sub_admins.

###### Fails when
caller is not admin

###### Events
```
provwasm.contracts.custom_marker.remove_sub_admins
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string, can be keep as empty>",
        "roles": [
            {
                "sub_admin": {
                    "update_type": {
                        "remove": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string, can be keep as empty>",
        "roles": [
            {
                "sub_admin": {
                    "update_type": {
                        "remove": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

#### Manage `admin`
Function to manage admin.

##### Function `add_sub_admins`
Function to update admin.

###### Fails when
caller is not admin

###### Events
```
provwasm.contracts.custom_marker.update_admin
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string, can be keep as empty>",
        "roles": [
            {
                "admin": {
                    "address": "<address: string representing new admin address>"
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string, can be keep as empty>",
        "roles": [
            {
                "admin": {
                    "address": "<address: string representing new admin address>"
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

#### Manage `agent`
Agent accesses can be added or removed based on the `update_type`, for addition `update_type` will be `UpdateType::Add(Vec<Addresses>)` whereas for removal `update_type` will be `UpdateType::Remove(Vec<Addresses>)`. There can be following accesses that can be granted to any agent:-
1. admin
2. burn
3. deposit
4. delete
5. mint
6. transfer
7. unspecified
8. withdraw
9. freeze
10. unfreeze
11. force_transfer

##### Function `grant_agent_access`
Function to provide accesses to agents

###### Fails when
caller is not sub_admin

###### Events
```
provwasm.contracts.custom_marker.grant_access
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "agent": {
                    "update_type": {
                        "add": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ],
                        "marker_access": [
                            "<string containing access type>",
                            "<string containing access type>",
                            "<string containing access type>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "agent": {
                    "update_type": {
                        "add": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ],
                        "marker_access": [
                            "<string containing access type>",
                            "<string containing access type>",
                            "<string containing access type>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

##### Function `ungrant_agent_access`
Function to remove accesses to agents.

###### Fails when
caller is not sub_admin

###### Events
```
provwasm.contracts.custom_marker.ungrant_access
```

###### Parameters
```
{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "agent": {
                    "update_type": {
                        "remove": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ],
                        "marker_access": [
                            "<string containing access type>",
                            "<string containing access type>",
                            "<string containing access type>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}
```

###### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_roles": {
        "denom": "<token_name: string>",
        "roles": [
            {
                "agent": {
                    "update_type": {
                        "remove": [
                            "<address: string representing new sub_admin address>",
                            "<address: string representing new sub_admin address>",
                            ...
                        ],
                        "marker_access": [
                            "<string containing access type>",
                            "<string containing access type>",
                            "<string containing access type>",
                            ...
                        ]
                    }
                }
            }
            ...
        ]
    }
}' \
    --from <caller> \
    --keyring-backend test \
    --home <path to key-chain file> \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_admin`

This function is used to query admin.

#### Returns
```
Admin Address
```

#### Parameters
```
{
    "get_admin": {}
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_admin": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_sub_admins`

This function is used to query list of sub_admins.

#### Returns
```
List of sub_admin addresses
```

#### Parameters
```
{
    "get_sub_admins": {}
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_sub_admins": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_by_address`

This function is used to query Marker by marker account address.

#### Fails when
1. marker account address is missing

#### Returns
```
Marker Struct
{
    "address": "<address: string, marker account address>",
    "allow_forced_transfer": <boolean: true or false, indicates force transfer is enabled or not>,
    "coins": [
      {
        "denom": "denom": "<token_name: string>",
        "amount": "<amount: string with unsigned 128 bit integer>"
      },
      {
        "denom": "denom": "<token_name: string>",
        "amount": "<amount: string with unsigned 128 bit integer>"
      },
      ...
    ],
    "account_number": <number: unsigned 64 bit integer>,
    "sequence": <number: unsigned 64 bit integer>,
    "manager": "<address: string>",
    "permissions": [
        {
            "permissions": [
                "<string: marker permissions like admin, burn, deposit, delete, mint, transfer or withdraw>"
            ],
            "address": "<address: string>"
        },
        {
            "permissions": [
                "<string: marker permissions like admin, burn, deposit, delete, mint, transfer or withdraw>"
            ],
            "address": "<address: string>"
        },
        ...
    ],
    "status": "<string: marker status can be either of unspecified, proposed, finalized, active, cancelled or destroyed>",
    "denom": "<token_name: string>",
    "total_supply": "<amount: string with unsigned 128 bit integer>",
    "marker_type": "<string, can be either of unspecified, coin or restricted>",
    "supply_fixed": <boolean, true or false, if fixed means supply cannot be increased further>
  }
```

#### Parameters
```
{
    "get_by_address": {
        "address": "<address: string, marker address>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_by_address": {
        "address": "<address: string, marker address>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_by_denom`

This function is used to query Marker by denom.

#### Fails when
1. denom is missing

#### Returns
```
Marker Struct
{
    "address": "<address: string, marker account address>",
    "allow_forced_transfer": <boolean: true or false, indicates force transfer is enabled or not>,
    "coins": [
      {
        "denom": "denom": "<token_name: string>",
        "amount": "<amount: string with unsigned 128 bit integer>"
      },
      {
        "denom": "denom": "<token_name: string>",
        "amount": "<amount: string with unsigned 128 bit integer>"
      },
      ...
    ],
    "account_number": <number: unsigned 64 bit integer>,
    "sequence": <number: unsigned 64 bit integer>,
    "manager": "<address: string>",
    "permissions": [
        {
            "permissions": [
                "<string: marker permissions like admin, burn, deposit, delete, mint, transfer or withdraw>"
            ],
            "address": "<address: string>"
        },
        {
            "permissions": [
                "<string: marker permissions like admin, burn, deposit, delete, mint, transfer or withdraw>"
            ],
            "address": "<address: string>"
        },
        ...
    ],
    "status": "<string: marker status can be either of unspecified, proposed, finalized, active, cancelled or destroyed>",
    "denom": "<token_name: string>",
    "total_supply": "<amount: string with unsigned 128 bit integer>",
    "marker_type": "<string, can be either of unspecified, coin or restricted>",
    "supply_fixed": <boolean, true or false, if fixed means supply cannot be increased further>
  }
```

#### Parameters
```
{
    "get_by_denom": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_by_denom": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_authorized_countries`

This function is used to query list of authorised country codes.

#### Returns
```
List of authorized country codes
```

#### Parameters
```
{
    "get_authorized_countries": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_authorized_countries": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_country_code_by_address`

This function is used to query whether the address is whitelisted or not. If some country code is returned then it means the particular address is whitelisted.

#### Returns
```
Country Code
```

#### Parameters
```
{
    "get_country_code_by_address": {
        "denom": "<token_name: string>",
        "address": "<address: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_country_code_by_address": {
        "denom": "<token_name: string>",
        "address": "<address: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_freezed_accounts`

This function is used to query freezed account for a particular denom.

#### Returns
```
List of freezed addresses
```

#### Parameters
```
{
    "get_freezed_accounts": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_freezed_accounts": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_frozen_balance`

This function is used to query frozen balance of an address for a particular denom.

#### Returns
```
Frozen Balance
```

#### Parameters
```
{
    "get_frozen_balance": {
        "denom": "<token_name: string>",
        "address": "<address: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_frozen_balance": {
        "denom": "<token_name: string>",
        "address": "<address: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_denom_config`

This function is used to query token limit and authorized country codes for a particular denom.

#### Returns
```
Denom Config
{
    "token_limit": "<token_limit: string with unsigned 128 bit integer>",
    "country_codes": [
        <list of unsigned 8 bit integer>
    ]
}
```

#### Parameters
```
{
    "get_denom_config": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_denom_config": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_balance`

This function is used to query balance of an address for a particular denom.

#### Returns
```
Balance
```

#### Parameters
```
{
    "get_balance": {
        "denom": "<token_name: string>",
        "address": "<address: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_balance": {
        "denom": "<token_name: string>",
        "address": "<address: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_frozen_tokens`

This function is used to query total number of frozen tokens.

#### Returns
```
Total Frozen Tokens
```

#### Parameters
```
{
    "get_frozen_tokens": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_frozen_tokens": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_ciculating_supply`

This function is used to query circulating supply.

#### Returns
```
Circulating Supply
```

#### Parameters
```
{
    "get_ciculating_supply": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_ciculating_supply": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

## Deployment Details

### contract_id
```
686
```

### contract_address
```
tp16h50hcp3m777t68vv42x6kzdrym9dyn5ucxq6tpj46qnnye0k97slzkku3
```
