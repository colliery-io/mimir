use std::collections::HashMap;
use anyhow::Result;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Storage trait for database operations
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Key-value operations
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<()>;
    
    /// Full-text search operations
    async fn fts_search(&self, query: &str, table: FtsTable) -> Result<Vec<FtsResult>>;
    async fn fts_search_all(&self, query: &str) -> Result<Vec<FtsResult>>;
    
    /// Vector search operations
    async fn vector_search(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>>;
    
    /// Hybrid search combining FTS and vector
    async fn hybrid_search(&self, query: &str, options: SearchOptions) -> Result<Vec<SearchResult>>;
}

/// LLM Provider trait for AI operations
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn stream(&self, prompt: &str) -> futures::stream::BoxStream<'_, Result<String>>;
}

/// Agent trait for campaign management agents
#[async_trait::async_trait]
pub trait Agent: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    async fn process(&mut self, request: Request, context: &Context) -> Result<Response>;
    fn capabilities(&self) -> Vec<Capability>;
    fn required_context(&self) -> Vec<ContextRequirement>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FtsTable {
    Rules,
    Npcs,
    Sessions,
    Plots,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtsResult {
    pub id: String,
    pub table: String,
    pub title: String,
    pub content: String,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub content_type: String,
    pub content: String,
    pub score: f32,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub fts_weight: f32,
    pub vector_weight: f32, 
    pub recency_weight: f32,
    pub limit: usize,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            fts_weight: 0.5,
            vector_weight: 0.35,
            recency_weight: 0.15,
            limit: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub from: RequestSource,
    pub intent: Intent,
    pub content: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestSource {
    User,
    Agent(String),
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Intent {
    QueryCampaign { query: String },
    ManageNpc { npc_id: Option<Uuid> },
    TrackPlot { plot_id: Option<Uuid> },
    PrepareSession { session_id: Option<Uuid> },
    SearchRules { query: String },
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: Uuid,
    pub request_id: Uuid,
    pub content: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub campaign_id: Uuid,
    pub session: SessionContext,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub current_session: Option<Uuid>,
    pub recent_npcs: Vec<Uuid>,
    pub active_plots: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    NpcManagement,
    PlotTracking,
    SessionPlanning,
    RuleSearch,
    ConsistencyChecking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextRequirement {
    CampaignAccess,
    NpcDatabase,
    PlotDatabase,
    RuleDatabase,
    SessionHistory,
}