use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::plots;

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = plots)]
pub struct Plot {
    pub id: String,
    pub campaign_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub status: String,
    pub connections: Option<String>, // JSON serialized
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = plots)]
pub struct NewPlot {
    pub id: String,
    pub campaign_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub status: String,
    pub connections: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = plots)]
pub struct UpdatePlot {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub status: Option<String>,
    pub connections: Option<String>,
    pub updated_at: NaiveDateTime,
}