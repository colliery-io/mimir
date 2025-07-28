use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::rules;

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = rules)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub source: Option<String>,
    pub page_reference: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = rules)]
pub struct NewRule {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub source: Option<String>,
    pub page_reference: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = rules)]
pub struct UpdateRule {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub source: Option<String>,
    pub page_reference: Option<String>,
    pub updated_at: NaiveDateTime,
}