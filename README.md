# NodeSpace Core Types

**Shared data structures and interfaces for NodeSpace multi-session development environment**

This repository contains the foundational type definitions and trait interfaces that all other NodeSpace repositories depend on. It serves as the **single source of truth** for data structures and contracts across the distributed system.

## 🎯 Purpose

- **Shared data structures** - Node types, metadata, search queries, embeddings
- **Interface contracts** - Traits that services must implement 
- **Error types** - Standardized error handling across repositories
- **Validation framework** - Contract compliance testing utilities

## 📦 Contents

- **Core data types** - `Node`, `EmbeddingVector`, `SearchQuery`, `RAGQuery`
- **Service interfaces** - `DataStore`, `NLPEngine`, `WorkflowEngine` traits
- **Tauri commands** - Complete interface for React ↔ Rust communication
- **Error handling** - Comprehensive error types with proper propagation

## 🔗 Dependencies

This repository has **no dependencies** on other NodeSpace repositories. All other repositories depend on this one.

## 🏗️ Architecture Context

Part of the [NodeSpace system architecture](https://github.com/malibio/nodespace-system-design):

1. **`nodespace-core-types`** ← **You are here**
2. `nodespace-data-store` - Database and vector storage
3. `nodespace-nlp-engine` - AI/ML processing and LLM integration  
4. `nodespace-workflow-engine` - Automation and event processing
5. `nodespace-core-logic` - Business logic orchestration
6. `nodespace-core-ui` - React components and UI
7. `nodespace-desktop-app` - Tauri application shell

## 🚀 Getting Started

```bash
# Add to your Cargo.toml
[dependencies]
nodespace-core-types = { git = "https://github.com/malibio/nodespace-core-types" }

# Use in your code
use nodespace_core_types::{Node, DataStore, SearchQuery};
```

## 🧪 Testing

```bash
# Validate all contracts compile
cargo check

# Run contract compliance tests  
cargo test

# Validate interface compatibility
cargo run --bin validate-contracts
```

## 📋 Development Status

- [ ] Copy contracts from system design repo
- [ ] Set up Cargo workspace
- [ ] Implement validation utilities
- [ ] Add comprehensive tests
- [ ] Documentation and examples

---

**Project Management:** All tasks tracked in [NodeSpace Project](https://github.com/users/malibio/projects/4)