//! Session data access layer

use crate::connection::DbConnection;
use crate::error::Result;
use crate::models::campaign::sessions::{Session, NewSession, UpdateSession};
use crate::schema::sessions;
use diesel::prelude::*;
use chrono::Utc;

/// Repository for session operations
pub struct SessionRepository<'a> {
    conn: &'a mut DbConnection,
}

impl<'a> SessionRepository<'a> {
    /// Create a new session repository
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }
    
    /// Create a new session
    pub fn create(&mut self, new_session: NewSession) -> Result<Session> {
        diesel::insert_into(sessions::table)
            .values(&new_session)
            .returning(Session::as_returning())
            .get_result(self.conn)
            .map_err(Into::into)
    }
    
    /// Find a session by ID
    pub fn find_by_id(&mut self, id: i32) -> Result<Option<Session>> {
        sessions::table
            .find(id)
            .first(self.conn)
            .optional()
            .map_err(Into::into)
    }
    
    /// Update a session
    pub fn update(&mut self, id: i32, update: UpdateSession) -> Result<Session> {
        diesel::update(sessions::table.find(id))
            .set(&update)
            .returning(Session::as_returning())
            .get_result(self.conn)
            .map_err(Into::into)
    }
    
    /// Transition a session to a new status
    pub fn transition_status(&mut self, id: i32, new_status: &str) -> Result<Session> {
        // First, get the session to check if transition is valid
        let session = self.find_by_id(id)?
            .ok_or_else(|| diesel::result::Error::NotFound)?;
            
        if !session.can_transition_to(new_status) {
            return Err(diesel::result::Error::QueryBuilderError(
                format!("Cannot transition from {} to {}", session.status, new_status).into()
            ).into());
        }
        
        let mut update = UpdateSession {
            status: Some(new_status.to_string()),
            ..Default::default()
        };
        
        // Set timestamps based on status transitions
        let now = Utc::now().to_rfc3339();
        match new_status {
            "in_prep" => {
                if session.prep_started_at.is_none() {
                    update.prep_started_at = Some(Some(now));
                }
            }
            "ready" => {
                update.prep_completed_at = Some(Some(now.clone()));
                // Auto-set prep_started_at if not set
                if session.prep_started_at.is_none() {
                    update.prep_started_at = Some(Some(now));
                }
            }
            "complete" => {
                update.completed_at = Some(Some(now));
            }
            _ => {}
        }
        
        self.update(id, update)
    }
    
    /// Delete a session
    pub fn delete(&mut self, id: i32) -> Result<()> {
        diesel::delete(sessions::table.find(id))
            .execute(self.conn)?;
        Ok(())
    }
    
    /// List all sessions for a campaign
    pub fn list_by_campaign(&mut self, campaign_id: i32) -> Result<Vec<Session>> {
        sessions::table
            .filter(sessions::campaign_id.eq(campaign_id))
            .order_by(sessions::session_number)
            .load(self.conn)
            .map_err(Into::into)
    }
    
    /// List sessions by status for a campaign
    pub fn list_by_campaign_and_status(&mut self, campaign_id: i32, status: &str) -> Result<Vec<Session>> {
        sessions::table
            .filter(sessions::campaign_id.eq(campaign_id))
            .filter(sessions::status.eq(status))
            .order_by(sessions::session_number)
            .load(self.conn)
            .map_err(Into::into)
    }
    
    /// List sessions for a module
    pub fn list_by_module(&mut self, module_id: i32) -> Result<Vec<Session>> {
        sessions::table
            .filter(sessions::module_id.eq(module_id))
            .order_by(sessions::session_number)
            .load(self.conn)
            .map_err(Into::into)
    }
    
    /// Find next session to prep (T-3 check)
    pub fn find_sessions_needing_prep(&mut self, campaign_id: i32, days_ahead: i32) -> Result<Vec<Session>> {
        use chrono::Duration;
        
        let target_date = (Utc::now() + Duration::days(days_ahead as i64))
            .date_naive()
            .to_string();
        
        sessions::table
            .filter(sessions::campaign_id.eq(campaign_id))
            .filter(sessions::scheduled_date.le(target_date))
            .filter(sessions::status.eq("next_week"))
            .order_by(sessions::scheduled_date)
            .load(self.conn)
            .map_err(Into::into)
    }
    
    /// Get the next session number for a campaign
    pub fn get_next_session_number(&mut self, campaign_id: i32) -> Result<i32> {
        let max_number = sessions::table
            .filter(sessions::campaign_id.eq(campaign_id))
            .select(diesel::dsl::max(sessions::session_number))
            .first::<Option<i32>>(self.conn)?
            .unwrap_or(0);
            
        Ok(max_number + 1)
    }
}

// Implement Default for UpdateSession
impl Default for UpdateSession {
    fn default() -> Self {
        Self {
            status: None,
            scheduled_date: None,
            prep_started_at: None,
            prep_completed_at: None,
            completed_at: None,
        }
    }
}