use serde::{Deserialize, Serialize};
use zerocopy::AsBytes;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embedding {
    pub rowid: i64,
    pub content_type: String,
    pub content_id: String,
    pub content_text: String,
    pub embedding: Vec<f32>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewEmbedding {
    pub content_type: String,
    pub content_id: String,
    pub content_text: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub rowid: i64,
    pub content_type: String,
    pub content_id: String,
    pub content_text: String,
    pub distance: f64,
}

impl NewEmbedding {
    pub fn embedding_bytes(&self) -> &[u8] {
        self.embedding.as_bytes()
    }
}