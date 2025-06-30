# Migration Guide Template for NodeSpace Core Types

## Migration Guide: [Version X.Y.Z] → [Version A.B.C]

### Overview

**Release Date**: [YYYY-MM-DD]
**Migration Difficulty**: [Low/Medium/High]
**Estimated Migration Time**: [X hours/days]
**Breaking Changes**: [Yes/No]

### Summary of Changes

[Provide a high-level summary of what changed and why]

### Breaking Changes

#### 1. [Breaking Change Title]

**What Changed**: [Detailed description of the change]
**Why**: [Rationale for the breaking change]
**Impact**: [Which functionality is affected]

**Before (Old API)**:
```rust
// Example of old code that will break
let old_result = OldAPI::old_method();
```

**After (New API)**:
```rust
// Example of new code that should be used
let new_result = NewAPI::new_method();
```

**Migration Steps**:
1. [Step-by-step instructions]
2. [Include any intermediate steps]
3. [Mention any gotchas or edge cases]

**Automated Migration**:
```bash
# If applicable, provide scripts or tools
find . -name "*.rs" -exec sed -i 's/old_pattern/new_pattern/g' {} \;
```

### New Features

#### 1. [New Feature Title]

**Description**: [What the new feature does]
**Availability**: [Feature flag requirements, if any]

**Usage Example**:
```rust
// Example of how to use the new feature
#[cfg(feature = "new-feature")]
let result = new_feature_api();
```

### Deprecations

#### 1. [Deprecated Feature Title]

**Status**: Deprecated in [version], will be removed in [future version]
**Replacement**: [What to use instead]

**Migration**:
```rust
// OLD (deprecated)
#[allow(deprecated)]
let old_way = deprecated_method();

// NEW (recommended)
let new_way = replacement_method();
```

### Performance Improvements

- [List any performance improvements]
- [Include benchmarks if significant]

### Bug Fixes

- [List important bug fixes]
- [Note if they might change behavior]

### Feature Flag Changes

#### New Flags
- `new-feature`: [Description]

#### Deprecated Flags  
- `old-feature`: Will be removed in [version]

#### Updated Flags
- `existing-feature`: [What changed]

### Testing Your Migration

#### Pre-Migration Checklist
- [ ] Review all usages of deprecated APIs
- [ ] Identify potential breaking changes in your codebase
- [ ] Plan migration strategy for each breaking change
- [ ] Set up test environment with new version

#### Migration Testing
```bash
# Test with new version
cargo update nodespace-core-types
cargo test

# Test with specific features
cargo test --features "new-feature"

# Test compatibility
cargo check --all-targets
```

#### Post-Migration Validation
- [ ] All tests pass
- [ ] No deprecation warnings (unless intentional)
- [ ] Performance is acceptable
- [ ] Integration tests pass

### Common Migration Issues

#### Issue 1: [Common Problem]
**Symptoms**: [How to recognize this issue]
**Solution**: [How to fix it]

#### Issue 2: [Another Common Problem]  
**Symptoms**: [How to recognize this issue]
**Solution**: [How to fix it]

### Rollback Plan

If you need to rollback:

```toml
# Pin to previous version
[dependencies]
nodespace-core-types = "=[previous-version]"
```

**Steps**:
1. Update Cargo.toml to pin previous version
2. Run `cargo update` to downgrade
3. Revert any API changes you made
4. Test thoroughly

### Repository-Specific Migration Guides

#### For Data Store Repository
- [Specific guidance for nodespace-data-store]

#### For NLP Engine Repository  
- [Specific guidance for nodespace-nlp-engine]

#### For Core Logic Repository
- [Specific guidance for nodespace-core-logic]

#### For UI Repository
- [Specific guidance for nodespace-core-ui]

#### For Desktop App Repository
- [Specific guidance for nodespace-desktop-app]

### Timeline and Coordination

#### Phase 1: Preparation (Week 1)
- [ ] Review migration guide
- [ ] Plan changes for your repository
- [ ] Test with preview version if available

#### Phase 2: Migration (Week 2)  
- [ ] Update dependencies
- [ ] Apply API changes
- [ ] Update tests
- [ ] Validate functionality

#### Phase 3: Validation (Week 3)
- [ ] Cross-repository integration testing
- [ ] Performance validation
- [ ] Final acceptance testing

### Support and Resources

- **Documentation**: [Link to updated documentation]
- **Example Code**: [Link to migration examples]
- **Linear Issues**: [Link to related Linear issues]
- **Discussion**: [Link to team discussion channel]

### Emergency Contacts

If you encounter blocking issues during migration:
- **Primary Contact**: [Team lead]
- **Secondary Contact**: [Backup contact]
- **Escalation**: Create high-priority Linear issue

---

## Migration Guide Example: v1.x → v2.0.0

### Overview

**Release Date**: 2025-06-30
**Migration Difficulty**: Medium
**Estimated Migration Time**: 2-4 hours
**Breaking Changes**: Yes

### Summary of Changes

NodeSpace Core Types v2.0.0 introduces semantic versioning, feature flags, and enhanced error handling while maintaining backward compatibility through the `deprecated-v1` feature flag.

### Breaking Changes

#### 1. Error Constructor Methods

**What Changed**: Legacy error constructors are now behind feature flags
**Why**: To encourage use of structured error types with better context
**Impact**: Direct usage of `NodeSpaceError::database_error()` etc. will not compile

**Before (Old API)**:
```rust
let error = NodeSpaceError::database_error("connection failed");
```

**After (New API)**:
```rust
let error = NodeSpaceError::Database(
    DatabaseError::connection_failed("postgres", "connection failed")
);
```

**Migration Steps**:
1. Enable `deprecated-v1` feature flag temporarily for compatibility
2. Update error creation to use specific error types
3. Remove `deprecated-v1` feature flag once migration is complete

**Automated Migration**:
```bash
# Enable feature flag in Cargo.toml
echo 'features = ["deprecated-v1"]' >> Cargo.toml
```

### New Features

#### 1. Feature Flag System

**Description**: Controlled API evolution with feature flags
**Availability**: All features available in v2.0.0

**Usage Example**:
```rust
// Check features at runtime
use nodespace_core_types::features;

if features::is_enhanced_errors_enabled() {
    // Use enhanced error features
}
```

#### 2. Enhanced Error Handling

**Description**: Error severity levels and enhanced context
**Availability**: Requires `enhanced-errors` feature flag

**Usage Example**:
```rust
#[cfg(feature = "enhanced-errors")]
let severity = error.severity();
```

### Deprecations

#### 1. Legacy Error Constructors

**Status**: Deprecated in v2.0.0, will be removed in v3.0.0
**Replacement**: Use specific error type constructors

**Migration**:
```rust
// OLD (deprecated, requires deprecated-v1 feature)
#[allow(deprecated)]
let error = NodeSpaceError::database_error("msg");

// NEW (recommended)
let error = NodeSpaceError::Database(
    DatabaseError::connection_failed("db", "msg")
);
```

### Feature Flag Changes

#### New Flags
- `v2-api`: Current stable API (enabled by default)
- `v3-preview`: Preview of v3 features
- `enhanced-errors`: Enhanced error handling capabilities
- `performance-opts`: Performance optimizations
- `deprecated-v1`: Legacy v1 compatibility

### Testing Your Migration

#### Pre-Migration Checklist
- [ ] Identify all usages of `NodeSpaceError::database_error()`
- [ ] Identify all usages of `NodeSpaceError::not_found()`
- [ ] Identify all usages of `NodeSpaceError::validation_error()`
- [ ] Plan to replace with specific error types

#### Migration Testing
```bash
# Test with deprecated-v1 flag first
cargo test --features "deprecated-v1"

# Then test with new API
cargo test

# Test specific features
cargo test --features "enhanced-errors"
```

### Common Migration Issues

#### Issue 1: Compilation Errors on Legacy Constructors
**Symptoms**: `no variant or associated item named 'database_error' found`
**Solution**: Enable `deprecated-v1` feature flag temporarily or migrate to new API

#### Issue 2: Missing Enhanced Error Methods
**Symptoms**: `no method named 'severity' found`
**Solution**: Enable `enhanced-errors` feature flag

---

**This template should be customized for each specific version migration.**