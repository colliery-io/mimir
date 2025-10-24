//! # GGUF Provider Configuration
//!
//! Configuration structures and parsing for the GGUF provider.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

use crate::config::ModelConfig;
use crate::traits::LlmError;

/// Configuration for the GGUF provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GgufConfig {
    /// Path to the GGUF model file
    pub model_path: PathBuf,
    
    /// Maximum context size (tokens)
    #[serde(default = "default_context_size")]
    pub context_size: usize,
    
    /// Number of GPU layers to offload (-1 for all, 0 for none)
    #[serde(default = "default_gpu_layers")]
    pub gpu_layers: i32,
    
    /// Batch size for prompt processing
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    
    /// Number of threads for CPU inference
    #[serde(default)]
    pub threads: Option<usize>,
    
    /// Keep model in RAM (prevent swapping)
    #[serde(default)]
    pub mlock: bool,
    
    /// Use memory mapping for model loading
    #[serde(default = "default_mmap")]
    pub mmap: bool,
    
    /// Temperature for sampling
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    
    /// Top-p sampling parameter
    #[serde(default)]
    pub top_p: Option<f32>,
    
    /// Top-k sampling parameter  
    #[serde(default)]
    pub top_k: Option<u32>,
    
    /// Repetition penalty
    #[serde(default = "default_repeat_penalty")]
    pub repeat_penalty: f32,
    
    /// Seed for reproducible generation
    #[serde(default)]
    pub seed: Option<u64>,
}

fn default_context_size() -> usize { 4096 }
fn default_gpu_layers() -> i32 { -1 }
fn default_batch_size() -> usize { 512 }
fn default_mmap() -> bool { true }
fn default_temperature() -> f32 { 0.7 }
fn default_repeat_penalty() -> f32 { 1.1 }

impl Default for GgufConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("model.gguf"),
            context_size: default_context_size(),
            gpu_layers: default_gpu_layers(),
            batch_size: default_batch_size(),
            threads: None,
            mlock: false,
            mmap: default_mmap(),
            temperature: default_temperature(),
            top_p: None,
            top_k: None,
            repeat_penalty: default_repeat_penalty(),
            seed: None,
        }
    }
}

impl GgufConfig {
    /// Create GGUF config from ModelConfig
    pub fn from_model_config(config: &ModelConfig) -> Result<Self, LlmError> {
        let config_map = config.config.as_ref()
            .ok_or_else(|| LlmError::ConfigError("Missing config section".to_string()))?;
        
        // Required: model_path
        let model_path = config_map.get("model_path")
            .ok_or_else(|| LlmError::ConfigError("Missing model_path in config".to_string()))?;
        
        let mut gguf_config = Self {
            model_path: PathBuf::from(model_path),
            ..Default::default()
        };
        
        // Parse optional configurations
        if let Some(context_size) = config_map.get("context_size") {
            gguf_config.context_size = context_size.parse()
                .map_err(|_| LlmError::ConfigError("Invalid context_size".to_string()))?;
        }
        
        if let Some(gpu_layers) = config_map.get("gpu_layers") {
            gguf_config.gpu_layers = gpu_layers.parse()
                .map_err(|_| LlmError::ConfigError("Invalid gpu_layers".to_string()))?;
        }
        
        if let Some(batch_size) = config_map.get("batch_size") {
            gguf_config.batch_size = batch_size.parse()
                .map_err(|_| LlmError::ConfigError("Invalid batch_size".to_string()))?;
        }
        
        if let Some(threads) = config_map.get("threads") {
            gguf_config.threads = Some(threads.parse()
                .map_err(|_| LlmError::ConfigError("Invalid threads".to_string()))?);
        }
        
        if let Some(mlock) = config_map.get("mlock") {
            gguf_config.mlock = mlock.parse()
                .map_err(|_| LlmError::ConfigError("Invalid mlock".to_string()))?;
        }
        
        if let Some(mmap) = config_map.get("mmap") {
            gguf_config.mmap = mmap.parse()
                .map_err(|_| LlmError::ConfigError("Invalid mmap".to_string()))?;
        }
        
        if let Some(temperature) = config_map.get("temperature") {
            gguf_config.temperature = temperature.parse()
                .map_err(|_| LlmError::ConfigError("Invalid temperature".to_string()))?;
        }
        
        if let Some(top_p) = config_map.get("top_p") {
            gguf_config.top_p = Some(top_p.parse()
                .map_err(|_| LlmError::ConfigError("Invalid top_p".to_string()))?);
        }
        
        if let Some(top_k) = config_map.get("top_k") {
            gguf_config.top_k = Some(top_k.parse()
                .map_err(|_| LlmError::ConfigError("Invalid top_k".to_string()))?);
        }
        
        if let Some(repeat_penalty) = config_map.get("repeat_penalty") {
            gguf_config.repeat_penalty = repeat_penalty.parse()
                .map_err(|_| LlmError::ConfigError("Invalid repeat_penalty".to_string()))?;
        }
        
        if let Some(seed) = config_map.get("seed") {
            gguf_config.seed = Some(seed.parse()
                .map_err(|_| LlmError::ConfigError("Invalid seed".to_string()))?);
        }
        
        Ok(gguf_config)
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), LlmError> {
        // Check model file exists
        if !self.model_path.exists() {
            return Err(LlmError::ConfigError(
                format!("Model file does not exist: {}", self.model_path.display())
            ));
        }
        
        // Check model file has .gguf extension
        if self.model_path.extension().and_then(|s| s.to_str()) != Some("gguf") {
            return Err(LlmError::ConfigError(
                "Model file must have .gguf extension".to_string()
            ));
        }
        
        // Validate numeric ranges
        if self.context_size == 0 {
            return Err(LlmError::ConfigError("context_size must be greater than 0".to_string()));
        }
        
        if self.batch_size == 0 {
            return Err(LlmError::ConfigError("batch_size must be greater than 0".to_string()));
        }
        
        if self.temperature < 0.0 {
            return Err(LlmError::ConfigError("temperature must be non-negative".to_string()));
        }
        
        if let Some(top_p) = self.top_p {
            if top_p <= 0.0 || top_p > 1.0 {
                return Err(LlmError::ConfigError("top_p must be between 0 and 1".to_string()));
            }
        }
        
        if self.repeat_penalty <= 0.0 {
            return Err(LlmError::ConfigError("repeat_penalty must be positive".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_default_config() {
        let config = GgufConfig::default();
        assert_eq!(config.context_size, 4096);
        assert_eq!(config.gpu_layers, -1);
        assert_eq!(config.temperature, 0.7);
        assert!(config.mmap);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = GgufConfig::default();
        
        // Should fail - file doesn't exist
        assert!(config.validate().is_err());
        
        // Should fail - invalid context size
        config.context_size = 0;
        assert!(config.validate().is_err());
        
        // Should fail - invalid temperature
        config.context_size = 4096;
        config.temperature = -1.0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_from_model_config() {
        let mut config_map = HashMap::new();
        config_map.insert("model_path".to_string(), "/path/to/model.gguf".to_string());
        config_map.insert("context_size".to_string(), "8192".to_string());
        config_map.insert("gpu_layers".to_string(), "32".to_string());
        
        let model_config = ModelConfig {
            name: "test".to_string(),
            supported_endpoints: vec![],
            provider: "gguf".to_string(),
            model: "test-model".to_string(),
            config: Some(config_map),
            limit: None,
        };
        
        let gguf_config = GgufConfig::from_model_config(&model_config).unwrap();
        assert_eq!(gguf_config.context_size, 8192);
        assert_eq!(gguf_config.gpu_layers, 32);
        assert_eq!(gguf_config.model_path, PathBuf::from("/path/to/model.gguf"));
    }
}