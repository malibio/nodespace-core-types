# NodeSpace Core Types - Semantic Versioning Strategy

## Overview

This document describes the comprehensive semantic versioning strategy for `nodespace-core-types` v2.0.0+, designed to enable safe API evolution across 7+ dependent repositories in the NodeSpace ecosystem.

## Version Policy

### Semantic Versioning (Major.Minor.Patch)

- **Major Version (X.y.z)**: Breaking changes to core types, method signatures, or API contracts
- **Minor Version (x.Y.z)**: New features, backward-compatible additions (new methods, optional fields)
- **Patch Version (x.y.Z)**: Bug fixes, performance improvements, documentation updates

### Current Version: 2.0.0

The v2.0.0 release establishes production-ready semantic versioning with feature flag support.

## Feature Flag Architecture

### Available Feature Flags

| Feature Flag | Purpose | Status | Stability |
|--------------|---------|--------|-----------|
| `v2-api` | Current stable API (default) | ✅ Stable | Production Ready |
| `v3-preview` | Preview next major version features | 🔬 Preview | Use with caution |
| `deprecated-v1` | Legacy v1 compatibility methods | ⚠️ Deprecated | Will be removed in v3.0 |
| `enhanced-errors` | Enhanced error handling capabilities | ✅ Stable | Production Ready |
| `performance-opts` | Performance optimizations | ✅ Stable | Production Ready |
| `experimental` | Unstable experimental features | 🧪 Experimental | May break without notice |

### Feature Flag Usage

```toml
# In dependent repository Cargo.toml
[dependencies]
nodespace-core-types = { version = "2.0", features = ["enhanced-errors", "performance-opts"] }
```

```rust
// Runtime feature detection
use nodespace_core_types::features;

if features::is_v3_preview_enabled() {
    // Use v3 preview features
}
```

## API Evolution Examples

### Current v2 API (Stable)

```rust
use nodespace_core_types::{Node, NodeId, NodeSpaceResult};

// Standard node creation
let node = Node::new(serde_json::json!({"content": "stable API"}));

// Standard error handling
let result: NodeSpaceResult<String> = Ok("success".to_string());
```

### v3 Preview Features

```rust
#[cfg(feature = "v3-preview")]
use nodespace_core_types::Node;

// Typed node creation (v3 preview)
let node = Node::new_typed(data, "document")?;
println!("Node type: {:?}", node.node_type());

// Enhanced NodeId with prefixes
let prefixed_id = NodeId::with_prefix("doc");
println!("Prefix: {:?}", prefixed_id.prefix());
```

### Enhanced Error Handling

```rust
#[cfg(feature = "enhanced-errors")]
use nodespace_core_types::{NodeSpaceError, ErrorSeverity};

let error = NodeSpaceError::Database(/* ... */);
match error.severity() {
    ErrorSeverity::Critical => { /* handle critical */ },
    ErrorSeverity::Warning => { /* handle warning */ },
    _ => { /* handle others */ },
}
```

## Version Compatibility Matrix

| Version Series | Compatible Features | Status | Support Timeline |
|----------------|-------------------|--------|------------------|
| 2.x | `v2-api`, `enhanced-errors`, `performance-opts` | ✅ Active | LTS until v4.0 |
| 3.x-preview | `v3-preview` | 🔬 Preview | Development only |
| 1.x | `deprecated-v1` | ⚠️ Deprecated | Removed in v3.0 |

## Breaking Change Process

### 1. Deprecation Phase (Current Version)

```rust
#[deprecated(since = "2.1.0", note = "Use new_method() instead. Will be removed in v3.0.0")]
pub fn old_method(&self) -> String {
    // Implementation with deprecation warning
}
```

### 2. Preview Phase (Next Major Version)

```rust
#[cfg(feature = "v3-preview")]
pub fn new_method(&self) -> String {
    // New implementation available for early testing
}
```

### 3. Migration Guide

Provide clear documentation with:
- **What changed**: Detailed description of breaking changes
- **Why it changed**: Rationale for the breaking change
- **How to migrate**: Step-by-step migration instructions
- **Code examples**: Before/after code samples

### 4. Coordinated Release

All dependent repositories must be updated and tested before the breaking change release.

## Compatibility Utilities

```rust
use nodespace_core_types::compatibility;

// Check version compatibility
assert!(compatibility::is_compatible_with("2.1"));

// Get compatibility matrix
let matrix = compatibility::compatibility_matrix();
```

## Testing Strategy

### Feature Flag Testing

```bash
# Test default features (v2-api)
cargo test

# Test v3 preview features
cargo test --features "v3-preview,enhanced-errors"

# Test legacy compatibility
cargo test --features "deprecated-v1"

# Test all stable features
cargo test --features "v2-api,enhanced-errors,performance-opts"
```

### Version Compatibility Validation

The crate includes comprehensive tests that validate:
- Feature flag combinations don't conflict
- Deprecated methods still work with warnings
- New features are properly gated
- Backward compatibility is maintained

## Migration Examples

### Migrating from v1 to v2

```rust
// OLD (v1, deprecated)
#[allow(deprecated)]
let error = NodeSpaceError::database_error("connection failed");

// NEW (v2, recommended)
let error = NodeSpaceError::Database(
    DatabaseError::connection_failed("postgres", "connection failed")
);
```

### Preparing for v3

```rust
// Current v2 API
let node = Node::new(content);

// Future v3 API (available now with feature flag)
#[cfg(feature = "v3-preview")]
let typed_node = Node::new_typed(content, "document")?;
```

## Emergency Rollback Strategy

### Version Pinning

```toml
# Pin to specific version for stability
[dependencies]
nodespace-core-types = "=2.0.0"
```

### Feature Flag Rollback

```toml
# Disable problematic features
[dependencies]
nodespace-core-types = { version = "2.0", default-features = false, features = ["v2-api"] }
```

### Coordinated Rollback Process

1. **Identify Issue**: Monitor for breaking changes or regressions
2. **Pin Versions**: Update all dependent repositories to pin to last known good version
3. **Coordinate Release**: Release coordinated fix across all affected repositories
4. **Validate**: Test entire system with pinned versions

## Best Practices for Dependent Repositories

### Version Constraints

```toml
# Recommended: Allow patch and minor updates
nodespace-core-types = "2.0"

# Conservative: Pin to exact version
nodespace-core-types = "=2.0.0"

# Flexible: Allow compatible updates with feature selection
nodespace-core-types = { version = "^2.0", features = ["enhanced-errors"] }
```

### Feature Flag Usage

```rust
// Always check feature availability at runtime when needed
#[cfg(feature = "v3-preview")]
fn use_v3_features() {
    // v3-specific code
}

// Use compatibility utilities for dynamic checks
if nodespace_core_types::compatibility::is_compatible_with("2.1") {
    // Use features available in 2.1+
}
```

### Testing Integration

```rust
#[cfg(test)]
mod tests {
    use nodespace_core_types::features;
    
    #[test]
    fn test_feature_integration() {
        // Test that your code works with different feature combinations
        let active_features = features::active_features();
        println!("Active features: {:?}", active_features);
        
        // Your integration tests here
    }
}
```

## Monitoring and Alerts

### Version Compatibility Monitoring

```rust
// In application startup
let version = nodespace_core_types::CORE_TYPES_VERSION;
let features = nodespace_core_types::features::active_features();

log::info!("Core types version: {}, features: {:?}", version, features);

// Verify expected features are available
assert!(nodespace_core_types::features::is_v2_api_enabled());
```

### Breaking Change Detection

Set up CI/CD to detect potential breaking changes:
- Automated testing with different feature flag combinations
- Version compatibility validation in integration tests
- Dependency update notifications with impact assessment

## Future Roadmap

### v2.x Series (Current)
- **v2.0.x**: Bug fixes and performance improvements
- **v2.1.x**: New backward-compatible features
- **v2.2.x**: Enhanced error handling improvements

### v3.x Series (Preview)
- **v3.0-preview**: Early access to breaking changes
- **v3.0**: Major version with breaking changes, removes `deprecated-v1`
- **v3.1+**: New features in v3 series

### End-of-Life Policy
- **v1.x**: End-of-life, removed in v3.0.0
- **v2.x**: Long-term support until v4.0.0
- **Each major version**: Minimum 1 year support after next major release

---

**For questions or issues with versioning strategy, please check [Linear workspace](https://linear.app/nodespace) or create an issue in the repository.**