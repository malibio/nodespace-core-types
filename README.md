# NodeSpace Core Types

**Foundational Rust types and data structures for the NodeSpace distributed system**

This repository contains the essential data structures that all other NodeSpace repositories depend on as a Cargo dependency. It provides core types like `Node`, `NodeId`, and `NodeSpaceResult` that enable type-safe communication across the distributed system.

## What is NodeSpace?

NodeSpace is a distributed knowledge management system that combines hierarchical note-taking with AI-powered search and automation. It's built as a collection of microservices that work together to provide a seamless user experience through a desktop application.

## Repository Purpose

This repository serves as the **foundational type system** for the entire NodeSpace architecture. It defines:

- **Core data structures** that represent knowledge entities
- **Error handling types** for consistent error propagation
- **Result types** for standardized return values
- **Utility types** for cross-service communication

## Key Types

### `Node`
The fundamental data structure representing any piece of knowledge in NodeSpace:
- Hierarchical organization with parent-child relationships
- Flexible JSON content storage
- Metadata for AI/ML processing
- Temporal tracking (created/updated timestamps)
- Performance-optimized root references

### `NodeId`
Unique identifier type for all entities across the system:
- UUID-based for global uniqueness
- Serializable for network transport
- Type-safe to prevent ID confusion

### `NodeSpaceResult<T>`
Standard Result type for all operations:
- Consistent error handling across services
- Rich error context with service attribution
- Proper error propagation patterns

### `NodeSpaceError`
Comprehensive error type covering all failure modes:
- Database errors (connection, query, transaction)
- Validation errors (schema, business rules)
- Network errors (timeouts, connectivity)
- Processing errors (AI/ML model failures)
- Service errors (orchestration, coordination)

## AI/ML Integration

This repository includes specialized types for AI/ML capabilities:

### Multi-Level Embeddings
- **Individual embeddings** - Basic content embeddings
- **Contextual embeddings** - Enhanced with relationship context
- **Hierarchical embeddings** - Full path context from root
- **Performance metrics** - Tracking for optimization

### Image AI Support
- **ImageNode** - Specialized node type for images
- **ImageMetadata** - AI-generated image analysis
- **Multimodal embeddings** - Combined text and image vectors

### Context Management
- **ContextStrategy** - Rule-based and AI-enhanced context generation
- **NodeContext** - Relationship-aware context building
- **Performance tracking** - Metrics for AI operations

## Architecture Role

This repository sits at the foundation of the NodeSpace system:

```
┌─────────────────────────────────────────────────────────────┐
│                    NodeSpace Architecture                    │
├─────────────────────────────────────────────────────────────┤
│  Desktop App (Tauri) - User Interface                      │
│  https://github.com/malibio/nodespace-desktop-app          │
├─────────────────────────────────────────────────────────────┤
│  Core UI (React) - Frontend Components                     │
│  https://github.com/malibio/nodespace-core-ui              │
├─────────────────────────────────────────────────────────────┤
│  Core Logic - Business Orchestration                       │
│  https://github.com/malibio/nodespace-core-logic           │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Data Store     │  │  NLP Engine     │  │ Workflow Engine │ │
│  │  (Vector DB)    │  │  (AI/ML)        │  │  (Automation)   │ │
│  │ github.com/     │  │ github.com/     │  │ github.com/     │ │
│  │ malibio/        │  │ malibio/        │  │ malibio/        │ │
│  │ nodespace-      │  │ nodespace-      │  │ nodespace-      │ │
│  │ data-store      │  │ nlp-engine      │  │ workflow-engine │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│           Core Types (This Repository)                      │
│    Node, NodeId, NodeSpaceResult, NodeSpaceError           │
│    https://github.com/malibio/nodespace-core-types         │
└─────────────────────────────────────────────────────────────┘
```

## Architecture Context

Part of the NodeSpace system architecture:

1. **[nodespace-core-types](https://github.com/malibio/nodespace-core-types)** ← **You are here**
2. [nodespace-data-store](https://github.com/malibio/nodespace-data-store) - Vector database integration and data persistence
3. [nodespace-nlp-engine](https://github.com/malibio/nodespace-nlp-engine) - AI/ML processing and LLM integration  
4. [nodespace-workflow-engine](https://github.com/malibio/nodespace-workflow-engine) - Automation and event processing
5. [nodespace-core-logic](https://github.com/malibio/nodespace-core-logic) - Business logic orchestration
6. [nodespace-core-ui](https://github.com/malibio/nodespace-core-ui) - React components and UI
7. [nodespace-desktop-app](https://github.com/malibio/nodespace-desktop-app) - Tauri application shell

## Development

### Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
nodespace-core-types = { git = "https://github.com/malibio/nodespace-core-types" }
```

Use in your code:
```rust
use nodespace_core_types::{Node, NodeId, NodeSpaceResult};
```

### Testing
```bash
# Validate all contracts compile
cargo check

# Run type validation tests
cargo test

# Check linting
cargo clippy -- -D warnings
```

## Design Principles

### Zero Dependencies
This repository has **no dependencies** on other NodeSpace repositories. All other repositories depend on this one through Cargo, ensuring a clean dependency graph.

### Type Safety
All inter-service communication uses strongly-typed interfaces defined here, preventing runtime errors and ensuring API compatibility.

### Performance
Types are designed for efficiency:
- Optimized serialization/deserialization
- Memory-efficient representations
- Performance feature flags for optimization

### Extensibility
The type system supports future growth:
- Feature flags for API versioning
- Flexible JSON content in nodes
- Extensible error types
- Modular AI/ML integration

## Contributing

This repository follows standard development practices. All changes must:
1. Maintain backward compatibility
2. Pass all tests and linting
3. Update documentation
4. Follow semantic versioning principles

Please ensure all contributions maintain the foundational role of this repository and its zero-dependency design.
