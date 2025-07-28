use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::campaigns;

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = campaigns)]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub settings: Option<String>, // JSON serialized CampaignSettings
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = campaigns)]
pub struct NewCampaign {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub settings: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = campaigns)]
pub struct UpdateCampaign {
    pub name: Option<String>,
    pub description: Option<String>,
    pub settings: Option<String>,
    pub updated_at: NaiveDateTime,
}