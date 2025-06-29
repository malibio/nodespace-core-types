use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
use chrono::{DateTime, Utc};

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

// Node type classification enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
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

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Text
    }
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

// Camera information from EXIF data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraInfo {
    pub make: Option<String>,
    pub model: Option<String>,
    pub software: Option<String>,
    pub lens_model: Option<String>,
    pub focal_length: Option<f32>,        // in mm
    pub aperture: Option<f32>,            // f-stop value
    pub shutter_speed: Option<String>,    // e.g., "1/60"
    pub iso: Option<u32>,
    pub flash: Option<bool>,
    pub white_balance: Option<String>,
    pub orientation: Option<u32>,         // EXIF orientation value 1-8
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            make: None,
            model: None,
            software: None,
            lens_model: None,
            focal_length: None,
            aperture: None,
            shutter_speed: None,
            iso: None,
            flash: None,
            white_balance: None,
            orientation: None,
        }
    }
}

// Image metadata extraction results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub ai_description: Option<String>,
    pub detected_objects: Vec<String>,
    pub scene_classification: Option<String>,
    pub keywords: Vec<String>,
    pub color_palette: Vec<String>,        // dominant colors as hex codes
    pub text_content: Option<String>,      // OCR extracted text
    pub faces_detected: Option<u32>,       // number of faces
    pub emotions: Vec<String>,             // detected emotions
    pub confidence_scores: std::collections::HashMap<String, f32>, // AI confidence for various detections
}

impl Default for ImageMetadata {
    fn default() -> Self {
        Self {
            ai_description: None,
            detected_objects: Vec::new(),
            scene_classification: None,
            keywords: Vec::new(),
            color_palette: Vec::new(),
            text_content: None,
            faces_detected: None,
            emotions: Vec::new(),
            confidence_scores: std::collections::HashMap::new(),
        }
    }
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
    pub dimensions: (u32, u32), // (width, height)
    pub timestamp: Option<DateTime<Utc>>, // from EXIF or file metadata
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
            return Err(NodeSpaceError::ValidationError("Filename cannot be empty".to_string()));
        }

        if self.content_type.is_empty() {
            return Err(NodeSpaceError::ValidationError("Content type cannot be empty".to_string()));
        }

        // Validate content type is image
        if !self.content_type.starts_with("image/") {
            return Err(NodeSpaceError::ValidationError(
                format!("Invalid content type for image: {}", self.content_type)
            ));
        }

        // Validate dimensions
        if self.dimensions.0 == 0 || self.dimensions.1 == 0 {
            return Err(NodeSpaceError::ValidationError("Image dimensions must be greater than 0".to_string()));
        }

        // Validate raw data
        if self.raw_data.is_empty() {
            return Err(NodeSpaceError::ValidationError("Image raw data cannot be empty".to_string()));
        }

        // Validate file size matches raw data if set
        if self.file_size > 0 && self.file_size != self.raw_data.len() {
            return Err(NodeSpaceError::ValidationError(
                "File size does not match raw data length".to_string()
            ));
        }

        // Validate embedding dimensions if present
        if !self.embedding.is_empty() && self.embedding.len() != 384 {
            return Err(NodeSpaceError::ValidationError(
                format!("Invalid embedding dimensions: expected 384, got {}", self.embedding.len())
            ));
        }

        // Validate GPS coordinates if present
        if let Some((lat, lon)) = self.gps_coordinates {
            if lat < -90.0 || lat > 90.0 {
                return Err(NodeSpaceError::ValidationError(
                    format!("Invalid latitude: {}", lat)
                ));
            }
            if lon < -180.0 || lon > 180.0 {
                return Err(NodeSpaceError::ValidationError(
                    format!("Invalid longitude: {}", lon)
                ));
            }
        }

        // Validate node type
        if self.node_type != NodeType::Image {
            return Err(NodeSpaceError::ValidationError(
                "Node type must be Image for ImageNode".to_string()
            ));
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
            parts.push(format!("Objects: {}", self.ai_metadata.detected_objects.join(", ")));
        }

        parts.join(" | ")
    }

    /// Convert to a generic Node for backwards compatibility
    pub fn to_node(&self) -> NodeSpaceResult<Node> {
        let content = serde_json::to_value(self)
            .map_err(|e| NodeSpaceError::SerializationError(e.to_string()))?;
        
        Ok(Node {
            id: self.id.clone(),
            content,
            metadata: None,
            created_at: self.created_at.to_rfc3339(),
            updated_at: self.updated_at.to_rfc3339(),
            next_sibling: self.next_sibling.clone(),
            previous_sibling: self.previous_sibling.clone(),
        })
    }

    /// Create ImageNode from a generic Node
    pub fn from_node(node: &Node) -> NodeSpaceResult<Self> {
        serde_json::from_value(node.content.clone())
            .map_err(|e| NodeSpaceError::SerializationError(
                format!("Failed to deserialize ImageNode from Node: {}", e)
            ))
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

    // ImageNode tests
    #[test]
    fn test_node_type_display() {
        assert_eq!(NodeType::Text.to_string(), "text");
        assert_eq!(NodeType::Image.to_string(), "image");
        assert_eq!(NodeType::Task.to_string(), "task");
        assert_eq!(NodeType::Custom("blog_post".to_string()).to_string(), "blog_post");
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
            detected_objects: vec!["mountain".to_string(), "sky".to_string(), "sunset".to_string()],
            scene_classification: Some("landscape".to_string()),
            keywords: vec!["nature".to_string(), "outdoor".to_string()],
            color_palette: vec!["#FF6B35".to_string(), "#F7931E".to_string(), "#FFD23F".to_string()],
            text_content: None,
            faces_detected: Some(0),
            emotions: Vec::new(),
            confidence_scores,
        };

        assert_eq!(metadata.ai_description, Some("A beautiful sunset over mountains".to_string()));
        assert_eq!(metadata.detected_objects.len(), 3);
        assert_eq!(metadata.scene_classification, Some("landscape".to_string()));
        assert_eq!(metadata.confidence_scores.get("object_detection"), Some(&0.95));
    }

    #[test]
    fn test_image_node_creation() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
        let filename = "test_image.jpg".to_string();
        let content_type = "image/jpeg".to_string();
        let dimensions = (1920, 1080);

        let image_node = ImageNode::new(raw_data.clone(), filename.clone(), content_type.clone(), dimensions);

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

        let image_node = ImageNode::with_id(custom_id.clone(), raw_data, filename, content_type, dimensions);

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
        assert_eq!(image_node.user_description, Some("User-provided description".to_string()));
        assert_eq!(image_node.user_tags, user_tags);
    }

    #[test]
    fn test_image_node_relationships() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let mut image_node = ImageNode::new(raw_data, "test.jpg".to_string(), "image/jpeg".to_string(), (640, 480));

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
        let mut image_node = ImageNode::new(raw_data, "test.jpg".to_string(), "image/jpeg".to_string(), (640, 480));

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
        let image_node = ImageNode::new(raw_data, "valid.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_gps_coordinates(45.0, 90.0); // Valid coordinates

        assert!(image_node.validate().is_ok());
    }

    #[test]
    fn test_image_node_validation_failures() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];

        // Test empty filename
        let invalid_node = ImageNode::new(raw_data.clone(), "".to_string(), "image/jpeg".to_string(), (800, 600));
        assert!(invalid_node.validate().is_err());

        // Test empty content type
        let invalid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "".to_string(), (800, 600));
        assert!(invalid_node.validate().is_err());

        // Test invalid content type
        let invalid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "text/plain".to_string(), (800, 600));
        assert!(invalid_node.validate().is_err());

        // Test zero dimensions
        let invalid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "image/jpeg".to_string(), (0, 600));
        assert!(invalid_node.validate().is_err());

        let invalid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "image/jpeg".to_string(), (800, 0));
        assert!(invalid_node.validate().is_err());

        // Test empty raw data
        let invalid_node = ImageNode::new(vec![], "test.jpg".to_string(), "image/jpeg".to_string(), (800, 600));
        assert!(invalid_node.validate().is_err());

        // Test invalid GPS coordinates
        let invalid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_gps_coordinates(91.0, 0.0); // Invalid latitude
        assert!(invalid_node.validate().is_err());

        let invalid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_gps_coordinates(0.0, 181.0); // Invalid longitude
        assert!(invalid_node.validate().is_err());

        // Test invalid embedding dimensions
        let invalid_embedding = vec![0.1; 256]; // Wrong size (should be 384)
        let invalid_node = ImageNode::new(raw_data, "test.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_embedding(invalid_embedding);
        assert!(invalid_node.validate().is_err());
    }

    #[test]
    fn test_image_node_summary() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        
        // Test with user description
        let image_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "image/jpeg".to_string(), (1920, 1080))
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

        let image_node = ImageNode::new(raw_data, "test.jpg".to_string(), "image/jpeg".to_string(), (1920, 1080))
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

        let original_image_node = ImageNode::new(raw_data, "conversion_test.jpg".to_string(), "image/jpeg".to_string(), (2048, 1536))
            .with_camera_info(camera_info.clone())
            .with_user_description("Conversion test image".to_string());

        // Convert to Node
        let node = original_image_node.to_node().unwrap();
        assert_eq!(node.id, original_image_node.id);

        // Convert back to ImageNode
        let converted_image_node = ImageNode::from_node(&node).unwrap();
        assert_eq!(converted_image_node.id, original_image_node.id);
        assert_eq!(converted_image_node.filename, original_image_node.filename);
        assert_eq!(converted_image_node.content_type, original_image_node.content_type);
        assert_eq!(converted_image_node.dimensions, original_image_node.dimensions);
        assert_eq!(converted_image_node.camera_info, original_image_node.camera_info);
        assert_eq!(converted_image_node.user_description, original_image_node.user_description);
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

        let image_node = ImageNode::new(raw_data, "serialize_test.jpg".to_string(), "image/jpeg".to_string(), (1024, 768))
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
        let mut image_node = ImageNode::new(raw_data, "touch_test.jpg".to_string(), "image/jpeg".to_string(), (640, 480));
        
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
        
        let image_node = ImageNode::new(raw_data, "child_image.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_parent(parent_id.clone());
        
        assert_eq!(image_node.parent_id, Some(parent_id));
    }

    #[test]
    fn test_image_node_file_size_validation() {
        let raw_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0xFF, 0xD9]; // 6 bytes
        
        // Test with correct file size
        let valid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_file_size(raw_data.len());
        assert!(valid_node.validate().is_ok());
        
        // Test with incorrect file size
        let invalid_node = ImageNode::new(raw_data.clone(), "test.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_file_size(10); // Wrong size
        assert!(invalid_node.validate().is_err());
        
        // Test with zero file size (should not validate file size in this case)
        let zero_size_node = ImageNode::new(raw_data, "test.jpg".to_string(), "image/jpeg".to_string(), (800, 600))
            .with_file_size(0);
        assert!(zero_size_node.validate().is_ok());
    }
}
