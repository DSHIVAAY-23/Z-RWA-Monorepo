#!/bin/bash
set -e

# Configuration
CHAIN_ID="mantra-local-1"
HOME_DIR="$HOME/.mantra-local"
MONIKER="local-node"

# 1. Clean previous setup
rm -rf "$HOME_DIR"

# 2. Initialize Node
echo "Initializing local MANTRA node..."
mantrachaind init "$MONIKER" --chain-id "$CHAIN_ID" --home "$HOME_DIR"

# 3. Add Keys
echo "Adding test keys..."
# Mnemonic for deterministic keys if needed, or just let it generate
mantrachaind keys add prover_admin --home "$HOME_DIR" --keyring-backend test
mantrachaind keys add user_minter --home "$HOME_DIR" --keyring-backend test

# 4. Add Genesis Account
echo "Adding genesis accounts..."
mantrachaind genesis add-genesis-account prover_admin 1000000000000uom --home "$HOME_DIR" --keyring-backend test
mantrachaind genesis add-genesis-account user_minter 1000000000uom --home "$HOME_DIR" --keyring-backend test

# 5. Genesis Transaction
echo "Creating genesis transaction..."
mantrachaind genesis gentx prover_admin 100000000uom --chain-id "$CHAIN_ID" --home "$HOME_DIR" --keyring-backend test

# 6. Collect Gentxs
mantrachaind genesis collect-gentxs --home "$HOME_DIR"

# 7. Start Node
echo "Starting mantrachaind..."
# Runs in background? The user asked to start it. 
# Usually scripts blocking is better or using screen/tmux. 
# We'll just print the command to run.
echo "Setup complete. Run the following command to start the node:"
echo "mantrachaind start --home $HOME_DIR"
