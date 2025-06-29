# CLAUDE.md

🚨 **STOP - READ WORKFLOW FIRST** 🚨
Before doing ANYTHING else, you MUST read the development workflow:
1. Read: `../nodespace-system-design/docs/development-workflow.md`
2. Check Linear for current tasks
3. Then return here for implementation guidance

❌ **FORBIDDEN:** Any code analysis, planning, or implementation before reading the workflow

## Development Workflow
**ALWAYS start with README.md** - This file contains the authoritative development workflow and setup instructions for this repository.

**Then return here** for repository-specific guidance and architecture details.

## Project Overview

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Purpose

This is `nodespace-core-types`, the **foundational types repository** for the NodeSpace system. It provides essential data structures that all other repositories import as Cargo dependencies.

**Key responsibility**: Define core types (`Node`, `NodeId`, `NodeSpaceResult`, `NodeSpaceError`) that enable type-safe communication across all NodeSpace services.

## 🎯 FINDING YOUR NEXT TASK

**See [development-workflow.md](../nodespace-system-design/docs/development-workflow.md)** for task management workflow.

## Getting Started

**New to NodeSpace? Start Here:**
1. **Read [NodeSpace System Design](../nodespace-system-design/README.md)** - Understand the full architecture
2. **Review [Development Workflow](../nodespace-system-design/docs/development-workflow.md)** - Process and procedures
3. **See distributed service interfaces** - Services own their interface traits and import types from this repository
4. **See [MVP User Flow](../nodespace-system-design/examples/mvp-user-flow.md)** - What you're building

## Development Setup

This repository is used as a dependency in other NodeSpace repositories:

```bash
# Add to Cargo.toml in dependent projects
[dependencies]
nodespace-core-types = { git = "https://github.com/malibio/nodespace-core-types" }

# Use in code
use nodespace_core_types::{Node, NodeId, NodeSpaceResult};
```

## Development Commands

Since this is a Rust crate for type definitions, the primary commands are:

```bash
# Validate all contracts compile
cargo check

# Run type validation tests
cargo test

# Test that downstream repos can use these types
cargo test --all-features
```

## Architecture Context

This repository sits at the foundation of the NodeSpace system architecture:

1. **`nodespace-core-types`** ← This repository (foundational types)
2. `nodespace-data-store` - Database and vector storage
3. `nodespace-nlp-engine` - AI/ML processing and LLM integration  
4. `nodespace-workflow-engine` - Automation and event processing
5. `nodespace-core-logic` - Business logic orchestration
6. `nodespace-core-ui` - React components and UI
7. `nodespace-desktop-app` - Tauri application shell

**Critical constraint**: This repository has **no dependencies** on other NodeSpace repositories. All other repositories depend on this one via Cargo. Changes here affect all downstream repositories.

## Core Components

- **`Node`**: Core entity type with serde_json::Value content for flexibility
- **`NodeId`**: Unique identifier type for all entities across the system
- **`NodeSpaceResult<T>`**: Standard Result type for consistent error handling
- **`NodeSpaceError`**: Comprehensive error type covering all failure modes
- **Supporting types**: Additional structures needed for cross-service communication

## Development Approach

**Dependency-Based Architecture**: This repository defines minimal essential types that other repositories import via Cargo dependencies. 

**Distributed contract ownership** - Each service repository owns its interface traits and imports types from this repository.

**Focus**: Define clean, minimal types that serve as the foundation for all other repositories.

## Project Management

All tasks are tracked in the [Linear workspace](https://linear.app/nodespace). Filter by `nodespace-core-types` to see relevant issues.

For new contributors: Start by reading the [NodeSpace System Design](../nodespace-system-design/README.md) to understand the full architecture before making changes.