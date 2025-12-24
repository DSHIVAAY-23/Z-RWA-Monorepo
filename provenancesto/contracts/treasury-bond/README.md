# Capital Call's Fund Contract in Provenance

This contracts is fully based on capital call functionalities in Provenance based on its fund contract.

---

## Introduction

This README file aims to guide the concept of capital call on Provenance Blockchain through its fund contract, and how to integrate them into project.

## Functions

### Function `instantiate`

This function is use to initialize the smart contract with new address. In this function the caller is set as Admin. Only this caller can call `migrate` function for smart contract upgradation.

#### Events
```
provwasm.contracts.fund.init
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
provwasm.contracts.fund.migrate
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

This function is use to create new fund contract for a particular token. In this function call
1. the creator is assigned as agent
2. the contract address will be assigned mint, burn, transfer and force_transfer accesses, in token contract

### Notes
Before calling the `create` function, the fund contract address must be assigned as sub_admin in token contract, as sub_admin in token contract have all the mint, burn, transfer and force_transfer accesses. 

#### Events
```
1. provwasm.contracts.fund.create
2. denom
3. fund_name
4. asset_type
5. issuer_name
6. target_aum
7. nav_launch_price
8. ccy
```

#### Parameters
```
{
    "create": {
        "params": {
            "denom": "<token_name: string>",
            "fund_name": "<fund_name: string>",
            "asset_type": "<asset_type: string, its an enum, can be either of token, stable_coin or fiat>",
            "issuer_name": "<issuer_name: string>",
            "target_aum": "<target_aum: string with unsigned 128 bit integer>",
            "nav_launch_price": "<nav_launch_price: string with unsigned 128 bit integer>",
            "ccy": "<address: string>"
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
            "denom": "<token_name: string>",
            "fund_name": "<fund_name: string>",
            "asset_type": "<asset_type: string, its an enum, can be either of token, stable_coin or fiat>",
            "issuer_name": "<issuer_name: string>",
            "target_aum": "<target_aum: string with unsigned 128 bit integer>",
            "nav_launch_price": "<nav_launch_price: string with unsigned 128 bit integer>",
            "ccy": "<address: string>"
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

### Function `manage_admins`

This function is use to manage admins. For addition of new addresses as admin, `update_type` will be `UpdateType::Add(Addresses)` and for removal of old addresses as admin, `update_type` will be `UpdateType::Remove(Addresses)`. This function supports batch operations, i.e. multiple addresses can be added / removed simultaneously.

#### Fails when
1. caller is not admin
2. address is already having admin rights during addition
3. address doesn't have admin rights during removal

#### Events
```
case 1. During addition, i.e., update_type = UpdateType::Add(Addresses)
    then
        provwasm.contracts.fund.add_admin
case 2. During removal, i.e., update_type = UpdateType::Remove(Addresses)
    then
        provwasm.contracts.fund.remove_admin
```

#### Parameters
For addition of new admins
```
{
    "manage_admins": {
        "update_type": {
            "add": [
                "<address: string>",
                "<address: string>",
                ...
            ]
        }
    }
}
```

For removal of old admins
```
{
    "manage_admins": {
        "update_type": {
            "remove": [
                "<address: string>",
                "<address: string>",
                ...
            ]
        }
    }
}
```

#### Script
For addition of new admins
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_admins": {
        "update_type": {
            "add": [
                "<address: string>",
                "<address: string>",
                ...
            ]
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

For removal of old admins
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_admins": {
        "update_type": {
            "remove": [
                "<address: string>",
                "<address: string>",
                ...
            ]
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

### Function `manage_agent`

This function is use to manage agent. For addition of new address as agent, `update_type` will be `UpdateType::Add(Address)` and for removal of old address as admin, `update_type` will be `UpdateType::Remove(Address)`.

#### Fails when
1. caller is not admin
2. address is already having agent rights during addition
3. address doesn't have agent rights during removal

#### Events
```
case 1. During addition, i.e., update_type = UpdateType::Add(Address)
    then
        provwasm.contracts.fund.add_agent
case 2. During removal, i.e., update_type = UpdateType::Remove(Address)
    then
        provwasm.contracts.fund.remove_agent
```

#### Parameters
For addition of new agent
```
{
    "manage_agent": {
        "update_type": {
            "add": "<address: string>"
        }
    }
}
```

For removal of old agent
```
{
    "manage_agent": {
        "update_type": {
            "remove": "<address: string>"
        }
    }
}
```

#### Script
For addition of new agent
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_agent": {
        "update_type": {
            "add": "<address: string>"
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

For removal of old agent
```
provenanced tx wasm execute <contract_address> \
	'{
    "manage_agent": {
        "update_type": {
            "remove": "<address: string>"
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

### Function `management_fees`

This function is use to manage management fees. For addition of new users, `update_type` will be `UpdateType::Add(Addresses)`, for updation of old users, `update_type` will be `UpdateType::Update(Addresses)` and for removal of old users, `update_type` will be `UpdateType::Remove(Addresses)`. This function supports batch operations, i.e. multiple addresses can be added / removed simultaneously.

#### Fails when
1. caller is not admin
2. address is already having agent rights during addition
3. address doesn't have agent rights during removal

#### Events
```
case 1. During addition, i.e., update_type = UpdateType::Add(Addresses)
    then
        provwasm.contracts.fund.add_management_fees
case 2. During update, i.e., update_type = UpdateType::Update(Addresses)
    then
        provwasm.contracts.fund.update_management_fees
case 3. During removal, i.e., update_type = UpdateType::Remove(Addresses)
    then
        provwasm.contracts.fund.remove_management_fees
```

#### Parameters
For addition of new users
```
{
    "management_fees": {
        "denom": "<token_name: string>",
        "managed_users": [
            {
                "add": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            {
                "add": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            ...
        ]
    }
}
```

For updation of existing users
```
{
    "management_fees": {
        "denom": "<token_name: string>",
        "managed_users": [
            {
                "update": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            {
                "update": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            ...
        ]
    }
}
```

For removal of old users
```
{
    "management_fees": {
        "denom": "<token_name: string>",
        "managed_users": [
            {
                "remove": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            {
                "remove": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            ...
        ]
    }
}
```

#### Script
For addition of new users
```
provenanced tx wasm execute <contract_address> \
	'{
    "management_fees": {
        "denom": "<token_name: string>",
        "managed_users": [
            {
                "add": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            {
                "add": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
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

For updation of existing users
```
provenanced tx wasm execute <contract_address> \
	'{
    "management_fees": {
        "denom": "<token_name: string>",
        "managed_users": [
            {
                "update": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            {
                "update": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
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

For removal of old users
```
provenanced tx wasm execute <contract_address> \
	'{
    "management_fees": {
        "denom": "<token_name: string>",
        "managed_users": [
            {
                "remove": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
                }
            },
            {
                "remove": {
                    "user": "<address: string>",
                    "fees": "<fees: string with unsigned 128 bit integer>"
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

### Function `share_dividend`

This function is used to share dividend to an address. For dividend share in token, `asset_type` will be `token`, for dividend share in stable coins, `asset_type` will be `stable_coin` and for dividend share in fiat, `asset_type` will be `fiat`. This function supports batch operations, i.e. multiple dividends can be shared simultaneously.

### Notes
Dividend stored during this function call is not used anywhere at the moment, that will be used in future verisons. Logic for Fiat is not implemented yet, which is subjected to be added on future versions.

#### Fails when
1. caller is not agent
2. agent doesn't have the tokens for the token transactions
3. agent doesn't have the stable coins for the stable coins transactions

#### Events
```
case 1. When asset_type = token
    then
        provwasm.contracts.fund.share_dividend.token
case 2. When asset_type = stable_coin
    then
        provwasm.contracts.fund.share_dividend.stable_coin
case 3. When asset_type = fiat
    then
        provwasm.contracts.fund.share_dividend.fiat
```

#### Parameters
```
{
    "share_dividend": {
        "denom": "<token_name: string>",
        "coin_type": "<coin_type: string, its an enum, can be either of usdt, usdc or dai>",
        "shared_dividends": [
            {
                "to": "<address: string>",
                "dividend": "<dividend: string with unsigned 128 bit integer>",
                "asset_type": "<asset_type: string, its an enum, can be either of token, stable_coin or fiat>"
            },
            {
                "to": "<address: string>",
                "dividend": "<dividend: string with unsigned 128 bit integer>",
                "asset_type": "<asset_type: string, its an enum, can be either of token, stable_coin or fiat>"
            },
            ...
        ]
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "share_dividend": {
        "denom": "<token_name: string>",
        "coin_type": "<coin_type: string, its an enum, can be either of usdt, usdc or dai>",
        "shared_dividends": [
            {
                "to": "<address: string>",
                "dividend": "<dividend: string with unsigned 128 bit integer>",
                "asset_type": "<asset_type: string, its an enum, can be either of token, stable_coin or fiat>"
            },
            {
                "to": "<address: string>",
                "dividend": "<dividend: string with unsigned 128 bit integer>",
                "asset_type": "<asset_type: string, its an enum, can be either of token, stable_coin or fiat>"
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

### Function `distribute_and_burn`

This function is used to exchange stable coins with the a particular token holding by the investor. The stable coins are transferred from agent account to the investor's account and tokens are burnt from the investor's accounts. This function supports batch operations, i.e. multiple distributions can happen simultaneously.

#### Fails when
1. caller is not agent
2. investor doesn't have the tokens
3. agent doesn't have the stable coins

#### Events
```
provwasm.contracts.fund.distribute_and_burn
```

#### Parameters
```
{
    "distribute_and_burn": {
        "denom": "<token_name: string>",
        "coin_type": "<coin_type: string, its an enum, can be either of usdt, usdc or dai>",
        "distributions": [
            {
                "investor": "<address: string>",
                "amount": "<amount: string with unsigned 128 bit integer, amount of stable coins to be transferred from agent's account>",
                "token": "<dividend: string with unsigned 128 bit integer, amount of tokens to be burnt from investor's account>"
            },
            {
                "investor": "<address: string>",
                "amount": "<amount: string with unsigned 128 bit integer, amount of stable coins to be transferred from agent's account>",
                "token": "<dividend: string with unsigned 128 bit integer, amount of tokens to be burnt from investor's account>"
            },
            ...
        ]
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "distribute_and_burn": {
        "denom": "<token_name: string>",
        "coin_type": "<coin_type: string, its an enum, can be either of usdt, usdc or dai>",
        "distributions": [
            {
                "investor": "<address: string>",
                "amount": "<amount: string with unsigned 128 bit integer, amount of stable coins to be transferred from agent's account>",
                "token": "<dividend: string with unsigned 128 bit integer, amount of tokens to be burnt from investor's account>"
            },
            {
                "investor": "<address: string>",
                "amount": "<amount: string with unsigned 128 bit integer, amount of stable coins to be transferred from agent's account>",
                "token": "<dividend: string with unsigned 128 bit integer, amount of tokens to be burnt from investor's account>"
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

### Function `rescue_token`

This function is used to return any extra token from the contract address to the user account.

#### Fails when
1. caller is not agent
2. contract doesn't have the tokens

#### Events
```
provwasm.contracts.fund.rescue_token
```

#### Parameters
```
{
    "rescue_token": {
        "denom": "<token_name: string>",
        "to": "<address: string, recipient's address>",
        "amount": "<amount: string with unsigned 128 bit integer, amount of tokens to be rescued>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "rescue_token": {
        "denom": "<token_name: string>",
        "to": "<address: string, recipient's address>",
        "amount": "<amount: string with unsigned 128 bit integer, amount of tokens to be rescued>"
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

### Function `fetch_price`

This function is used to fetch current price from onchain data.

### Notes
This function is not used anywhere at the moment, that will be used in future verisons.

#### Fails when
1. caller is not admin

#### Events
```
provwasm.contracts.fund.fetch_price
```

#### Parameters
```
{
    "fetch_price": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "fetch_price": {
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

### Function `update_currency`

This function is used to update currency field.

### Notes
This function is not used anywhere at the moment, that will be used in future verisons.

#### Fails when
1. caller is not admin

#### Events
```
provwasm.contracts.fund.update_ccy
```

#### Parameters
```
{
    "update_currency": {
        "denom": "<token_name: string>",
        "ccy": "<currency: string>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "update_currency": {
        "denom": "<token_name: string>",
        "ccy": "<currency: string>"
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

### Function `send_stable_coins`

This function is used to transfer stable coins to agent's account.

#### Events
```
1. provwasm.contracts.fund.send_stable_coins
2. to
3. amount
```

#### Parameters
```
{
    "send_stable_coins": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm execute <contract_address> \
	'{
    "send_stable_coins": {
        "denom": "<token_name: string>"
    }
}' \
    --from <caller> \
    --amount <amount of stable coins> \
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

### Function `get_admins`

This function is used to query list of admins.

#### Fails when
1. admin list is not initialized

#### Returns
```
List of Admins
```

#### Parameters
```
{
    "get_admins": {}
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_admins": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_management_fees`

This function is used to query management fees for particular denom and user address combination.

#### Returns
```
management fees
```

#### Parameters
```
{
    "get_management_fees": {
        "denom": "<token_name: string>",
        "user": "<address: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_management_fees": {
        "denom": "<token_name: string>",
        "user": "<address: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_nav`

This function is used to query Net Asset Value (NAV) for a particular denom.

#### Returns
```
Net Asset Value (NAV)
```

#### Parameters
```
{
    "get_nav": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_nav": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_aum`

This function is used to query Asset Under Management (AUM) for a particular denom.

#### Returns
```
Asset Under Management (AUM)
```

#### Parameters
```
{
    "get_aum": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_aum": {
        "denom": "<token_name: string>"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
```

### Function `get_agent_by_denom`

This function is used to query agent assigned to a paricular denom.

#### Fails when
1. agent is missing

#### Returns
```
Agent Address
```

#### Parameters
```
{
    "get_agent_by_denom": {
        "denom": "<token_name: string>"
    }
}
```

#### Script
```
provenanced tx wasm contract-state smart <contract_address> \
	'{
    "get_agent_by_denom": {
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
663
```

### contract_address

#### Dev
```
tp190s9ns0jvgr423jznaxcca332kze964dqpf64rr47vxrx2lag04sney6sx
```

#### UAT
```
tp1htqxkv59fm2mpm7uefjpf68dk8swceyte6l79yue9hz5q5maqylqvaw3pt
```