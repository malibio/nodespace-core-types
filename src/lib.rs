use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

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
    // Sibling pointer fields for sequential navigation
    pub next_sibling: Option<NodeId>, // → Next node in sequence (None = last)
    pub previous_sibling: Option<NodeId>, // ← Previous node in sequence (None = first)
}

impl Node {
    /// Create a new Node with generated ID and content
    pub fn new(content: serde_json::Value) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: NodeId::new(),
            content,
            metadata: None,
            created_at: now.clone(),
            updated_at: now,
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

// Comprehensive error type for NodeSpace operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeSpaceError {
    // Data access errors
    DatabaseError(String),
    NotFound(String),

    // Validation errors
    ValidationError(String),
    InvalidData(String),

    // Network/IO errors
    NetworkError(String),
    IoError(String),

    // Processing errors
    ProcessingError(String),
    SerializationError(String),

    // System errors
    ConfigurationError(String),
    InternalError(String),
}

impl fmt::Display for NodeSpaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeSpaceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            NodeSpaceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            NodeSpaceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            NodeSpaceError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            NodeSpaceError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            NodeSpaceError::IoError(msg) => write!(f, "IO error: {}", msg),
            NodeSpaceError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            NodeSpaceError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            NodeSpaceError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            NodeSpaceError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for NodeSpaceError {}

// Standard Result type for all NodeSpace operations
pub type NodeSpaceResult<T> = Result<T, NodeSpaceError>;

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
        let error = NodeSpaceError::DatabaseError("Connection failed".to_string());
        assert_eq!(error.to_string(), "Database error: Connection failed");

        let error = NodeSpaceError::NotFound("User not found".to_string());
        assert_eq!(error.to_string(), "Not found: User not found");
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
            Err(NodeSpaceError::ValidationError("Invalid input".to_string()));
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
}
