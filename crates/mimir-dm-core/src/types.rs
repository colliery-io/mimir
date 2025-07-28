use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Core campaign data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: CampaignSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignSettings {
    pub rule_system: String, // "dnd5e" initially
    pub fts_weights: SearchWeights,
    pub ai_settings: AiSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWeights {
    pub fts_weight: f32,
    pub vector_weight: f32,
    pub recency_weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    pub model: String,
    pub embedding_model: String,
    pub endpoint: String,
}

/// NPC data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub name: String,
    pub description: String,
    pub personality: String,
    pub voice_notes: Option<String>,
    pub relationships: Vec<Relationship>,
    pub location: Option<String>,
    pub status: NpcStatus,
    pub first_seen: Option<Uuid>, // session_id
    pub last_seen: Option<Uuid>,  // session_id
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcStatus {
    Active,
    Inactive, 
    Deceased,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub target_id: Uuid,
    pub relationship_type: RelationshipType,
    pub description: String,
    pub strength: i32, // -100 to 100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Friend,
    Enemy,
    Ally,
    Rival,
    Family,
    Business,
    Romantic,
    Unknown,
}

/// Plot thread data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plot {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub title: String,
    pub summary: String,
    pub status: PlotStatus,
    pub progress: f32, // 0.0 to 100.0
    pub connections: Vec<PlotConnection>,
    pub key_npcs: Vec<Uuid>,
    pub key_locations: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotStatus {
    Active,
    Dormant,
    Resolved,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotConnection {
    pub target_id: Uuid,
    pub connection_type: ConnectionType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Depends,
    Blocks,
    Related,
    Triggers,
}

/// Session data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub number: i32,
    pub title: Option<String>,
    pub date: DateTime<Utc>,
    pub summary: String,
    pub notes: String,
    pub events: Vec<SessionEvent>,
    pub npcs_present: Vec<Uuid>,
    pub plots_advanced: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub id: Uuid,
    pub description: String,
    pub event_type: EventType,
    pub related_npcs: Vec<Uuid>,
    pub related_plots: Vec<Uuid>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Combat,
    Social,
    Exploration,
    PlotDevelopment,
    CharacterDevelopment,
    Other(String),
}

/// Rule data structure for D&D content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: RuleCategory,
    pub source: String,
    pub page: Option<i32>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCategory {
    Spell,
    Item,
    Monster,
    Class,
    Race,
    Background,
    Feat,
    Rule,
    Other(String),
}