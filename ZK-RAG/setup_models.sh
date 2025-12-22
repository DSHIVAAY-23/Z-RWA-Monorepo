#!/bin/bash
set -e

MODEL_DIR="models/all-MiniLM-L6-v2"
mkdir -p "$MODEL_DIR"

BASE_URL="https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main"

echo "Downloading model files to $MODEL_DIR..."

if [ ! -f "$MODEL_DIR/config.json" ]; then
    wget -q --show-progress "$BASE_URL/config.json" -O "$MODEL_DIR/config.json"
fi

if [ ! -f "$MODEL_DIR/tokenizer.json" ]; then
    wget -q --show-progress "$BASE_URL/tokenizer.json" -O "$MODEL_DIR/tokenizer.json"
fi

if [ ! -f "$MODEL_DIR/model.safetensors" ]; then
    wget -q --show-progress "$BASE_URL/model.safetensors" -O "$MODEL_DIR/model.safetensors"
fi

if [ ! -f "$MODEL_DIR/vocab.txt" ]; then
    wget -q --show-progress "$BASE_URL/vocab.txt" -O "$MODEL_DIR/vocab.txt"
fi

echo "Model download complete."
