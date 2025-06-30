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

// Core node structure for cross-service communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub content: serde_json::Value,          // Flexible content
    pub metadata: Option<serde_json::Value>, // Optional system metadata
    pub created_at: String,                  // ISO format timestamp
    pub updated_at: String,                  // ISO format timestamp
    // Hierarchical relationship
    pub parent_id: Option<NodeId>, // → Parent node (None = root)
    // Sibling pointer fields for sequential navigation
    pub next_sibling: Option<NodeId>, // → Next node in sequence (None = last)
    pub previous_sibling: Option<NodeId>, // ← Previous node in sequence (None = first)
}

impl Node {
    /// Create a new Node with generated ID and content
    pub fn new(content: serde_json::Value) -> Self {
        #[cfg(feature = "performance-opts")]
        let id = NodeId::new_fast();
        #[cfg(not(feature = "performance-opts"))]
        let id = NodeId::new();

        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id,
            content,
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
            parent_id: None,
            next_sibling: None,
            previous_sibling: None,
        }
    }

    /// Create a Node with existing ID
    pub fn with_id(id: NodeId, content: serde_json::Value) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id,
            content,
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
            parent_id: None,
            next_sibling: None,
            previous_sibling: None,
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
    pub fn with_previous_sibling(mut self, previous_sibling: Option<NodeId>) -> Self {
        self.previous_sibling = previous_sibling;
        self
    }

    /// Set the parent ID
    pub fn with_parent(mut self, parent_id: Option<NodeId>) -> Self {
        self.parent_id = parent_id;
        self
    }

    /// Set both sibling pointers
    pub fn with_siblings(mut self, previous: Option<NodeId>, next: Option<NodeId>) -> Self {
        self.previous_sibling = previous;
        self.next_sibling = next;
        self
    }

    /// Update sibling pointers
    pub fn set_next_sibling(&mut self, next_sibling: Option<NodeId>) {
        self.next_sibling = next_sibling;
        self.touch();
    }

    /// Update previous sibling pointer
    pub fn set_previous_sibling(&mut self, previous_sibling: Option<NodeId>) {
        self.previous_sibling = previous_sibling;
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
    pub fn has_previous_sibling(&self) -> bool {
        self.previous_sibling.is_some()
    }

    /// Check if this node is first in sequence (no previous sibling)
    pub fn is_first(&self) -> bool {
        self.previous_sibling.is_none()
    }

    /// Check if this node is last in sequence (no next sibling)
    pub fn is_last(&self) -> bool {
        self.next_sibling.is_none()
    }

    /// Create a new date node with proper schema-based structure
    pub fn new_date_node(date: chrono::NaiveDate) -> Self {
        let date_metadata = DateNodeMetadata::new(date);
        let content = serde_json::json!({
            "type": "date",
            "content": date_metadata.display_format.clone(),
            "date_metadata": date_metadata
        });

        Self::new(content)
    }

    /// Create a date node with timezone context
    pub fn new_date_node_with_timezone(date: chrono::NaiveDate, timezone: &str) -> Self {
        let date_metadata = DateNodeMetadata::with_timezone(date, timezone);
        let content = serde_json::json!({
            "type": "date",
            "content": date_metadata.display_format.clone(),
            "date_metadata": date_metadata
        });

        Self::new(content)
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

    // Sibling pointer fields for sequential navigation
    pub next_sibling: Option<NodeId>, // → Next image in sequence (None = last)
    pub previous_sibling: Option<NodeId>, // ← Previous image in sequence (None = first)
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
            next_sibling: None,
            previous_sibling: None,
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
            next_sibling: None,
            previous_sibling: None,
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

    /// Set sibling pointers
    pub fn with_siblings(mut self, previous: Option<NodeId>, next: Option<NodeId>) -> Self {
        self.previous_sibling = previous;
        self.next_sibling = next;
        self.touch();
        self
    }

    /// Set next sibling
    pub fn set_next_sibling(&mut self, next_sibling: Option<NodeId>) {
        self.next_sibling = next_sibling;
        self.touch();
    }

    /// Set previous sibling
    pub fn set_previous_sibling(&mut self, previous_sibling: Option<NodeId>) {
        self.previous_sibling = previous_sibling;
        self.touch();
    }

    /// Check if this node has a next sibling
    pub fn has_next_sibling(&self) -> bool {
        self.next_sibling.is_some()
    }

    /// Check if this node has a previous sibling
    pub fn has_previous_sibling(&self) -> bool {
        self.previous_sibling.is_some()
    }

    /// Check if this node is first in sequence
    pub fn is_first(&self) -> bool {
        self.previous_sibling.is_none()
    }

    /// Check if this node is last in sequence
    pub fn is_last(&self) -> bool {
        self.next_sibling.is_none()
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
            content,
            metadata: None,
            created_at: self.created_at.to_rfc3339(),
            updated_at: self.updated_at.to_rfc3339(),
            parent_id: self.parent_id.clone(),
            next_sibling: self.next_sibling.clone(),
            previous_sibling: self.previous_sibling.clone(),
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
    /// Previous sibling for sequential context
    pub previous_sibling: Option<Node>,
    /// Next sibling for sequential context
    pub next_sibling: Option<Node>,
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
    pub fn with_siblings(
        mut self,
        previous: Option<Node>,
        next: Option<Node>,
        all_siblings: Vec<Node>,
    ) -> Self {
        self.previous_sibling = previous;
        self.next_sibling = next;
        self.siblings = all_siblings;
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_node_id_creation() {
        let node_id = NodeId::new();
        // Should be a valid UUID string
        assert_eq!(node_id.as_str().len(), 36); // UUID v4 length
        assert!(node_id.as_str().contains('-')); // UUID format
    }

    #[test]
    fn test_node_id_from_string() {
        let id_str = "some-custom-identifier".to_string();
        let node_id = NodeId::from_string(id_str.clone());
        assert_eq!(node_id.as_str(), "some-custom-identifier");
    }

    #[test]
    fn test_node_creation() {
        let content = json!({"name": "Test User", "email": "test@example.com"});
        let node = Node::new(content.clone());

        assert_eq!(node.id.as_str().len(), 36); // UUID length
        assert_eq!(node.content, content);
        assert!(node.metadata.is_none());
        assert!(!node.created_at.is_empty());
        assert!(!node.updated_at.is_empty());
    }

    #[test]
    fn test_node_with_metadata() {
        let content = json!({"title": "Test Document"});
        let metadata = json!({"version": 1, "author": "test"});
        let node = Node::new(content.clone()).with_metadata(metadata.clone());

        assert_eq!(node.content, content);
        assert_eq!(node.metadata, Some(metadata));
    }

    #[test]
    fn test_relationship_ref() {
        let target_id = NodeId::new();
        let rel = RelationshipRef::new(target_id.clone(), "authored_by".to_string());

        assert_eq!(rel.target_id, target_id);
        assert_eq!(rel.relationship_type, "authored_by");
        assert!(rel.properties.is_null());
    }

    #[test]
    fn test_relationship_with_properties() {
        let target_id = NodeId::new();
        let properties = json!({"role": "admin", "since": "2024-01-01"});
        let rel = RelationshipRef::new(target_id, "member_of".to_string())
            .with_properties(properties.clone());

        assert_eq!(rel.properties, properties);
    }

    #[test]
    fn test_error_display() {
        let error = NodeSpaceError::Database(DatabaseError::connection_failed(
            "postgres",
            "Connection failed",
        ));
        assert_eq!(
            error.to_string(),
            "Database operation failed: Connection failed to postgres: Connection failed"
        );

        let error = NodeSpaceError::Database(DatabaseError::not_found("User", "user-123"));
        assert_eq!(
            error.to_string(),
            "Database operation failed: Record not found: User with id user-123"
        );
    }

    #[test]
    fn test_node_metadata() {
        let mut metadata = NodeMetadata::new();
        let initial_version = metadata.version;
        let initial_updated = metadata.updated_at.clone();

        std::thread::sleep(std::time::Duration::from_millis(10));
        metadata.update();

        assert_eq!(metadata.version, initial_version + 1);
        assert_ne!(metadata.updated_at, initial_updated);
    }

    #[test]
    fn test_serialization() {
        let node_id = NodeId::new();
        let serialized = serde_json::to_string(&node_id).unwrap();
        let deserialized: NodeId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(node_id, deserialized);

        let content = json!({"name": "Test"});
        let node = Node::new(content);
        let serialized = serde_json::to_string(&node).unwrap();
        let deserialized: Node = serde_json::from_str(&serialized).unwrap();
        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.content, deserialized.content);
    }

    #[test]
    fn test_node_space_result() {
        let success: NodeSpaceResult<String> = Ok("success".to_string());
        assert!(success.is_ok());

        let error: NodeSpaceResult<String> =
            Err(ValidationError::required_field("input", "test").into());
        assert!(error.is_err());
    }

    #[test]
    fn test_node_sibling_pointers() {
        let content = json!({"text": "Hello World"});
        let node = Node::new(content);

        // New nodes should have no siblings
        assert!(node.next_sibling.is_none());
        assert!(node.previous_sibling.is_none());
        assert!(node.is_first());
        assert!(node.is_last());
        assert!(!node.has_next_sibling());
        assert!(!node.has_previous_sibling());
    }

    #[test]
    fn test_node_with_siblings() {
        let content = json!({"text": "Middle Node"});
        let prev_id = NodeId::new();
        let next_id = NodeId::new();

        let node = Node::new(content).with_siblings(Some(prev_id.clone()), Some(next_id.clone()));

        assert_eq!(node.previous_sibling, Some(prev_id));
        assert_eq!(node.next_sibling, Some(next_id));
        assert!(!node.is_first());
        assert!(!node.is_last());
        assert!(node.has_next_sibling());
        assert!(node.has_previous_sibling());
    }

    #[test]
    fn test_node_sibling_builder_methods() {
        let content = json!({"text": "Test Node"});
        let next_id = NodeId::new();
        let prev_id = NodeId::new();

        let node = Node::new(content)
            .with_next_sibling(Some(next_id.clone()))
            .with_previous_sibling(Some(prev_id.clone()));

        assert_eq!(node.next_sibling, Some(next_id));
        assert_eq!(node.previous_sibling, Some(prev_id));
    }

    #[test]
    fn test_node_sibling_mutation() {
        let content = json!({"text": "Mutable Node"});
        let mut node = Node::new(content);

        let next_id = NodeId::new();
        let prev_id = NodeId::new();

        // Test setting next sibling
        node.set_next_sibling(Some(next_id.clone()));
        assert_eq!(node.next_sibling, Some(next_id));
        assert!(node.has_next_sibling());
        assert!(!node.is_last());

        // Test setting previous sibling
        node.set_previous_sibling(Some(prev_id.clone()));
        assert_eq!(node.previous_sibling, Some(prev_id));
        assert!(node.has_previous_sibling());
        assert!(!node.is_first());

        // Test clearing siblings
        node.set_next_sibling(None);
        node.set_previous_sibling(None);
        assert!(node.is_first());
        assert!(node.is_last());
    }

    #[test]
    fn test_node_sequence_chain() {
        // Create a chain of 3 nodes: A -> B -> C
        let node_a = Node::new(json!({"text": "Node A"}));
        let node_b = Node::new(json!({"text": "Node B"}));
        let node_c = Node::new(json!({"text": "Node C"}));

        let a_id = node_a.id.clone();
        let b_id = node_b.id.clone();
        let c_id = node_c.id.clone();

        // Set up the chain
        let node_a = node_a.with_next_sibling(Some(b_id.clone()));
        let node_b = node_b
            .with_previous_sibling(Some(a_id.clone()))
            .with_next_sibling(Some(c_id.clone()));
        let node_c = node_c.with_previous_sibling(Some(b_id.clone()));

        // Test chain properties
        assert!(node_a.is_first());
        assert!(!node_a.is_last());
        assert_eq!(node_a.next_sibling, Some(b_id.clone()));

        assert!(!node_b.is_first());
        assert!(!node_b.is_last());
        assert_eq!(node_b.previous_sibling, Some(a_id));
        assert_eq!(node_b.next_sibling, Some(c_id.clone()));

        assert!(!node_c.is_first());
        assert!(node_c.is_last());
        assert_eq!(node_c.previous_sibling, Some(b_id));
    }

    #[test]
    fn test_node_sibling_serialization() {
        let prev_id = NodeId::new();
        let next_id = NodeId::new();
        let content = json!({"text": "Serialization Test"});

        let node = Node::new(content).with_siblings(Some(prev_id.clone()), Some(next_id.clone()));

        // Test serialization
        let serialized = serde_json::to_string(&node).unwrap();
        let deserialized: Node = serde_json::from_str(&serialized).unwrap();

        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.content, deserialized.content);
        assert_eq!(node.next_sibling, deserialized.next_sibling);
        assert_eq!(node.previous_sibling, deserialized.previous_sibling);
        assert_eq!(deserialized.previous_sibling, Some(prev_id));
        assert_eq!(deserialized.next_sibling, Some(next_id));
    }

    #[test]
    fn test_node_backward_compatibility() {
        // Test that nodes without sibling fields can still be deserialized
        let json_without_siblings = r#"{
            "id": "test-id-123",
            "content": {"text": "Legacy Node"},
            "metadata": null,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;

        let node: Node = serde_json::from_str(json_without_siblings).unwrap();
        assert_eq!(node.id.as_str(), "test-id-123");
        assert!(node.next_sibling.is_none());
        assert!(node.previous_sibling.is_none());
        assert!(node.is_first());
        assert!(node.is_last());
    }

    // ImageNode tests
    #[test]
    fn test_node_type_display() {
        assert_eq!(NodeType::Text.to_string(), "text");
        assert_eq!(NodeType::Image.to_string(), "image");
        assert_eq!(NodeType::Task.to_string(), "task");
        assert_eq!(
            NodeType::Custom("blog_post".to_string()).to_string(),
            "blog_post"
        );
    }

    #[test]
    fn test_node_type_default() {
        let default_type: NodeType = Default::default();
        assert_eq!(default_type, NodeType::Text);
    }

    #[test]
    fn test_camera_info_creation() {
        let camera_info = CameraInfo {
            make: Some("Canon".to_string()),
            model: Some("EOS R5".to_string()),
            focal_length: Some(85.0),
            aperture: Some(2.8),
            iso: Some(800),
            ..Default::default()
        };

        assert_eq!(camera_info.make, Some("Canon".to_string()));
        assert_eq!(camera_info.model, Some("EOS R5".to_string()));
        assert_eq!(camera_info.focal_length, Some(85.0));
        assert_eq!(camera_info.aperture, Some(2.8));
        assert_eq!(camera_info.iso, Some(800));
    }

    #[test]
    fn test_image_metadata_creation() {
        let mut confidence_scores = std::collections::HashMap::new();
        confidence_scores.insert("object_detection".to_string(), 0.95);
        confidence_scores.insert("scene_classification".to_string(), 0.87);

        let metadata = ImageMetadata {
            ai_description: Some("A beautiful sunset over mountains".to_string()),
            detected_objects: vec![
                "mountain".to_string(),
                "sky".to_string(),
                "sunset".to_string(),
            ],
            scene_classification: Some("landscape".to_string()),
            keywords: vec!["nature".to_string(), "outdoor".to_string()],
            color_palette: vec![
                "#FF6B35".to_string(),
                "#F7931E".to_string(),
                "#FFD23F".to_string(),
            ],
            text_content: None,
            faces_detected: Some(0),
            emotions: Vec::new(),
            confidence_scores,
        };

        assert_eq!(
            metadata.ai_description,
            Some("A beautiful sunset over mountains".to_string())
        );
        assert_eq!(metadata.detected_objects.len(), 3);
        assert_eq!(metadata.scene_classification, Some("landscape".to_string()));
        assert_eq!(
            metadata.confidence_scores.get("object_detection"),
            Some(&0.95)
        );
    }

    #[test]
    fn test_image_node_creation() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
        let filename = "test_image.jpg".to_string();
        let content_type = "image/jpeg".to_string();
        let dimensions = (1920, 1080);

        let image_node = ImageNode::new(
            raw_data.clone(),
            filename.clone(),
            content_type.clone(),
            dimensions,
        );

        assert_eq!(image_node.node_type, NodeType::Image);
        assert_eq!(image_node.raw_data, raw_data);
        assert_eq!(image_node.filename, filename);
        assert_eq!(image_node.content_type, content_type);
        assert_eq!(image_node.dimensions, dimensions);
        assert!(image_node.embedding.is_empty());
        assert!(image_node.camera_info.is_none());
        assert!(image_node.user_description.is_none());
        assert!(image_node.relationships.is_empty());
    }

    #[test]
    fn test_image_node_with_id() {
        let custom_id = NodeId::from_string("custom-image-id".to_string());
        let raw_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        let filename = "test_image.png".to_string();
        let content_type = "image/png".to_string();
        let dimensions = (800, 600);

        let image_node = ImageNode::with_id(
            custom_id.clone(),
            raw_data,
            filename,
            content_type,
            dimensions,
        );

        assert_eq!(image_node.id, custom_id);
        assert_eq!(image_node.node_type, NodeType::Image);
    }

    #[test]
    fn test_image_node_builder_methods() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let filename = "builder_test.jpg".to_string();
        let content_type = "image/jpeg".to_string();
        let dimensions = (1024, 768);

        let embedding = vec![0.1, 0.2, 0.3]; // Simplified for test
        let camera_info = CameraInfo {
            make: Some("Sony".to_string()),
            model: Some("A7R IV".to_string()),
            ..Default::default()
        };

        let mut confidence_scores = std::collections::HashMap::new();
        confidence_scores.insert("detection".to_string(), 0.92);

        let ai_metadata = ImageMetadata {
            ai_description: Some("Test image description".to_string()),
            detected_objects: vec!["object1".to_string(), "object2".to_string()],
            confidence_scores,
            ..Default::default()
        };

        let user_tags = vec!["test".to_string(), "sample".to_string()];

        let image_node = ImageNode::new(raw_data.clone(), filename, content_type, dimensions)
            .with_file_size(raw_data.len())
            .with_embedding(embedding.clone())
            .with_camera_info(camera_info.clone())
            .with_gps_coordinates(37.7749, -122.4194) // San Francisco
            .with_timestamp(Utc::now())
            .with_ai_metadata(ai_metadata.clone())
            .with_user_description("User-provided description".to_string())
            .with_user_tags(user_tags.clone());

        assert_eq!(image_node.file_size, raw_data.len());
        assert_eq!(image_node.embedding, embedding);
        assert_eq!(image_node.camera_info, Some(camera_info));
        assert_eq!(image_node.gps_coordinates, Some((37.7749, -122.4194)));
        assert!(image_node.timestamp.is_some());
        assert_eq!(image_node.ai_metadata, ai_metadata);
        assert_eq!(
            image_node.user_description,
            Some("User-provided description".to_string())
        );
        assert_eq!(image_node.user_tags, user_tags);
    }

    #[test]
    fn test_image_node_relationships() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let mut image_node = ImageNode::new(
            raw_data,
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (640, 480),
        );

        let related_node_id = NodeId::new();
        let another_node_id = NodeId::new();

        // Test adding relationships
        image_node.add_relationship(related_node_id.clone());
        image_node.add_relationship(another_node_id.clone());
        assert_eq!(image_node.relationships.len(), 2);
        assert!(image_node.relationships.contains(&related_node_id));
        assert!(image_node.relationships.contains(&another_node_id));

        // Test adding duplicate relationship (should not be added)
        image_node.add_relationship(related_node_id.clone());
        assert_eq!(image_node.relationships.len(), 2);

        // Test removing relationship
        image_node.remove_relationship(&related_node_id);
        assert_eq!(image_node.relationships.len(), 1);
        assert!(!image_node.relationships.contains(&related_node_id));
        assert!(image_node.relationships.contains(&another_node_id));
    }

    #[test]
    fn test_image_node_sibling_pointers() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let mut image_node = ImageNode::new(
            raw_data,
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (640, 480),
        );

        // Test initial state
        assert!(image_node.is_first());
        assert!(image_node.is_last());
        assert!(!image_node.has_next_sibling());
        assert!(!image_node.has_previous_sibling());

        // Test setting siblings
        let prev_id = NodeId::new();
        let next_id = NodeId::new();

        image_node.set_next_sibling(Some(next_id.clone()));
        image_node.set_previous_sibling(Some(prev_id.clone()));

        assert!(!image_node.is_first());
        assert!(!image_node.is_last());
        assert!(image_node.has_next_sibling());
        assert!(image_node.has_previous_sibling());
        assert_eq!(image_node.next_sibling, Some(next_id));
        assert_eq!(image_node.previous_sibling, Some(prev_id));
    }

    #[test]
    fn test_image_node_validation_success() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0xFF, 0xD9]; // Valid JPEG
        let image_node = ImageNode::new(
            raw_data,
            "valid.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_gps_coordinates(45.0, 90.0); // Valid coordinates

        assert!(image_node.validate().is_ok());
    }

    #[test]
    fn test_image_node_validation_failures() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];

        // Test empty filename
        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        );
        assert!(invalid_node.validate().is_err());

        // Test empty content type
        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "".to_string(),
            (800, 600),
        );
        assert!(invalid_node.validate().is_err());
        // Test invalid content type
        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "text/plain".to_string(),
            (800, 600),
        );
        assert!(invalid_node.validate().is_err());

        // Test zero dimensions
        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (0, 600),
        );
        assert!(invalid_node.validate().is_err());

        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 0),
        );
        assert!(invalid_node.validate().is_err());
        // Test empty raw data
        let invalid_node = ImageNode::new(
            vec![],
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        );
        assert!(invalid_node.validate().is_err());

        // Test invalid GPS coordinates
        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_gps_coordinates(91.0, 0.0); // Invalid latitude
        assert!(invalid_node.validate().is_err());

        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_gps_coordinates(0.0, 181.0); // Invalid longitude
        assert!(invalid_node.validate().is_err());
        // Test invalid embedding dimensions
        let invalid_embedding = vec![0.1; 256]; // Wrong size (should be 384)
        let invalid_node = ImageNode::new(
            raw_data,
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_embedding(invalid_embedding);
        assert!(invalid_node.validate().is_err());
    }

    #[test]
    fn test_image_node_summary() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];

        // Test with user description
        let image_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (1920, 1080),
        )
        .with_user_description("My vacation photo".to_string());

        let summary = image_node.summary();
        assert!(summary.contains("1920x1080"));
        assert!(summary.contains("image/jpeg"));
        assert!(summary.contains("My vacation photo"));

        // Test with AI description (no user description)
        let ai_metadata = ImageMetadata {
            ai_description: Some("A cityscape at night".to_string()),
            detected_objects: vec!["building".to_string(), "lights".to_string()],
            ..Default::default()
        };

        let image_node = ImageNode::new(
            raw_data,
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (1920, 1080),
        )
        .with_ai_metadata(ai_metadata);

        let summary = image_node.summary();
        assert!(summary.contains("1920x1080"));
        assert!(summary.contains("image/jpeg"));
        assert!(summary.contains("A cityscape at night"));
        assert!(summary.contains("Objects: building, lights"));
    }

    #[test]
    fn test_image_node_to_from_node_conversion() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let camera_info = CameraInfo {
            make: Some("Nikon".to_string()),
            model: Some("D850".to_string()),
            ..Default::default()
        };

        let original_image_node = ImageNode::new(
            raw_data,
            "conversion_test.jpg".to_string(),
            "image/jpeg".to_string(),
            (2048, 1536),
        )
        .with_camera_info(camera_info.clone())
        .with_user_description("Conversion test image".to_string());

        // Convert to Node
        let node = original_image_node.to_node().unwrap();
        assert_eq!(node.id, original_image_node.id);

        // Convert back to ImageNode
        let converted_image_node = ImageNode::from_node(&node).unwrap();
        assert_eq!(converted_image_node.id, original_image_node.id);
        assert_eq!(converted_image_node.filename, original_image_node.filename);
        assert_eq!(
            converted_image_node.content_type,
            original_image_node.content_type
        );
        assert_eq!(
            converted_image_node.dimensions,
            original_image_node.dimensions
        );
        assert_eq!(
            converted_image_node.camera_info,
            original_image_node.camera_info
        );
        assert_eq!(
            converted_image_node.user_description,
            original_image_node.user_description
        );
    }

    #[test]
    fn test_image_node_serialization() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0xFF, 0xD9];
        let mut confidence_scores = std::collections::HashMap::new();
        confidence_scores.insert("object_detection".to_string(), 0.95);

        let ai_metadata = ImageMetadata {
            ai_description: Some("Serialization test".to_string()),
            detected_objects: vec!["test_object".to_string()],
            confidence_scores,
            ..Default::default()
        };

        let image_node = ImageNode::new(
            raw_data,
            "serialize_test.jpg".to_string(),
            "image/jpeg".to_string(),
            (1024, 768),
        )
        .with_ai_metadata(ai_metadata)
        .with_user_tags(vec!["test".to_string(), "serialization".to_string()]);

        // Test serialization
        let serialized = serde_json::to_string(&image_node).unwrap();
        let deserialized: ImageNode = serde_json::from_str(&serialized).unwrap();

        assert_eq!(image_node.id, deserialized.id);
        assert_eq!(image_node.filename, deserialized.filename);
        assert_eq!(image_node.content_type, deserialized.content_type);
        assert_eq!(image_node.dimensions, deserialized.dimensions);
        assert_eq!(image_node.raw_data, deserialized.raw_data);
        assert_eq!(image_node.ai_metadata, deserialized.ai_metadata);
        assert_eq!(image_node.user_tags, deserialized.user_tags);
    }

    #[test]
    fn test_image_node_touch_updates_timestamp() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let mut image_node = ImageNode::new(
            raw_data,
            "touch_test.jpg".to_string(),
            "image/jpeg".to_string(),
            (640, 480),
        );

        let initial_timestamp = image_node.updated_at;

        // Wait a tiny bit to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));

        image_node.touch();

        assert!(image_node.updated_at > initial_timestamp);
    }

    #[test]
    fn test_image_node_with_parent() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let parent_id = NodeId::new();

        let image_node = ImageNode::new(
            raw_data,
            "child_image.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_parent(parent_id.clone());

        assert_eq!(image_node.parent_id, Some(parent_id));
    }

    #[test]
    fn test_image_node_file_size_validation() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0xFF, 0xD9]; // 6 bytes

        // Test with correct file size
        let valid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_file_size(raw_data.len());
        assert!(valid_node.validate().is_ok());

        // Test with incorrect file size
        let invalid_node = ImageNode::new(
            raw_data.clone(),
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_file_size(10); // Wrong size
        assert!(invalid_node.validate().is_err());

        // Test with zero file size (should not validate file size in this case)
        let zero_size_node = ImageNode::new(
            raw_data,
            "test.jpg".to_string(),
            "image/jpeg".to_string(),
            (800, 600),
        )
        .with_file_size(0);
        assert!(zero_size_node.validate().is_ok());
    }

    // Hierarchical Error System Tests
    #[test]
    fn test_database_error_creation() {
        let error = DatabaseError::connection_failed("postgres", "Connection refused");
        assert!(matches!(error, DatabaseError::ConnectionFailed { .. }));
        assert_eq!(
            error.to_string(),
            "Connection failed to postgres: Connection refused"
        );

        let error = DatabaseError::not_found("User", "user-123");
        assert!(matches!(error, DatabaseError::NotFound { .. }));
        assert_eq!(error.to_string(), "Record not found: User with id user-123");
    }

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::required_field("email", "User registration");
        assert!(matches!(
            error,
            ValidationError::RequiredFieldMissing { .. }
        ));
        assert_eq!(
            error.to_string(),
            "Required field missing: email in User registration"
        );

        let error = ValidationError::invalid_format("email", "user@domain.com", "invalid-email");
        assert!(matches!(error, ValidationError::InvalidFormat { .. }));
        assert_eq!(
            error.to_string(),
            "Invalid format for email: expected user@domain.com, got invalid-email"
        );
    }

    #[test]
    fn test_network_error_creation() {
        let error = NetworkError::connection_timeout("https://api.example.com", 5000);
        assert!(matches!(error, NetworkError::ConnectionTimeout { .. }));
        assert_eq!(
            error.to_string(),
            "Connection timeout to https://api.example.com after 5000ms"
        );

        let error = NetworkError::http_error(404, "Not Found", "/api/users/123");
        assert!(matches!(error, NetworkError::HttpError { .. }));
        assert_eq!(error.to_string(), "HTTP error 404: Not Found");
    }

    #[test]
    fn test_processing_error_creation() {
        let error = ProcessingError::model_error("nlp-engine", "mistral-7b", "Out of memory");
        assert!(matches!(error, ProcessingError::ModelError { .. }));
        assert_eq!(
            error.to_string(),
            "AI model error in nlp-engine: mistral-7b - Out of memory"
        );

        let error = ProcessingError::embedding_failed("Invalid input format", "text");
        assert!(matches!(error, ProcessingError::EmbeddingFailed { .. }));
        assert_eq!(
            error.to_string(),
            "Embedding generation failed: Invalid input format"
        );
    }

    #[test]
    fn test_service_error_creation() {
        let error = ServiceError::service_unavailable("data-store", "http://localhost:8080");
        assert!(matches!(error, ServiceError::ServiceUnavailable { .. }));
        assert_eq!(error.to_string(), "Service unavailable: data-store");

        let error = ServiceError::version_mismatch("core-logic", "2.0.0", "1.5.0");
        assert!(matches!(error, ServiceError::VersionMismatch { .. }));
        assert_eq!(
            error.to_string(),
            "Service version mismatch: core-logic expected 2.0.0, got 1.5.0"
        );
    }

    #[test]
    fn test_nodespace_error_hierarchy() {
        let db_error = DatabaseError::connection_failed("postgres", "timeout");
        let ns_error: NodeSpaceError = db_error.into();
        assert!(matches!(ns_error, NodeSpaceError::Database(_)));

        let validation_error = ValidationError::required_field("name", "User");
        let ns_error: NodeSpaceError = validation_error.into();
        assert!(matches!(ns_error, NodeSpaceError::Validation(_)));

        let network_error = NetworkError::connection_timeout("api.test.com", 1000);
        let ns_error: NodeSpaceError = network_error.into();
        assert!(matches!(ns_error, NodeSpaceError::Network(_)));
    }

    #[test]
    fn test_error_utility_methods() {
        // Test retryable analysis
        let retryable_error =
            NodeSpaceError::Network(NetworkError::connection_timeout("api.test.com", 1000));
        assert!(retryable_error.is_retryable());

        let non_retryable_error = NodeSpaceError::Service(ServiceError::CircuitBreakerOpen {
            service: "test".to_string(),
            failure_count: 5,
            failure_threshold: 3,
            reset_time: Utc::now(),
        });
        assert!(!non_retryable_error.is_retryable());

        // Test service attribution
        let service_error = NodeSpaceError::Processing(ProcessingError::model_error(
            "nlp-engine",
            "test-model",
            "error",
        ));
        assert_eq!(
            service_error.service_attribution(),
            Some("nlp-engine".to_string())
        );

        // Test error category
        assert_eq!(retryable_error.error_category(), "network");
        assert_eq!(service_error.error_category(), "processing");
    }

    #[test]
    fn test_error_retry_after() {
        let error_with_retry = NodeSpaceError::Network(NetworkError::RateLimitExceeded {
            limit: 100,
            window: "hour".to_string(),
            reset_time: Utc::now(),
            retry_after: Duration::from_secs(3600),
        });
        assert_eq!(
            error_with_retry.retry_after(),
            Some(Duration::from_secs(3600))
        );

        let error_without_retry =
            NodeSpaceError::Validation(ValidationError::required_field("test", "context"));
        assert_eq!(error_without_retry.retry_after(), None);
    }

    #[test]
    fn test_error_serialization() {
        let error = NodeSpaceError::Database(DatabaseError::ConnectionFailed {
            database: "test_db".to_string(),
            reason: "Network error".to_string(),
            retry_after: Some(Duration::from_secs(5)),
        });

        // Test serialization
        let serialized = serde_json::to_string(&error).unwrap();
        let deserialized: NodeSpaceError = serde_json::from_str(&serialized).unwrap();

        match (&error, &deserialized) {
            (NodeSpaceError::Database(orig), NodeSpaceError::Database(deser)) => {
                assert_eq!(format!("{}", orig), format!("{}", deser));
            }
            _ => panic!("Serialization changed error type"),
        }
    }

    #[cfg(feature = "deprecated-v1")]
    #[test]
    fn test_legacy_compatibility() {
        // Test that legacy constructors still work but are deprecated
        #[allow(deprecated)]
        let legacy_error = NodeSpaceError::database_error("test error");
        assert!(matches!(legacy_error, NodeSpaceError::Database(_)));

        #[allow(deprecated)]
        let legacy_not_found = NodeSpaceError::not_found("item not found");
        assert!(matches!(legacy_not_found, NodeSpaceError::Database(_)));
    }

    #[test]
    fn test_detailed_error_context() {
        // Test that new errors provide much more context than legacy ones
        let detailed_error = NetworkError::HttpError {
            status_code: 429,
            reason: "Rate limit exceeded".to_string(),
            endpoint: "/api/v1/users".to_string(),
            headers: {
                let mut headers = std::collections::HashMap::new();
                headers.insert("X-RateLimit-Remaining".to_string(), "0".to_string());
                headers.insert("X-RateLimit-Reset".to_string(), "1640995200".to_string());
                headers
            },
            retryable: true,
        };

        let error_string = detailed_error.to_string();
        assert!(error_string.contains("429"));
        assert!(error_string.contains("Rate limit exceeded"));

        // Access fields via pattern matching
        match detailed_error {
            NetworkError::HttpError {
                retryable, headers, ..
            } => {
                assert!(retryable);
                assert!(!headers.is_empty());
            }
            _ => panic!("Expected HttpError variant"),
        }
    }

    #[test]
    fn test_error_chaining() {
        // Test that errors chain properly with ? operator
        fn test_function() -> NodeSpaceResult<String> {
            let _db_result = Err(DatabaseError::connection_failed("test", "failed"))?;
            Ok("success".to_string())
        }

        let result = test_function();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NodeSpaceError::Database(_)));
    }

    #[test]
    fn test_structured_error_fields() {
        let validation_error = ValidationError::OutOfRange {
            field: "age".to_string(),
            value: "150".to_string(),
            min: "0".to_string(),
            max: "120".to_string(),
        };

        match validation_error {
            ValidationError::OutOfRange {
                field,
                value,
                min,
                max,
            } => {
                assert_eq!(field, "age");
                assert_eq!(value, "150");
                assert_eq!(min, "0");
                assert_eq!(max, "120");
            }
            _ => panic!("Expected OutOfRange variant"),
        }
    }

    // ========================================
    // Semantic Versioning System Tests
    // ========================================

    #[test]
    fn test_version_constants() {
        assert_eq!(crate::version::V2_API, "2.0");
        assert_eq!(crate::version::V3_PREVIEW, "3.0-preview");
    }

    #[test]
    fn test_core_types_version() {
        assert_eq!(CORE_TYPES_VERSION, "2.0.0");
    }

    #[test]
    fn test_feature_flags() {
        // v2-api should be enabled by default
        assert!(crate::features::is_v2_api_enabled());

        // Active features should include v2-api
        let active = crate::features::active_features();
        assert!(active.contains(&"v2-api"));
    }

    #[test]
    fn test_version_compatibility() {
        // v2 compatibility
        assert!(crate::compatibility::is_compatible_with("2.0"));
        assert!(crate::compatibility::is_compatible_with("2.1"));

        // Compatibility matrix
        let matrix = crate::compatibility::compatibility_matrix();
        assert!(matrix.contains_key("v2-api"));
        assert!(matrix["v2-api"].contains(&"2.0"));
    }

    #[test]
    fn test_nodeid_basic_functionality() {
        let id = NodeId::new();
        assert_eq!(id.as_str().len(), 36); // UUID v4 length

        let custom_id = NodeId::from_string("test-id".to_string());
        assert_eq!(custom_id.as_str(), "test-id");
    }

    #[cfg(feature = "v3-preview")]
    #[test]
    fn test_nodeid_v3_preview_features() {
        let prefixed_id = NodeId::with_prefix("test");
        assert!(prefixed_id.as_str().starts_with("test:"));
        assert_eq!(prefixed_id.prefix(), Some("test"));

        let regular_id = NodeId::new();
        assert_eq!(regular_id.prefix(), None);
    }

    #[cfg(feature = "performance-opts")]
    #[test]
    fn test_performance_optimizations() {
        let fast_id = NodeId::new_fast();
        assert_eq!(fast_id.as_str().len(), 36); // Should still be valid UUID

        // Node creation should use fast NodeId generation
        let node = Node::new(serde_json::json!({"test": "content"}));
        assert_eq!(node.id.as_str().len(), 36);
    }

    #[cfg(feature = "deprecated-v1")]
    #[test]
    fn test_legacy_node_methods() {
        let id = NodeId::new();
        #[allow(deprecated)]
        let legacy_string = id.to_string_legacy();
        assert_eq!(legacy_string, id.as_str());

        let mut node = Node::new(serde_json::json!({"test": "content"}));
        #[allow(deprecated)]
        node.set_metadata_legacy(serde_json::json!({"version": "legacy"}));
        assert!(node.metadata.is_some());
    }

    #[cfg(feature = "enhanced-errors")]
    #[test]
    fn test_enhanced_error_handling() {
        let critical_error = NodeSpaceError::Database(DatabaseError::ConnectionFailed {
            database: "test".to_string(),
            reason: "timeout".to_string(),
            retry_after: None,
        });
        assert_eq!(critical_error.severity(), ErrorSeverity::Critical);

        let warning_error = NodeSpaceError::Database(DatabaseError::QueryTimeout {
            seconds: 30,
            query: "SELECT *".to_string(),
            suggested_limit: None,
        });
        assert_eq!(warning_error.severity(), ErrorSeverity::Warning);

        // Test severity display
        assert_eq!(ErrorSeverity::Critical.to_string(), "CRITICAL");
        assert_eq!(ErrorSeverity::Warning.to_string(), "WARN");
    }

    #[cfg(feature = "v3-preview")]
    #[test]
    fn test_typed_node_creation() {
        #[derive(serde::Serialize)]
        struct TestData {
            name: String,
            value: i32,
        }

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        let node = Node::new_typed(data, "test_type").unwrap();
        assert_eq!(node.node_type(), Some("test_type".to_string()));

        // Verify content was serialized correctly
        assert!(node.content.get("name").is_some());
        assert_eq!(node.content["value"], 42);
    }

    #[test]
    fn test_feature_flag_combinations() {
        // Test that different feature combinations don't conflict
        let active_features = crate::features::active_features();

        // v2-api should always be active (it's default)
        assert!(active_features.contains(&"v2-api"));

        // Log active features for debugging
        println!("Active features: {:?}", active_features);
    }

    #[test]
    fn test_version_migration_scenario() {
        // Simulate a migration scenario where both v2 and v3 features might be active
        let node = Node::new(serde_json::json!({
            "content": "migration test",
            "version": "2.0"
        }));

        // Node should always have basic functionality regardless of features
        assert!(!node.id.as_str().is_empty());
        assert!(node.created_at.len() > 0);
        assert!(node.is_root());
    }

    #[cfg(all(feature = "v3-preview", feature = "enhanced-errors"))]
    #[test]
    fn test_feature_combination_v3_enhanced() {
        // Test combining v3 preview with enhanced errors
        let error = NodeSpaceError::Validation(ValidationError::required_field("test", "context"));
        let _enhanced_error = error.with_context({
            let mut context = std::collections::HashMap::new();
            context.insert("feature_test".to_string(), "v3+enhanced".to_string());
            context
        });

        // Verify the error still has proper severity
        let validation_error =
            NodeSpaceError::Validation(ValidationError::required_field("test", "context"));
        assert_eq!(validation_error.severity(), ErrorSeverity::Error);
    }

    #[test]
    fn test_backward_compatibility_guarantee() {
        // Essential functionality that must work regardless of feature flags
        let node_id = NodeId::new();
        assert!(node_id.as_str().len() > 0);

        let node = Node::new(serde_json::json!({"essential": "functionality"}));
        assert!(node.content.get("essential").is_some());

        let error = DatabaseError::not_found("test", "123");
        let ns_error: NodeSpaceError = error.into();
        assert!(matches!(ns_error, NodeSpaceError::Database(_)));
    }

    #[test]
    fn test_version_policy_compliance() {
        // Test that our versioning follows semantic versioning rules

        // Major version (2.x) should maintain API compatibility within series
        let current_version = CORE_TYPES_VERSION;
        assert!(current_version.starts_with("2."));

        // Feature flags should enable controlled evolution
        assert!(crate::features::is_v2_api_enabled());

        // Compatibility checks should work
        assert!(crate::compatibility::is_compatible_with("2.0"));
        assert!(crate::compatibility::is_compatible_with("2.1"));
    }
}
