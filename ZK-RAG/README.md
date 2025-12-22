# Private Context Awareness (ZK-RAG)

A privacy-first RAG system that ingests, secures, and verifies personal data locally. It allows users to prove the relevance of their private documents to a query using Zero-Knowledge proofs, without revealing the document content.

## 🔄 System Flow

The system operates in three main stages: Ingestion, Retrieval, and Verification.

```mermaid
graph LR
    subgraph "1. Local Ingestion"
        Docs[Documents] -- Parse & Chunk --> Chunks
        Chunks -- Candle Embedding --> Vectors
        Vectors -- Store --> DB[LanceDB]
    end

    subgraph "2. Retrieval"
        Query -- Embed --> QVec[Query Vector]
        QVec -- Search --> DB
        DB -- Return --> Result[Best Chunk]
    end

    subgraph "3. ZK Verification"
        Result --> Circuit[ZK Circuit / Mock Host]
        QVec --> Circuit
        Threshold --> Circuit
        Circuit -- Verify Logic --> Proof[Relevance Proof]
    end
```

## 🚀 Quick Start

### Prerequisites
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Models**: Ensure `all-MiniLM-L6-v2` is in `./models/`.

### 1. Ingest Data
Parse a text file, chunk it, and store embeddings locally.
```bash
# From workspace root
cargo run -p private-context-ingestion -- ingest private-context/example.txt
```
*Stores vectors in `./data/lancedb`.*

### 2. Search Context
Retrieve the most relevant chunk for a query.
```bash
cargo run -p private-context-ingestion -- search "Privacy"
```

### 3. Generate Proof (ZK-RAG)
Verify that you hold a relevant document for a query without revealing it.
```bash
# Runs in Mock Mode (Host CPU) if SP1 toolchain is missing
cargo run -p private-context-ingestion -- prove "Privacy" --threshold 0.4
```
**Output:**
```
✅ Proof Valid: Chunk is relevant.
```

## 📚 Documentation
- **[Usage Guide](file:///home/user/.gemini/antigravity/brain/af33fbe7-61ca-45cb-8a9b-18551079f5a7/usage_guide.md)**: Detailed command reference.
- **[Technical Architecture](file:///home/user/.gemini/antigravity/brain/af33fbe7-61ca-45cb-8a9b-18551079f5a7/technical_architecture.md)**: Deep dive into components and design.
- **[Changelog](file:///home/user/.gemini/antigravity/brain/af33fbe7-61ca-45cb-8a9b-18551079f5a7/CHANGELOG.md)**: Release history.
