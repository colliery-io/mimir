//! Type-safe ID wrappers for domain entities

use serde::{Deserialize, Serialize};

/// Type-safe wrapper for Campaign IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CampaignId(pub i32);

impl CampaignId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl From<i32> for CampaignId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

/// Type-safe wrapper for Module IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModuleId(pub i32);

impl ModuleId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl From<i32> for ModuleId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

/// Type-safe wrapper for Session IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub i32);

impl SessionId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl From<i32> for SessionId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}