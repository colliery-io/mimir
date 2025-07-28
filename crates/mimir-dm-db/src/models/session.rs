use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, NaiveDate};
use crate::schema::sessions;

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: String,
    pub campaign_id: String,
    pub session_number: i32,
    pub date: NaiveDate,
    pub summary: Option<String>,
    pub notes: Option<String>,
    pub participants: Option<String>, // JSON serialized
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub id: String,
    pub campaign_id: String,
    pub session_number: i32,
    pub date: NaiveDate,
    pub summary: Option<String>,
    pub notes: Option<String>,
    pub participants: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct UpdateSession {
    pub session_number: Option<i32>,
    pub date: Option<NaiveDate>,
    pub summary: Option<String>,
    pub notes: Option<String>,
    pub participants: Option<String>,
    pub updated_at: NaiveDateTime,
}