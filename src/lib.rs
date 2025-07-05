use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;
use uuid::Uuid;

// ========================================
// Semantic Versioning and Feature Management
// ========================================

/// Semantic version information for this crate
pub const CORE_TYPES_VERSION: &str = env!("CARGO_PKG_VERSION");

/// API version constants for feature flag coordination
pub mod version {
    /// Current stable API version (2.x series)
    pub const V2_API: &str = "2.0";

    /// Preview API version (3.x series)
    pub const V3_PREVIEW: &str = "3.0-preview";

    /// Legacy API version (1.x series, deprecated)
    #[deprecated(since = "2.0.0", note = "v1 API is deprecated, please migrate to v2")]
    pub const V1_LEGACY: &str = "1.0-legacy";
}

/// Feature flag utilities for version management
pub mod features {
    /// Check if v2 API is enabled (default)
    pub fn is_v2_api_enabled() -> bool {
        cfg!(feature = "v2-api")
    }

    /// Check if v3 preview features are enabled
    pub fn is_v3_preview_enabled() -> bool {
        cfg!(feature = "v3-preview")
    }

    /// Check if deprecated v1 features are enabled
    #[deprecated(since = "2.0.0", note = "v1 features will be removed in v3.0.0")]
    pub fn is_v1_legacy_enabled() -> bool {
        cfg!(feature = "deprecated-v1")
    }

    /// Check if enhanced error handling is enabled
    pub fn is_enhanced_errors_enabled() -> bool {
        cfg!(feature = "enhanced-errors")
    }

    /// Check if performance optimizations are enabled
    pub fn is_performance_opts_enabled() -> bool {
        cfg!(feature = "performance-opts")
    }

    /// Get currently active feature flags as a string
    pub fn active_features() -> Vec<&'static str> {
        let mut features = Vec::new();

        if is_v2_api_enabled() {
            features.push("v2-api");
        }
        if is_v3_preview_enabled() {
            features.push("v3-preview");
        }
        #[allow(deprecated)]
        if is_v1_legacy_enabled() {
            features.push("deprecated-v1");
        }
        if is_enhanced_errors_enabled() {
            features.push("enhanced-errors");
        }
        if is_performance_opts_enabled() {
            features.push("performance-opts");
        }
        if cfg!(feature = "experimental") {
            features.push("experimental");
        }

        features
    }
}

/// Version compatibility utilities
pub mod compatibility {
    /// Check if the current version is compatible with a required version
    pub fn is_compatible_with(required_version: &str) -> bool {
        match required_version {
            v if v.starts_with("2.") => cfg!(feature = "v2-api"),
            v if v.starts_with("3.") => cfg!(feature = "v3-preview"),
            #[allow(deprecated)]
            v if v.starts_with("1.") => cfg!(feature = "deprecated-v1"),
            _ => false,
        }
    }

    /// Get version compatibility matrix
    pub fn compatibility_matrix() -> std::collections::HashMap<&'static str, Vec<&'static str>> {
        let mut matrix = std::collections::HashMap::new();
        matrix.insert("v2-api", vec!["2.0", "2.1", "2.x"]);
        matrix.insert("v3-preview", vec!["3.0-preview", "3.0-alpha"]);

        #[allow(deprecated)]
        matrix.insert("deprecated-v1", vec!["1.0", "1.1", "1.x"]);

        matrix
    }
}

// NodeId - database-agnostic unique identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    /// Create a new random NodeId
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a NodeId from an existing string (for deserialization)
    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    /// Get the ID as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the ID as a string (legacy method name)
    #[deprecated(since = "2.0.0", note = "Use `as_str()` instead for consistency")]
    #[cfg(feature = "deprecated-v1")]
    pub fn to_string_legacy(&self) -> String {
        self.0.clone()
    }

    /// Create a NodeId with custom prefix (v3 preview feature)
    #[cfg(feature = "v3-preview")]
    pub fn with_prefix(prefix: &str) -> Self {
        Self(format!("{}:{}", prefix, Uuid::new_v4()))
    }

    /// Extract prefix from NodeId (v3 preview feature)
    #[cfg(feature = "v3-preview")]
    pub fn prefix(&self) -> Option<&str> {
        if self.0.contains(':') {
            self.0.split(':').next()
        } else {
            None
        }
    }

    /// Performance-optimized NodeId creation (when performance-opts enabled)
    #[cfg(feature = "performance-opts")]
    pub fn new_fast() -> Self {
        // In real implementation, this might use a faster UUID generation
        // For now, it's the same as new() but demonstrates the concept
        Self::new()
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NodeId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for NodeId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// Core node structure for cross-service communication
///
/// This structure represents the fundamental data unit in NodeSpace, supporting hierarchical
/// organization with performance optimizations for efficient querying.
///
/// ## Root Hierarchy Optimization
///
/// The `root_id` and `root_type` fields enable efficient hierarchy queries by denormalizing
/// the root relationship. Instead of multiple O(N) database scans to traverse up the hierarchy,
/// nodes can be queried directly by their root with O(1) indexed lookups.
///
/// ### Performance Benefits
///
/// * **Before**: O(N × depth) multiple database queries to build hierarchy
/// * **After**: O(1) single indexed query + O(M) memory operations  
/// * **Expected improvement**: 10x-100x performance for hierarchical operations
///
/// ### Usage Pattern
///
/// ```rust
/// use nodespace_core_types::{Node, NodeId};
/// use serde_json::json;
///
/// // Create a text node
/// let node = Node::new("text".to_string(), json!({"content": "Hello world"}));
///
/// // Create a parent-child relationship
/// let child = Node::new("text".to_string(), json!({"content": "Child node"}))
///     .with_parent(Some(node.id.clone()));
///
/// // Create sibling relationships
/// let sibling = Node::new("text".to_string(), json!({"content": "Sibling node"}))
///     .with_before_sibling(Some(child.id.clone()));
/// ```
///
/// ### Data Pattern
///
/// ```text
/// // Root node points to itself
/// Node {
///     id: "date:2025-06-30",
///     root_id: Some("date:2025-06-30"),  // Points to itself
///     root_type: Some("date"),
///     parent_id: None,  // True root
///     // ... other fields
/// }
///
/// // Child nodes all point to the same root
/// Node {
///     id: "text:meeting-notes",
///     root_id: Some("date:2025-06-30"),  // Same as root!
///     root_type: Some("date"),
///     parent_id: Some("date:2025-06-30"),
///     // ... other fields
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub r#type: String, // Required by LanceDB: "text", "date", "task", etc.
    pub content: serde_json::Value, // Flexible content
    pub metadata: Option<serde_json::Value>, // Optional system metadata
    pub created_at: String, // ISO format timestamp
    pub updated_at: String, // ISO format timestamp
    // Hierarchical relationship
    pub parent_id: Option<NodeId>, // → Parent node (None = root)
    // Sibling navigation (bidirectional)
    pub before_sibling: Option<NodeId>, // → Previous node in sequence (None = first)
    pub next_sibling: Option<NodeId>,   // → Next node in sequence (None = last)
    /// Root hierarchy optimization for efficient queries
    ///
    /// Points to the hierarchy root node, enabling O(1) indexed queries instead of
    /// multiple O(N) scans. For root nodes, this points to the node itself.
    pub root_id: Option<NodeId>,
}

impl Node {
    /// Create a new Node with generated ID, type, and content
    pub fn new(r#type: String, content: serde_json::Value) -> Self {
        #[cfg(feature = "performance-opts")]
        let id = NodeId::new_fast();
        #[cfg(not(feature = "performance-opts"))]
        let id = NodeId::new();

        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id,
            r#type,
            content,
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
            parent_id: None,
            before_sibling: None,
            next_sibling: None,
            root_id: None,
        }
    }

    /// Create a Node with existing ID, type, and content
    pub fn with_id(id: NodeId, r#type: String, content: serde_json::Value) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id,
            r#type,
            content,
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
            parent_id: None,
            before_sibling: None,
            next_sibling: None,
            root_id: None,
        }
    }

    /// Set metadata for the node
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Update the node's timestamp
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Set the next sibling pointer
    pub fn with_next_sibling(mut self, next_sibling: Option<NodeId>) -> Self {
        self.next_sibling = next_sibling;
        self
    }

    /// Set the previous sibling pointer
    pub fn with_before_sibling(mut self, before_sibling: Option<NodeId>) -> Self {
        self.before_sibling = before_sibling;
        self
    }

    /// Set the parent ID
    pub fn with_parent(mut self, parent_id: Option<NodeId>) -> Self {
        self.parent_id = parent_id;
        self
    }

    /// Update sibling pointers
    pub fn set_next_sibling(&mut self, next_sibling: Option<NodeId>) {
        self.next_sibling = next_sibling;
        self.touch();
    }

    /// Update previous sibling pointer
    pub fn set_before_sibling(&mut self, before_sibling: Option<NodeId>) {
        self.before_sibling = before_sibling;
        self.touch();
    }

    /// Update parent ID
    pub fn set_parent_id(&mut self, parent_id: Option<NodeId>) {
        self.parent_id = parent_id;
        self.touch();
    }

    /// Check if this node has a parent
    pub fn has_parent(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Check if this node is a root node (no parent)
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Check if this node has a next sibling
    pub fn has_next_sibling(&self) -> bool {
        self.next_sibling.is_some()
    }

    /// Check if this node has a previous sibling
    pub fn has_before_sibling(&self) -> bool {
        self.before_sibling.is_some()
    }

    /// Check if this node is last in sequence (no next sibling)
    pub fn is_last(&self) -> bool {
        self.next_sibling.is_none()
    }

    /// Check if this node is first in sequence (no previous sibling)
    pub fn is_first_sibling(&self) -> bool {
        self.before_sibling.is_none()
    }

    /// Create a new date node with proper schema-based structure
    pub fn new_date_node(date: chrono::NaiveDate) -> Self {
        let date_metadata = DateNodeMetadata::new(date);
        let content = serde_json::json!({
            "type": "date",
            "content": date_metadata.display_format.clone(),
            "date_metadata": date_metadata
        });

        Self::new("date".to_string(), content)
    }

    /// Create a date node with timezone context
    pub fn new_date_node_with_timezone(date: chrono::NaiveDate, timezone: &str) -> Self {
        let date_metadata = DateNodeMetadata::with_timezone(date, timezone);
        let content = serde_json::json!({
            "type": "date",
            "content": date_metadata.display_format.clone(),
            "date_metadata": date_metadata
        });

        Self::new("date".to_string(), content)
    }

    /// Check if this node is a date node by examining its structure
    pub fn is_date_node(&self) -> bool {
        if let Some(type_field) = self.content.get("type") {
            type_field.as_str() == Some("date")
        } else {
            false
        }
    }

    /// Extract date metadata from a date node
    pub fn get_date_metadata(&self) -> Option<DateNodeMetadata> {
        if self.is_date_node() {
            self.content
                .get("date_metadata")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
        } else {
            None
        }
    }

    /// Get the date from a date node
    pub fn get_date(&self) -> Option<chrono::NaiveDate> {
        self.get_date_metadata()
            .and_then(|metadata| metadata.parse_date().ok())
    }

    /// Create a node with typed content (v3 preview feature)
    #[cfg(feature = "v3-preview")]
    pub fn new_typed<T: serde::Serialize>(content: T, node_type: &str) -> NodeSpaceResult<Self> {
        let content_value =
            serde_json::to_value(content).map_err(|e| ProcessingError::SerializationFailed {
                format: "JSON".to_string(),
                reason: e.to_string(),
                data_type: std::any::type_name::<T>().to_string(),
                fallback_formats: vec!["MessagePack".to_string()],
            })?;

        let mut node = Self::new(content_value);
        if let Some(metadata) = node.metadata.as_mut() {
            if let Some(map) = metadata.as_object_mut() {
                map.insert(
                    "node_type".to_string(),
                    serde_json::Value::String(node_type.to_string()),
                );
            }
        } else {
            node.metadata = Some(serde_json::json!({
                "node_type": node_type
            }));
        }
        Ok(node)
    }

    /// Get node type from metadata (v3 preview feature)
    #[cfg(feature = "v3-preview")]
    pub fn node_type(&self) -> Option<String> {
        self.metadata
            .as_ref()
            .and_then(|m| m.get("node_type"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
    }

    /// Legacy method for backward compatibility
    #[deprecated(since = "2.0.0", note = "Use `with_metadata()` instead")]
    #[cfg(feature = "deprecated-v1")]
    pub fn set_metadata_legacy(&mut self, metadata: serde_json::Value) {
        self.metadata = Some(metadata);
        self.touch();
    }

    // Root hierarchy optimization methods

    /// Update root hierarchy information
    ///
    /// This method allows updating the root optimization fields after node creation,
    /// automatically updating the node's timestamp.
    ///
    /// # Arguments
    /// * `root_id` - Optional NodeId of the hierarchy root (None to clear)
    pub fn set_root(&mut self, root_id: Option<NodeId>) {
        self.root_id = root_id;
        self.touch();
    }

    /// Check if this node belongs to a hierarchy root
    ///
    /// Returns true if the node has root optimization information configured,
    /// enabling efficient hierarchy queries.
    pub fn has_root(&self) -> bool {
        self.root_id.is_some()
    }

    /// Check if this node is itself a hierarchy root
    ///
    /// Returns true if this node is configured as a hierarchy root (root_id points
    /// to itself and has no parent), making it the top of an optimized hierarchy tree.
    pub fn is_hierarchy_root(&self) -> bool {
        matches!((&self.root_id, &self.parent_id), (Some(root_id), None) if root_id == &self.id)
    }
}

// Relationship reference for graph model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRef {
    pub target_id: NodeId,
    pub relationship_type: String,
    pub properties: serde_json::Value,
}

impl RelationshipRef {
    /// Create a new relationship reference
    pub fn new(target_id: NodeId, relationship_type: String) -> Self {
        Self {
            target_id,
            relationship_type,
            properties: serde_json::Value::Null,
        }
    }

    /// Add properties to the relationship
    pub fn with_properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = properties;
        self
    }
}

// Database-specific errors with structured context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed to {database}: {reason}")]
    ConnectionFailed {
        database: String,
        reason: String,
        retry_after: Option<Duration>,
    },

    #[error("Query timeout after {seconds}s: {query}")]
    QueryTimeout {
        seconds: u64,
        query: String,
        suggested_limit: Option<usize>,
    },

    #[error("Record not found: {entity_type} with id {id}")]
    NotFound {
        entity_type: String,
        id: String,
        suggestions: Vec<String>,
    },

    #[error("Constraint violation: {constraint} on {table}")]
    ConstraintViolation {
        constraint: String,
        table: String,
        conflicting_value: String,
    },

    #[error("Migration failed: {version} -> {target_version}: {reason}")]
    MigrationFailed {
        version: String,
        target_version: String,
        reason: String,
        rollback_available: bool,
    },

    #[error("Transaction failed: {operation}")]
    TransactionFailed {
        operation: String,
        reason: String,
        can_retry: bool,
    },

    #[error("Index corruption detected: {index_name}")]
    IndexCorruption {
        index_name: String,
        table: String,
        repair_command: Option<String>,
    },
}

// Validation errors with field-specific context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ValidationError {
    #[error("Required field missing: {field} in {context}")]
    RequiredFieldMissing {
        field: String,
        context: String,
        suggestion: Option<String>,
    },

    #[error("Invalid format for {field}: expected {expected}, got {actual}")]
    InvalidFormat {
        field: String,
        expected: String,
        actual: String,
        examples: Vec<String>,
    },

    #[error("Value out of range for {field}: {value} not in [{min}, {max}]")]
    OutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },

    #[error("Invalid relationship: {source_type} cannot reference {target_type}")]
    InvalidRelationship {
        source_type: String,
        target_type: String,
        allowed_types: Vec<String>,
    },

    #[error("Schema validation failed: {schema_path}")]
    SchemaValidationFailed {
        schema_path: String,
        violations: Vec<String>,
        schema_version: String,
    },

    #[error("Business rule violation: {rule}")]
    BusinessRuleViolation {
        rule: String,
        context: serde_json::Value,
        resolution_steps: Vec<String>,
    },
}

// Network errors with retry and recovery guidance
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum NetworkError {
    #[error("Connection timeout to {endpoint} after {timeout_ms}ms")]
    ConnectionTimeout {
        endpoint: String,
        timeout_ms: u64,
        retry_after: Option<Duration>,
        max_retries: u32,
    },

    #[error("DNS resolution failed for {hostname}")]
    DnsResolutionFailed {
        hostname: String,
        dns_servers: Vec<String>,
        fallback_endpoints: Vec<String>,
    },

    #[error("HTTP error {status_code}: {reason}")]
    HttpError {
        status_code: u16,
        reason: String,
        endpoint: String,
        headers: std::collections::HashMap<String, String>,
        retryable: bool,
    },

    #[error("TLS/SSL error: {reason}")]
    TlsError {
        reason: String,
        certificate_info: Option<String>,
        suggested_action: String,
    },

    #[error("Rate limit exceeded: {limit} requests per {window}")]
    RateLimitExceeded {
        limit: u32,
        window: String,
        reset_time: DateTime<Utc>,
        retry_after: Duration,
    },

    #[error("Network unreachable: {network}")]
    NetworkUnreachable {
        network: String,
        interface: Option<String>,
        routing_table: Vec<String>,
    },
}

// Processing errors with service attribution
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ProcessingError {
    #[error("AI model error in {service}: {model_name} - {reason}")]
    ModelError {
        service: String,
        model_name: String,
        reason: String,
        model_version: Option<String>,
        fallback_available: bool,
    },

    #[error("Embedding generation failed: {reason}")]
    EmbeddingFailed {
        reason: String,
        input_type: String,
        dimensions: Option<usize>,
        model_info: Option<String>,
    },

    #[error("Vector search failed: {reason}")]
    VectorSearchFailed {
        reason: String,
        index_name: String,
        query_dimensions: usize,
        similarity_threshold: Option<f32>,
    },

    #[error("Workflow execution failed: {workflow_id} at step {step}")]
    WorkflowFailed {
        workflow_id: String,
        step: String,
        reason: String,
        can_resume: bool,
        checkpoint_available: bool,
    },

    #[error("Resource exhausted: {resource_type}")]
    ResourceExhausted {
        resource_type: String,
        current_usage: String,
        limit: String,
        suggested_action: String,
    },

    #[error("Serialization failed: {format} - {reason}")]
    SerializationFailed {
        format: String,
        reason: String,
        data_type: String,
        fallback_formats: Vec<String>,
    },
}

// Cross-service errors for distributed debugging
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ServiceError {
    #[error("Service unavailable: {service_name}")]
    ServiceUnavailable {
        service_name: String,
        endpoint: String,
        health_check_url: Option<String>,
        estimated_recovery: Option<Duration>,
    },

    #[error("Service version mismatch: {service} expected {expected}, got {actual}")]
    VersionMismatch {
        service: String,
        expected: String,
        actual: String,
        compatibility_matrix: Vec<String>,
    },

    #[error("Configuration error in {service}: {config_key}")]
    ConfigurationError {
        service: String,
        config_key: String,
        expected_type: String,
        current_value: Option<String>,
        valid_values: Vec<String>,
    },

    #[error("Circuit breaker open for {service}: {failure_count} failures")]
    CircuitBreakerOpen {
        service: String,
        failure_count: u32,
        failure_threshold: u32,
        reset_time: DateTime<Utc>,
    },

    #[error("Authentication failed for service {service}: {reason}")]
    AuthenticationFailed {
        service: String,
        reason: String,
        auth_type: String,
        renewal_required: bool,
    },

    #[error("Service capacity exceeded: {service}")]
    CapacityExceeded {
        service: String,
        current_load: f32,
        max_capacity: f32,
        queue_length: Option<u32>,
    },
}

/// Error severity levels for enhanced error handling
#[cfg(feature = "enhanced-errors")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Informational message, no action required
    Info,
    /// Warning that should be noted but doesn't prevent operation
    Warning,
    /// Error that prevents the current operation from completing
    Error,
    /// Critical error that affects system stability
    Critical,
}

#[cfg(feature = "enhanced-errors")]
impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARN"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

// Top-level hierarchical error system
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum NodeSpaceError {
    #[error("Database operation failed: {0}")]
    Database(#[from] DatabaseError),

    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),

    #[error("Network operation failed: {0}")]
    Network(#[from] NetworkError),

    #[error("Processing failed: {0}")]
    Processing(#[from] ProcessingError),

    #[error("Service error: {0}")]
    Service(#[from] ServiceError),

    // Legacy compatibility variants (will be deprecated)
    #[error("IO error: {message}")]
    IoError { message: String },

    #[error("Internal error: {message} (service: {service})")]
    InternalError { message: String, service: String },
}

// Standard Result type for all NodeSpace operations
pub type NodeSpaceResult<T> = Result<T, NodeSpaceError>;

// Convenience constructors for common error scenarios
impl DatabaseError {
    pub fn connection_failed(database: &str, reason: &str) -> Self {
        Self::ConnectionFailed {
            database: database.to_string(),
            reason: reason.to_string(),
            retry_after: Some(Duration::from_secs(5)),
        }
    }

    pub fn not_found(entity_type: &str, id: &str) -> Self {
        Self::NotFound {
            entity_type: entity_type.to_string(),
            id: id.to_string(),
            suggestions: vec![],
        }
    }

    pub fn query_timeout(query: &str, seconds: u64) -> Self {
        Self::QueryTimeout {
            seconds,
            query: query.to_string(),
            suggested_limit: Some(1000),
        }
    }
}

impl ValidationError {
    pub fn required_field(field: &str, context: &str) -> Self {
        Self::RequiredFieldMissing {
            field: field.to_string(),
            context: context.to_string(),
            suggestion: Some(format!("Please provide a value for '{}'", field)),
        }
    }

    pub fn invalid_format(field: &str, expected: &str, actual: &str) -> Self {
        Self::InvalidFormat {
            field: field.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
            examples: vec![],
        }
    }

    pub fn out_of_range(field: &str, value: &str, min: &str, max: &str) -> Self {
        Self::OutOfRange {
            field: field.to_string(),
            value: value.to_string(),
            min: min.to_string(),
            max: max.to_string(),
        }
    }
}

impl NetworkError {
    pub fn connection_timeout(endpoint: &str, timeout_ms: u64) -> Self {
        Self::ConnectionTimeout {
            endpoint: endpoint.to_string(),
            timeout_ms,
            retry_after: Some(Duration::from_secs(1)),
            max_retries: 3,
        }
    }

    pub fn http_error(status_code: u16, reason: &str, endpoint: &str) -> Self {
        Self::HttpError {
            status_code,
            reason: reason.to_string(),
            endpoint: endpoint.to_string(),
            headers: std::collections::HashMap::new(),
            retryable: matches!(status_code, 500..=599),
        }
    }

    pub fn rate_limit_exceeded(limit: u32, window: &str, reset_time: DateTime<Utc>) -> Self {
        Self::RateLimitExceeded {
            limit,
            window: window.to_string(),
            reset_time,
            retry_after: Duration::from_secs(60),
        }
    }
}

impl ProcessingError {
    pub fn model_error(service: &str, model_name: &str, reason: &str) -> Self {
        Self::ModelError {
            service: service.to_string(),
            model_name: model_name.to_string(),
            reason: reason.to_string(),
            model_version: None,
            fallback_available: false,
        }
    }

    pub fn embedding_failed(reason: &str, input_type: &str) -> Self {
        Self::EmbeddingFailed {
            reason: reason.to_string(),
            input_type: input_type.to_string(),
            dimensions: Some(384),
            model_info: None,
        }
    }

    pub fn vector_search_failed(reason: &str, index_name: &str, query_dimensions: usize) -> Self {
        Self::VectorSearchFailed {
            reason: reason.to_string(),
            index_name: index_name.to_string(),
            query_dimensions,
            similarity_threshold: Some(0.7),
        }
    }
}

impl ServiceError {
    pub fn service_unavailable(service_name: &str, endpoint: &str) -> Self {
        Self::ServiceUnavailable {
            service_name: service_name.to_string(),
            endpoint: endpoint.to_string(),
            health_check_url: None,
            estimated_recovery: Some(Duration::from_secs(30)),
        }
    }

    pub fn version_mismatch(service: &str, expected: &str, actual: &str) -> Self {
        Self::VersionMismatch {
            service: service.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
            compatibility_matrix: vec![],
        }
    }

    pub fn configuration_error(service: &str, config_key: &str, expected_type: &str) -> Self {
        Self::ConfigurationError {
            service: service.to_string(),
            config_key: config_key.to_string(),
            expected_type: expected_type.to_string(),
            current_value: None,
            valid_values: vec![],
        }
    }
}

impl NodeSpaceError {
    // Legacy compatibility constructors (deprecated, require v1 feature flag)
    #[deprecated(
        since = "2.0.0",
        note = "Use DatabaseError::connection_failed or other specific error constructors instead. This legacy method will be removed in v3.0.0"
    )]
    #[cfg(feature = "deprecated-v1")]
    pub fn database_error(msg: &str) -> Self {
        Self::Database(DatabaseError::connection_failed("unknown", msg))
    }

    #[deprecated(
        since = "2.0.0",
        note = "Use DatabaseError::not_found instead. This legacy method will be removed in v3.0.0"
    )]
    #[cfg(feature = "deprecated-v1")]
    pub fn not_found(msg: &str) -> Self {
        Self::Database(DatabaseError::not_found("unknown", msg))
    }

    #[deprecated(
        since = "2.0.0",
        note = "Use ValidationError::required_field instead. This legacy method will be removed in v3.0.0"
    )]
    #[cfg(feature = "deprecated-v1")]
    pub fn validation_error(msg: &str) -> Self {
        Self::Validation(ValidationError::required_field("unknown", msg))
    }

    /// Enhanced error context (v3 preview feature)
    #[cfg(feature = "v3-preview")]
    pub fn with_context(self, _context: std::collections::HashMap<String, String>) -> Self {
        // In a real implementation, this would add context to the error
        // For now, it demonstrates the concept
        self
    }

    /// Get error severity level (enhanced errors feature)
    #[cfg(feature = "enhanced-errors")]
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Database(DatabaseError::ConnectionFailed { .. }) => ErrorSeverity::Critical,
            Self::Database(DatabaseError::QueryTimeout { .. }) => ErrorSeverity::Warning,
            Self::Validation(_) => ErrorSeverity::Error,
            Self::Network(NetworkError::ConnectionTimeout { .. }) => ErrorSeverity::Warning,
            Self::Network(NetworkError::HttpError { status_code, .. }) if *status_code >= 500 => {
                ErrorSeverity::Error
            }
            Self::Processing(ProcessingError::ModelError { .. }) => ErrorSeverity::Error,
            Self::Service(ServiceError::ServiceUnavailable { .. }) => ErrorSeverity::Critical,
            _ => ErrorSeverity::Info,
        }
    }

    // Utility methods for error analysis
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Database(DatabaseError::TransactionFailed { can_retry, .. }) => *can_retry,
            Self::Network(NetworkError::HttpError { retryable, .. }) => *retryable,
            Self::Network(NetworkError::ConnectionTimeout { .. }) => true,
            Self::Network(NetworkError::RateLimitExceeded { .. }) => true,
            Self::Processing(ProcessingError::ModelError {
                fallback_available, ..
            }) => *fallback_available,
            Self::Service(ServiceError::ServiceUnavailable { .. }) => true,
            Self::Service(ServiceError::CircuitBreakerOpen { .. }) => false,
            _ => false,
        }
    }

    pub fn service_attribution(&self) -> Option<String> {
        match self {
            Self::Processing(ProcessingError::ModelError { service, .. }) => Some(service.clone()),
            Self::Service(ServiceError::ServiceUnavailable { service_name, .. }) => {
                Some(service_name.clone())
            }
            Self::Service(ServiceError::VersionMismatch { service, .. }) => Some(service.clone()),
            Self::Service(ServiceError::ConfigurationError { service, .. }) => {
                Some(service.clone())
            }
            Self::Service(ServiceError::CircuitBreakerOpen { service, .. }) => {
                Some(service.clone())
            }
            Self::Service(ServiceError::AuthenticationFailed { service, .. }) => {
                Some(service.clone())
            }
            Self::Service(ServiceError::CapacityExceeded { service, .. }) => Some(service.clone()),
            Self::InternalError { service, .. } => Some(service.clone()),
            _ => None,
        }
    }

    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Database(DatabaseError::ConnectionFailed { retry_after, .. }) => *retry_after,
            Self::Network(NetworkError::ConnectionTimeout { retry_after, .. }) => *retry_after,
            Self::Network(NetworkError::RateLimitExceeded { retry_after, .. }) => {
                Some(*retry_after)
            }
            Self::Service(ServiceError::ServiceUnavailable {
                estimated_recovery, ..
            }) => *estimated_recovery,
            _ => None,
        }
    }

    pub fn error_category(&self) -> &'static str {
        match self {
            Self::Database(_) => "database",
            Self::Validation(_) => "validation",
            Self::Network(_) => "network",
            Self::Processing(_) => "processing",
            Self::Service(_) => "service",
            Self::IoError { .. } => "io",
            Self::InternalError { .. } => "internal",
        }
    }
}

// Additional utility types for common patterns

/// Common metadata structure for flexible use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub created_at: String, // ISO format timestamp
    pub updated_at: String, // ISO format timestamp
    pub version: u64,
    pub tags: Vec<String>,
}

impl Default for NodeMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            created_at: now.clone(),
            updated_at: now,
            version: 1,
            tags: Vec::new(),
        }
    }
}

impl NodeMetadata {
    /// Create new metadata with current timestamp
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the timestamp and increment version
    pub fn update(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
        self.version += 1;
    }
}

// Node type classification enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NodeType {
    #[default]
    Text,
    Image,
    Task,
    Document,
    Link,
    Entity,
    Date,
    Audio,
    Video,
    Custom(String),
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Text => write!(f, "text"),
            NodeType::Image => write!(f, "image"),
            NodeType::Task => write!(f, "task"),
            NodeType::Document => write!(f, "document"),
            NodeType::Link => write!(f, "link"),
            NodeType::Entity => write!(f, "entity"),
            NodeType::Date => write!(f, "date"),
            NodeType::Audio => write!(f, "audio"),
            NodeType::Video => write!(f, "video"),
            NodeType::Custom(name) => write!(f, "{}", name),
        }
    }
}

// Date-specific metadata structure for schema-based date operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateNodeMetadata {
    pub date: String,                // ISO format YYYY-MM-DD
    pub timezone: String,            // UTC offset or timezone name
    pub display_format: String,      // Localized display format (e.g., "June 30, 2025")
    pub created_by_navigation: bool, // True if auto-created by date navigation
    pub locale: Option<String>,      // Language/locale for formatting (e.g., "en-US")
}

impl Default for DateNodeMetadata {
    fn default() -> Self {
        Self {
            date: chrono::Utc::now()
                .date_naive()
                .format("%Y-%m-%d")
                .to_string(),
            timezone: "UTC".to_string(),
            display_format: "".to_string(),
            created_by_navigation: false,
            locale: None,
        }
    }
}

impl DateNodeMetadata {
    /// Create DateNodeMetadata for a specific date
    pub fn new(date: chrono::NaiveDate) -> Self {
        Self {
            date: date.format("%Y-%m-%d").to_string(),
            timezone: "UTC".to_string(),
            display_format: date.format("%B %-d, %Y").to_string(),
            created_by_navigation: true,
            locale: Some("en-US".to_string()),
        }
    }

    /// Create DateNodeMetadata with timezone context
    pub fn with_timezone(date: chrono::NaiveDate, timezone: &str) -> Self {
        Self {
            date: date.format("%Y-%m-%d").to_string(),
            timezone: timezone.to_string(),
            display_format: date.format("%B %-d, %Y").to_string(),
            created_by_navigation: true,
            locale: Some("en-US".to_string()),
        }
    }

    /// Parse the stored date back to NaiveDate
    pub fn parse_date(&self) -> Result<chrono::NaiveDate, chrono::ParseError> {
        chrono::NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")
    }
}

// Camera information from EXIF data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CameraInfo {
    pub make: Option<String>,
    pub model: Option<String>,
    pub software: Option<String>,
    pub lens_model: Option<String>,
    pub focal_length: Option<f32>,     // in mm
    pub aperture: Option<f32>,         // f-stop value
    pub shutter_speed: Option<String>, // e.g., "1/60"
    pub iso: Option<u32>,
    pub flash: Option<bool>,
    pub white_balance: Option<String>,
    pub orientation: Option<u32>, // EXIF orientation value 1-8
}

// Image metadata extraction results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageMetadata {
    pub ai_description: Option<String>,
    pub detected_objects: Vec<String>,
    pub scene_classification: Option<String>,
    pub keywords: Vec<String>,
    pub color_palette: Vec<String>,   // dominant colors as hex codes
    pub text_content: Option<String>, // OCR extracted text
    pub faces_detected: Option<u32>,  // number of faces
    pub emotions: Vec<String>,        // detected emotions
    pub confidence_scores: std::collections::HashMap<String, f32>, // AI confidence for various detections
}

// Comprehensive ImageNode structure for multimodal RAG
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageNode {
    // Core identification
    pub id: NodeId,
    pub node_type: NodeType, // Always NodeType::Image
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // Image data and metadata
    pub raw_data: Vec<u8>,
    pub embedding: Vec<f32>, // 384 dimensions for multimodal embeddings
    pub filename: String,
    pub content_type: String, // MIME type (image/jpeg, image/png, etc.)
    pub file_size: usize,

    // Image properties
    pub dimensions: (u32, u32),              // (width, height)
    pub timestamp: Option<DateTime<Utc>>,    // from EXIF or file metadata
    pub gps_coordinates: Option<(f64, f64)>, // (latitude, longitude)

    // EXIF and camera metadata
    pub camera_info: Option<CameraInfo>,

    // AI-generated metadata
    pub ai_metadata: ImageMetadata,

    // NodeSpace integration
    pub relationships: Vec<NodeId>, // references to related nodes
    pub parent_id: Option<NodeId>,  // parent node if this is part of a document

    // User-provided metadata
    pub user_description: Option<String>,
    pub user_tags: Vec<String>,

    // Sibling navigation (bidirectional)
    pub before_sibling: Option<NodeId>, // → Previous image in sequence (None = first)
    pub next_sibling: Option<NodeId>,   // → Next image in sequence (None = last)

    // Root hierarchy optimization for efficient queries
    pub root_id: Option<NodeId>, // → Points to hierarchy root (enables O(1) queries)
}

impl ImageNode {
    /// Create a new ImageNode with minimal required data
    pub fn new(
        raw_data: Vec<u8>,
        filename: String,
        content_type: String,
        dimensions: (u32, u32),
    ) -> Self {
        let now = Utc::now();
        Self {
            id: NodeId::new(),
            node_type: NodeType::Image,
            created_at: now,
            updated_at: now,
            raw_data,
            embedding: Vec::new(), // Will be populated by NLP engine
            filename,
            content_type,
            file_size: 0, // Will be calculated
            dimensions,
            timestamp: None,
            gps_coordinates: None,
            camera_info: None,
            ai_metadata: ImageMetadata::default(),
            relationships: Vec::new(),
            parent_id: None,
            user_description: None,
            user_tags: Vec::new(),
            before_sibling: None,
            next_sibling: None,
            root_id: None,
        }
    }

    /// Create an ImageNode with existing ID
    pub fn with_id(
        id: NodeId,
        raw_data: Vec<u8>,
        filename: String,
        content_type: String,
        dimensions: (u32, u32),
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            node_type: NodeType::Image,
            created_at: now,
            updated_at: now,
            raw_data,
            embedding: Vec::new(),
            filename,
            content_type,
            file_size: 0,
            dimensions,
            timestamp: None,
            gps_coordinates: None,
            camera_info: None,
            ai_metadata: ImageMetadata::default(),
            relationships: Vec::new(),
            parent_id: None,
            user_description: None,
            user_tags: Vec::new(),
            before_sibling: None,
            next_sibling: None,
            root_id: None,
        }
    }

    /// Set the file size (typically calculated from raw_data.len())
    pub fn with_file_size(mut self, file_size: usize) -> Self {
        self.file_size = file_size;
        self
    }

    /// Add embedding data
    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = embedding;
        self.touch();
        self
    }

    /// Set camera information from EXIF data
    pub fn with_camera_info(mut self, camera_info: CameraInfo) -> Self {
        self.camera_info = Some(camera_info);
        self.touch();
        self
    }

    /// Set GPS coordinates
    pub fn with_gps_coordinates(mut self, latitude: f64, longitude: f64) -> Self {
        self.gps_coordinates = Some((latitude, longitude));
        self.touch();
        self
    }

    /// Set timestamp from EXIF or file metadata
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self.touch();
        self
    }

    /// Set AI-generated metadata
    pub fn with_ai_metadata(mut self, ai_metadata: ImageMetadata) -> Self {
        self.ai_metadata = ai_metadata;
        self.touch();
        self
    }

    /// Add user description
    pub fn with_user_description(mut self, description: String) -> Self {
        self.user_description = Some(description);
        self.touch();
        self
    }

    /// Add user tags
    pub fn with_user_tags(mut self, tags: Vec<String>) -> Self {
        self.user_tags = tags;
        self.touch();
        self
    }

    /// Set parent node
    pub fn with_parent(mut self, parent_id: NodeId) -> Self {
        self.parent_id = Some(parent_id);
        self.touch();
        self
    }

    /// Add relationship to another node
    pub fn add_relationship(&mut self, node_id: NodeId) {
        if !self.relationships.contains(&node_id) {
            self.relationships.push(node_id);
            self.touch();
        }
    }

    /// Remove relationship
    pub fn remove_relationship(&mut self, node_id: &NodeId) {
        if let Some(pos) = self.relationships.iter().position(|id| id == node_id) {
            self.relationships.remove(pos);
            self.touch();
        }
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Set next sibling pointer
    pub fn with_next_sibling(mut self, next: Option<NodeId>) -> Self {
        self.next_sibling = next;
        self.touch();
        self
    }

    /// Set previous sibling pointer
    pub fn with_before_sibling(mut self, before: Option<NodeId>) -> Self {
        self.before_sibling = before;
        self.touch();
        self
    }

    /// Set next sibling
    pub fn set_next_sibling(&mut self, next_sibling: Option<NodeId>) {
        self.next_sibling = next_sibling;
        self.touch();
    }

    /// Set previous sibling
    pub fn set_before_sibling(&mut self, before_sibling: Option<NodeId>) {
        self.before_sibling = before_sibling;
        self.touch();
    }

    /// Check if this node has a next sibling
    pub fn has_next_sibling(&self) -> bool {
        self.next_sibling.is_some()
    }

    /// Check if this node has a previous sibling
    pub fn has_before_sibling(&self) -> bool {
        self.before_sibling.is_some()
    }

    /// Check if this node is last in sequence
    pub fn is_last(&self) -> bool {
        self.next_sibling.is_none()
    }

    /// Check if this node is first in sequence
    pub fn is_first_sibling(&self) -> bool {
        self.before_sibling.is_none()
    }

    /// Validate ImageNode data integrity
    pub fn validate(&self) -> NodeSpaceResult<()> {
        // Validate required fields
        if self.filename.is_empty() {
            return Err(ValidationError::required_field("filename", "ImageNode").into());
        }

        if self.content_type.is_empty() {
            return Err(ValidationError::required_field("content_type", "ImageNode").into());
        }

        // Validate content type is image
        if !self.content_type.starts_with("image/") {
            return Err(ValidationError::invalid_format(
                "content_type",
                "image/*",
                &self.content_type,
            )
            .into());
        }

        // Validate dimensions
        if self.dimensions.0 == 0 || self.dimensions.1 == 0 {
            return Err(ValidationError::out_of_range(
                "dimensions",
                &format!("{}x{}", self.dimensions.0, self.dimensions.1),
                "1",
                "unlimited",
            )
            .into());
        }

        // Validate raw data
        if self.raw_data.is_empty() {
            return Err(ValidationError::required_field("raw_data", "ImageNode").into());
        }

        // Validate file size matches raw data if set
        if self.file_size > 0 && self.file_size != self.raw_data.len() {
            return Err(ValidationError::InvalidFormat {
                field: "file_size".to_string(),
                expected: self.raw_data.len().to_string(),
                actual: self.file_size.to_string(),
                examples: vec!["Use raw_data.len() to set correct file_size".to_string()],
            }
            .into());
        }

        // Validate embedding dimensions if present
        if !self.embedding.is_empty() && self.embedding.len() != 384 {
            return Err(ValidationError::out_of_range(
                "embedding.len()",
                &self.embedding.len().to_string(),
                "384",
                "384",
            )
            .into());
        }

        // Validate GPS coordinates if present
        if let Some((lat, lon)) = self.gps_coordinates {
            if !(-90.0..=90.0).contains(&lat) {
                return Err(ValidationError::out_of_range(
                    "latitude",
                    &lat.to_string(),
                    "-90.0",
                    "90.0",
                )
                .into());
            }
            if !(-180.0..=180.0).contains(&lon) {
                return Err(ValidationError::out_of_range(
                    "longitude",
                    &lon.to_string(),
                    "-180.0",
                    "180.0",
                )
                .into());
            }
        }

        // Validate node type
        if self.node_type != NodeType::Image {
            return Err(ValidationError::invalid_format(
                "node_type",
                "NodeType::Image",
                &format!("{:?}", self.node_type),
            )
            .into());
        }

        Ok(())
    }

    /// Get a summary of the image for display purposes
    pub fn summary(&self) -> String {
        let mut parts = vec![
            format!("{}x{}", self.dimensions.0, self.dimensions.1),
            self.content_type.clone(),
        ];

        if let Some(desc) = &self.user_description {
            parts.push(desc.clone());
        } else if let Some(ai_desc) = &self.ai_metadata.ai_description {
            parts.push(ai_desc.clone());
        }

        if !self.ai_metadata.detected_objects.is_empty() {
            parts.push(format!(
                "Objects: {}",
                self.ai_metadata.detected_objects.join(", ")
            ));
        }

        parts.join(" | ")
    }

    /// Convert to a generic Node for backwards compatibility
    pub fn to_node(&self) -> NodeSpaceResult<Node> {
        let content =
            serde_json::to_value(self).map_err(|e| ProcessingError::SerializationFailed {
                format: "JSON".to_string(),
                reason: e.to_string(),
                data_type: "ImageNode".to_string(),
                fallback_formats: vec!["MessagePack".to_string(), "CBOR".to_string()],
            })?;

        Ok(Node {
            id: self.id.clone(),
            r#type: "image".to_string(),
            content,
            metadata: None,
            created_at: self.created_at.to_rfc3339(),
            updated_at: self.updated_at.to_rfc3339(),
            parent_id: self.parent_id.clone(),
            before_sibling: self.before_sibling.clone(),
            next_sibling: self.next_sibling.clone(),
            root_id: self.root_id.clone(),
        })
    }

    /// Create ImageNode from a generic Node
    pub fn from_node(node: &Node) -> NodeSpaceResult<Self> {
        serde_json::from_value(node.content.clone()).map_err(|e| {
            ProcessingError::SerializationFailed {
                format: "JSON".to_string(),
                reason: format!("Failed to deserialize ImageNode from Node: {}", e),
                data_type: "ImageNode".to_string(),
                fallback_formats: vec!["Direct field access".to_string()],
            }
            .into()
        })
    }
}

// ========================================
// Multi-Level Embedding Types (Shared)
// ========================================

/// Context strategy for contextual embedding generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContextStrategy {
    /// Fast rule-based context generation using parent/sibling/mention patterns
    RuleBased,
    /// Phi-4 enhanced context curation (future implementation)
    Phi4Enhanced,
    /// Adaptive strategy selection based on content analysis
    Adaptive,
}

impl Default for ContextStrategy {
    fn default() -> Self {
        Self::RuleBased
    }
}

/// Node context information for contextual embedding generation
/// Used by core-logic to build context and nlp-engine to generate embeddings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeContext {
    /// Parent node for hierarchical context
    pub parent: Option<Node>,
    /// All sibling nodes for broader context
    pub siblings: Vec<Node>,
    /// Nodes that mention this node (references)
    pub mentions: Vec<Node>,
    /// Related nodes by topic or content similarity
    pub related_nodes: Vec<Node>,
    /// Strategy to use for context generation
    pub strategy: ContextStrategy,
}

impl NodeContext {
    /// Create a new NodeContext with specified strategy
    pub fn with_strategy(strategy: ContextStrategy) -> Self {
        Self {
            strategy,
            ..Default::default()
        }
    }

    /// Add parent context
    pub fn with_parent(mut self, parent: Node) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Add sibling context
    pub fn with_siblings(mut self, siblings: Vec<Node>) -> Self {
        self.siblings = siblings;
        self
    }

    /// Add mention context
    pub fn with_mentions(mut self, mentions: Vec<Node>) -> Self {
        self.mentions = mentions;
        self
    }

    /// Add related nodes context
    pub fn with_related_nodes(mut self, related_nodes: Vec<Node>) -> Self {
        self.related_nodes = related_nodes;
        self
    }
}

/// Performance metrics for embedding generation
/// Used by data-store, core-logic, and nlp-engine for tracking
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmbeddingGenerationMetrics {
    /// Time taken for individual embedding generation (ms)
    pub individual_time_ms: u64,
    /// Time taken for contextual embedding generation (ms)
    pub contextual_time_ms: Option<u64>,
    /// Time taken for hierarchical embedding generation (ms)
    pub hierarchical_time_ms: Option<u64>,
    /// Total time for all embeddings (ms)
    pub total_time_ms: u64,
    /// Context text length used for contextual embedding
    pub context_length: Option<usize>,
    /// Hierarchical path depth
    pub path_depth: Option<usize>,
    /// Cache hits during generation
    pub cache_hits: u8,
    /// Cache misses during generation
    pub cache_misses: u8,
}

/// Multi-level embeddings containing individual, contextual, and hierarchical embeddings
/// Used by data-store for storage, core-logic for caching, and nlp-engine for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiLevelEmbeddings {
    /// Individual embedding - just the node content
    pub individual: Vec<f32>,
    /// Contextual embedding - enhanced with relationship context
    pub contextual: Option<Vec<f32>>,
    /// Hierarchical embedding - full path context from root
    pub hierarchical: Option<Vec<f32>>,
    /// Context strategy used for generation
    pub context_strategy: ContextStrategy,
    /// When the embeddings were generated
    pub generated_at: DateTime<Utc>,
    /// Performance metrics for embedding generation
    pub generation_metrics: EmbeddingGenerationMetrics,
}

impl MultiLevelEmbeddings {
    /// Create new multi-level embeddings with individual embedding
    pub fn new(individual: Vec<f32>, strategy: ContextStrategy) -> Self {
        Self {
            individual,
            contextual: None,
            hierarchical: None,
            context_strategy: strategy,
            generated_at: Utc::now(),
            generation_metrics: EmbeddingGenerationMetrics::default(),
        }
    }

    /// Add contextual embedding
    pub fn with_contextual(mut self, contextual: Vec<f32>) -> Self {
        self.contextual = Some(contextual);
        self
    }

    /// Add hierarchical embedding
    pub fn with_hierarchical(mut self, hierarchical: Vec<f32>) -> Self {
        self.hierarchical = Some(hierarchical);
        self
    }

    /// Add generation metrics
    pub fn with_metrics(mut self, metrics: EmbeddingGenerationMetrics) -> Self {
        self.generation_metrics = metrics;
        self
    }

    /// Check if all embedding levels are available
    pub fn is_complete(&self) -> bool {
        self.contextual.is_some() && self.hierarchical.is_some()
    }

    /// Get the most specific embedding available (hierarchical > contextual > individual)
    pub fn best_embedding(&self) -> &Vec<f32> {
        self.hierarchical
            .as_ref()
            .or(self.contextual.as_ref())
            .unwrap_or(&self.individual)
    }

    /// Count of available embedding levels
    pub fn embedding_levels(&self) -> u8 {
        let mut count = 1; // individual is always present
        if self.contextual.is_some() {
            count += 1;
        }
        if self.hierarchical.is_some() {
            count += 1;
        }
        count
    }
}
