# NodeSpace Core Types - Release Process & Emergency Rollback

## Coordinated Release Process

### Overview
Since nodespace-core-types is the foundational dependency for 7+ repositories, all releases must be coordinated to prevent system-wide disruption.

### Release Types

#### 1. Patch Release (x.y.Z)
**Example**: 2.0.0 ’ 2.0.1
**Changes**: Bug fixes, documentation updates
**Coordination**: Low - can be released independently
**Timeline**: 1-2 days

#### 2. Minor Release (x.Y.z)  
**Example**: 2.0.0 ’ 2.1.0
**Changes**: New features, backward-compatible additions
**Coordination**: Medium - notify dependent repositories
**Timeline**: 1-2 weeks

#### 3. Major Release (X.y.z)
**Example**: 2.x.x ’ 3.0.0
**Changes**: Breaking changes, API changes
**Coordination**: High - full system coordination required
**Timeline**: 4-8 weeks

### Release Workflow

#### Phase 1: Preparation
1. **Version Planning**
   - Determine semantic version increment
   - Document breaking changes
   - Create migration guides
   - Update VERSIONING.md

2. **Feature Flag Review**
   - Assess which preview features are ready for stable
   - Plan deprecation timeline for old features
   - Update feature flag documentation

3. **Cross-Repository Impact Assessment**
   ```bash
   # Check dependent repositories
   grep -r "nodespace-core-types" ../*/Cargo.toml
   
   # Assess breaking change impact
   # Create repository-specific migration plans
   ```

#### Phase 2: Testing & Validation
1. **Comprehensive Testing**
   ```bash
   # Test all feature combinations
   cargo test
   cargo test --features "v3-preview,enhanced-errors"
   cargo test --features "deprecated-v1"
   cargo test --all-features
   
   # Performance benchmarks
   cargo bench
   
   # Documentation tests
   cargo test --doc
   ```

2. **Integration Testing**
   - Test with mock dependent repositories
   - Validate API contract compliance
   - Performance regression testing

#### Phase 3: Coordinated Release
1. **Pre-Release Communication**
   - Notify all repository maintainers
   - Share migration guides
   - Set coordinated release timeline

2. **Release Execution**
   ```bash
   # Update version
   cargo update-version 2.1.0
   
   # Create release tag
   git tag v2.1.0
   git push origin v2.1.0
   
   # Publish to crates.io (if applicable)
   cargo publish
   ```

3. **Dependent Repository Updates**
   - Coordinate updates to dependent repositories
   - Validate integration testing
   - Monitor for regressions

## Emergency Rollback Strategy

### Rollback Triggers
- **Critical Bug**: Data corruption, security vulnerability
- **Performance Regression**: >20% performance degradation  
- **Integration Failure**: Breaking dependent repositories
- **Widespread Compilation Errors**: Preventing development

### Immediate Response (0-1 hour)

#### 1. Assessment
```bash
# Quickly assess scope of issue
git log --oneline -10
cargo test --all-features

# Check dependent repository status
# Review Linear issues for reports
```

#### 2. Communication
- Create high-priority Linear issue
- Notify all repository maintainers
- Document specific problems observed

#### 3. Immediate Mitigation
```bash
# Pin all repositories to last known good version
# Update Cargo.toml in all dependent repos:
nodespace-core-types = "=2.0.5"  # Pin to specific version
```

### Full Rollback (1-4 hours)

#### 1. Version Rollback
```bash
# Revert problematic commits
git revert <problematic-commit-range>

# Create hotfix version
cargo update-version 2.0.6  # Patch version with fixes

# Release rollback version
git tag v2.0.6-hotfix
git push origin v2.0.6-hotfix
```

#### 2. Coordinated Repository Updates
```bash
# Update all dependent repositories
# Use automation script if available:
./scripts/update-all-repos.sh "2.0.6-hotfix"
```

#### 3. Validation
```bash
# Test entire system with rollback
cargo test --all
# Run integration tests across repositories
# Performance validation
```

### Post-Rollback Analysis (4-24 hours)

#### 1. Root Cause Analysis
- Identify what caused the issue
- Review testing gaps
- Document lessons learned

#### 2. Fix Development
- Develop proper fix
- Enhanced testing for the issue
- Review process improvements

#### 3. Re-Release Planning
- Plan next release with fixes
- Update testing procedures
- Improve coordination process

## Automation & Tools

### Release Scripts
```bash
#!/bin/bash
# scripts/release.sh
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

echo "Releasing version $VERSION"

# Run full test suite
cargo test --all-features
cargo fmt --check
cargo clippy -- -D warnings

# Update version in Cargo.toml
sed -i "s/version = .*/version = \"$VERSION\"/" Cargo.toml

# Create release commit
git add Cargo.toml
git commit -m "Release v$VERSION"
git tag "v$VERSION"

echo "Release $VERSION ready. Push with: git push origin v$VERSION"
```

### Monitoring & Alerts
- **CI/CD Integration**: Automated testing on release
- **Performance Monitoring**: Track regression metrics
- **Dependency Scanning**: Monitor for security issues
- **Integration Health**: Cross-repository build status

### Emergency Contacts
- **Primary**: Core types team lead
- **Secondary**: System architect  
- **Escalation**: Engineering manager
- **24/7 Contact**: On-call rotation (for critical issues)

## Best Practices

### Release Checklist
- [ ] All tests pass across feature combinations
- [ ] Migration guides are complete and tested
- [ ] Breaking changes are documented
- [ ] Dependent repositories are notified
- [ ] Performance benchmarks are acceptable
- [ ] Documentation is updated
- [ ] Emergency rollback plan is ready

### Communication Protocol
1. **Advance Notice**: 2 weeks for major, 1 week for minor
2. **Migration Support**: Dedicated support during migration window
3. **Progress Tracking**: Daily updates during coordinated releases
4. **Issue Escalation**: Clear escalation path for blocking issues

### Quality Gates
- **Code Coverage**: >90% test coverage
- **Performance**: No >10% regression in benchmarks
- **Documentation**: All public APIs documented
- **Compatibility**: Backward compatibility maintained unless major version

---

**This process ensures safe, coordinated evolution of the foundational NodeSpace core types while maintaining system stability.**