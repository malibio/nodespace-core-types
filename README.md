# ⚠️ BEFORE STARTING ANY WORK
👉 **STEP 1**: Read development workflow: `../nodespace-system-design/docs/development-workflow.md`
👉 **STEP 2**: Check Linear for assigned tasks
👉 **STEP 3**: Repository-specific patterns below

**This README.md only contains**: Repository-specific type definitions and Rust patterns

# NodeSpace Core Types

**Shared data structures and interfaces for NodeSpace multi-session development environment**

This repository contains the **foundational types** that all other NodeSpace repositories depend on as a Cargo dependency. It provides essential data structures like `Node`, `NodeId`, and `NodeSpaceResult` that enable type-safe communication across the distributed system.

## 🎯 Purpose

- **Essential data types** - `Node`, `NodeId`, `NodeSpaceResult`, `NodeSpaceError`
- **Foundational structures** - Core types needed across all services
- **Type safety** - Ensure consistent data structures via Cargo dependencies
- **Zero dependencies** - Pure Rust types with no external NodeSpace dependencies

## 📦 Contents

- **`Node`** - Core entity type with serde_json::Value content
- **`NodeId`** - Unique identifier type for entities
- **`NodeSpaceResult<T>`** - Standard Result type for all operations
- **`NodeSpaceError`** - Comprehensive error type with proper propagation
- **Utility types** - Supporting structures for cross-service communication

## 🔗 Dependencies

This repository has **no dependencies** on other NodeSpace repositories. All other repositories depend on this one.

## 🚀 Getting Started

### **New to NodeSpace? Start Here:**
1. **Read [NodeSpace System Design](../nodespace-system-design/README.md)** - Understand the full architecture
2. **Check [Linear workspace](https://linear.app/nodespace)** - Find your current tasks (filter by `nodespace-core-types`)
3. **Review [Development Workflow](../nodespace-system-design/docs/development-workflow.md)** - Process and procedures
4. **See distributed service interfaces** - Services own their interface traits and import types from this repository
5. **See [MVP User Flow](../nodespace-system-design/examples/mvp-user-flow.md)** - What you're building

### **Development Setup:**
```bash
# Add to your Cargo.toml
[dependencies]
nodespace-core-types = { git = "https://github.com/malibio/nodespace-core-types" }

# Use in your code
use nodespace_core_types::{Node, NodeId, NodeSpaceResult};
```

## 🏗️ Architecture Context

Part of the [NodeSpace system architecture](../nodespace-system-design/README.md):

1. **`nodespace-core-types`** ← **You are here**
2. `nodespace-data-store` - Database and vector storage
3. `nodespace-nlp-engine` - AI/ML processing and LLM integration  
4. `nodespace-workflow-engine` - Automation and event processing
5. `nodespace-core-logic` - Business logic orchestration
6. `nodespace-core-ui` - React components and UI
7. `nodespace-desktop-app` - Tauri application shell

## 🧪 Testing

```bash
# Validate all contracts compile
cargo check

# Run type validation tests  
cargo test

# Check that other repos can use these types
cargo test --all-features
```

---

**Project Management:** All development tasks tracked in [Linear workspace](https://linear.app/nodespace)