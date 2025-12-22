# User Guide: Private Context Awareness CLI

This guide provides instructions on how to use the `private-context` CLI for local document ingestion, semantic search, and Zero-Knowledge relevance proving.

## Prerequisites
- **Rust Toolchain**: Ensure `cargo` is installed.
- **Models**: The embedding model (`all-MiniLM-L6-v2`) must be present in `./models/`.
- **Data Directory**: The CLI will automatically create a `data/` directory for storage.

## 1. Local Ingestion
Ingest text documents into your private local store. The system generates embeddings and indexes them using HNSW.

### Basic Ingestion
```bash
cargo run -p private-context-ingestion -- ingest <path-to-file>
```
**Example:**
```bash
cargo run -p private-context-ingestion -- ingest test.txt
```

### Ingestion with Provenance (zkTLS)
To attach cryptographic proof of origin (e.g., from Reclaim Protocol) to your data:
```bash
cargo run -p private-context-ingestion -- ingest <path-to-file> --proof <path-to-proof.json>
```
**Example:**
```bash
cargo run -p private-context-ingestion -- ingest test.txt --proof valid_proof.json
```
*Note: Depending on the validity of the proof, the ingestion will either succeed (✅) or abort (❌).*

## 2. Semantic Search
Search your local private knowledge base.
```bash
cargo run -p private-context-ingestion -- search "<query>"
```
**Example:**
```bash
cargo run -p private-context-ingestion -- search "Rust storage"
```

## 3. ZK-RAG Verification
Generate a Zero-Knowledge proof (or simulate one) that shows you hold a document relevant to a query, without revealing the document.

### Command
```bash
cargo run -p private-context-ingestion -- prove "<query>" --threshold <value>
```

### Examples
**Verify Relevance (Success Case):**
Prove that you have a document with similarity >= 0.3.
```bash
cargo run -p private-context-ingestion -- prove "Rust storage" --threshold 0.3
```

**Verify Relevance (Failure Case):**
Attempt to prove relevance with a strict threshold (0.7).
```bash
cargo run -p private-context-ingestion -- prove "Rust storage" --threshold 0.7
```

### Note on Mock Mode
If the SP1 Guest ELF binary is not found (which is typical in dev environments without the full SP1 toolchain), the CLI operates in **Mock Mode**.
- It runs the **identical fixed-point math logic** on the host CPU.
- It outputs `✅ Host check passed!` or `❌ Host check failed!` instead of a cryptographic proof.
- This ensures the logical correctness of the circuit before compilation.
