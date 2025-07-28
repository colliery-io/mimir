use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::npcs;

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = npcs)]
pub struct Npc {
    pub id: String,
    pub campaign_id: String,
    pub name: String,
    pub description: Option<String>,
    pub personality: Option<String>,
    pub relationships: Option<String>, // JSON serialized
    pub stats: Option<String>, // JSON serialized
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = npcs)]
pub struct NewNpc {
    pub id: String,
    pub campaign_id: String,
    pub name: String,
    pub description: Option<String>,
    pub personality: Option<String>,
    pub relationships: Option<String>,
    pub stats: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = npcs)]
pub struct UpdateNpc {
    pub name: Option<String>,
    pub description: Option<String>,
    pub personality: Option<String>,
    pub relationships: Option<String>,
    pub stats: Option<String>,
    pub updated_at: NaiveDateTime,
}