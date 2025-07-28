use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub campaign: CampaignConfig,
    pub llm: LlmConfig,
    pub search: SearchConfig,
    pub agents: AgentConfig,
    pub behaviors: BehaviorConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignConfig {
    pub default_rules: String,
    pub storage_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub embedding_model: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Hybrid search weights (must sum to 1.0)
    pub fts_weight: f32,
    pub vector_weight: f32,
    pub recency_weight: f32,
    
    /// FTS5 configuration  
    pub fts_tokenizer: String,
    pub fts_snippet_length: usize,
    pub fts_highlight_start: String,
    pub fts_highlight_end: String,
    
    /// Search behavior
    pub spell_name_fuzzy_match: bool,
    pub expand_abbreviations: bool,
    pub boost_recent_content: bool,
    pub recent_session_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub enabled: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub enabled: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub animation_speed: String,
    pub context_panel_width: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            campaign: CampaignConfig {
                default_rules: "dnd5e".to_string(),
                storage_path: dirs::data_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("mimir")
                    .join("campaigns"),
            },
            llm: LlmConfig {
                provider: "ollama".to_string(),
                model: "llama3".to_string(),
                embedding_model: "nomic-embed-text".to_string(),
                endpoint: "http://localhost:11434".to_string(),
            },
            search: SearchConfig {
                fts_weight: 0.5,
                vector_weight: 0.35,
                recency_weight: 0.15,
                fts_tokenizer: "porter unicode61 remove_diacritics 2".to_string(),
                fts_snippet_length: 64,
                fts_highlight_start: "**".to_string(),
                fts_highlight_end: "**".to_string(),
                spell_name_fuzzy_match: true,
                expand_abbreviations: true,
                boost_recent_content: true,
                recent_session_count: 3,
            },
            agents: AgentConfig {
                enabled: vec![
                    "npc_manager".to_string(),
                    "plot_manager".to_string(),
                    "session_orchestrator".to_string(),
                    "rule_advisor".to_string(),
                ],
            },
            behaviors: BehaviorConfig {
                enabled: vec![
                    "consistency_check".to_string(),
                    "narrative_enhancement".to_string(),
                    "rule_validation".to_string(),
                ],
            },
            ui: UiConfig {
                theme: "dark".to_string(),
                animation_speed: "normal".to_string(),
                context_panel_width: 40,
            },
        }
    }
}