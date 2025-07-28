use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use mimir_dm_core::{Storage, SearchOptions, SearchResult, FtsResult, FtsTable};
use rusqlite::params;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use zerocopy::AsBytes;

use crate::connection::DatabaseConnection;
use crate::models::*;
use crate::schema::*;

pub struct SqliteStorage {
    pub(crate) conn: Arc<Mutex<DatabaseConnection>>,
}

impl SqliteStorage {
    pub fn new(database_url: &str) -> Result<Self> {
        let conn = DatabaseConnection::establish(database_url)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn in_memory() -> Result<Self> {
        let conn = DatabaseConnection::in_memory()?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    // Campaign operations
    pub async fn create_campaign(&self, campaign: NewCampaign) -> Result<Campaign> {
        let mut conn = self.conn.lock().await;
        diesel::insert_into(campaigns::table)
            .values(&campaign)
            .execute(&mut conn.diesel_conn)?;
        
        let result = campaigns::table
            .filter(campaigns::id.eq(&campaign.id))
            .select(Campaign::as_select())
            .first(&mut conn.diesel_conn)?;
        Ok(result)
    }

    pub async fn get_campaign(&self, campaign_id: &str) -> Result<Option<Campaign>> {
        let mut conn = self.conn.lock().await;
        let result = campaigns::table
            .filter(campaigns::id.eq(campaign_id))
            .select(Campaign::as_select())
            .first(&mut conn.diesel_conn)
            .optional()?;
        Ok(result)
    }

    pub async fn update_campaign(&self, campaign_id: &str, update: UpdateCampaign) -> Result<Campaign> {
        let mut conn = self.conn.lock().await;
        diesel::update(campaigns::table.filter(campaigns::id.eq(campaign_id)))
            .set(&update)
            .execute(&mut conn.diesel_conn)?;
            
        let result = campaigns::table
            .filter(campaigns::id.eq(campaign_id))
            .select(Campaign::as_select())
            .first(&mut conn.diesel_conn)?;
        Ok(result)
    }

    pub async fn delete_campaign(&self, campaign_id: &str) -> Result<bool> {
        let mut conn = self.conn.lock().await;
        let affected = diesel::delete(campaigns::table.filter(campaigns::id.eq(campaign_id)))
            .execute(&mut conn.diesel_conn)?;
        Ok(affected > 0)
    }

    // NPC operations
    pub async fn create_npc(&self, npc: NewNpc) -> Result<Npc> {
        let mut conn = self.conn.lock().await;
        diesel::insert_into(npcs::table)
            .values(&npc)
            .execute(&mut conn.diesel_conn)?;
        
        let result = npcs::table
            .filter(npcs::id.eq(&npc.id))
            .select(Npc::as_select())
            .first(&mut conn.diesel_conn)?;
        Ok(result)
    }

    pub async fn get_npc(&self, npc_id: &str) -> Result<Option<Npc>> {
        let mut conn = self.conn.lock().await;
        let result = npcs::table
            .filter(npcs::id.eq(npc_id))
            .select(Npc::as_select())
            .first(&mut conn.diesel_conn)
            .optional()?;
        Ok(result)
    }

    pub async fn get_npcs_by_campaign(&self, campaign_id: &str) -> Result<Vec<Npc>> {
        let mut conn = self.conn.lock().await;
        let results = npcs::table
            .filter(npcs::campaign_id.eq(campaign_id))
            .select(Npc::as_select())
            .load(&mut conn.diesel_conn)?;
        Ok(results)
    }

    // Plot operations
    pub async fn create_plot(&self, plot: NewPlot) -> Result<Plot> {
        let mut conn = self.conn.lock().await;
        diesel::insert_into(plots::table)
            .values(&plot)
            .execute(&mut conn.diesel_conn)?;
        
        let result = plots::table
            .filter(plots::id.eq(&plot.id))
            .select(Plot::as_select())
            .first(&mut conn.diesel_conn)?;
        Ok(result)
    }

    pub async fn get_plots_by_campaign(&self, campaign_id: &str) -> Result<Vec<Plot>> {
        let mut conn = self.conn.lock().await;
        let results = plots::table
            .filter(plots::campaign_id.eq(campaign_id))
            .select(Plot::as_select())
            .load(&mut conn.diesel_conn)?;
        Ok(results)
    }

    // Session operations
    pub async fn create_session(&self, session: NewSession) -> Result<Session> {
        let mut conn = self.conn.lock().await;
        diesel::insert_into(sessions::table)
            .values(&session)
            .execute(&mut conn.diesel_conn)?;
        
        let result = sessions::table
            .filter(sessions::id.eq(&session.id))
            .select(Session::as_select())
            .first(&mut conn.diesel_conn)?;
        Ok(result)
    }

    pub async fn get_sessions_by_campaign(&self, campaign_id: &str) -> Result<Vec<Session>> {
        let mut conn = self.conn.lock().await;
        let results = sessions::table
            .filter(sessions::campaign_id.eq(campaign_id))
            .select(Session::as_select())
            .order(sessions::session_number.asc())
            .load(&mut conn.diesel_conn)?;
        Ok(results)
    }

    // Rule operations
    pub async fn create_rule(&self, rule: NewRule) -> Result<Rule> {
        let mut conn = self.conn.lock().await;
        diesel::insert_into(rules::table)
            .values(&rule)
            .execute(&mut conn.diesel_conn)?;
        
        let result = rules::table
            .filter(rules::id.eq(&rule.id))
            .select(Rule::as_select())
            .first(&mut conn.diesel_conn)?;
            
        // Manually insert into FTS table since we removed triggers
        conn.rusqlite_conn.execute(
            "INSERT INTO rules_fts(rule_id, title, content, category) VALUES (?, ?, ?, ?)",
            params![&rule.id, &rule.title, &rule.content, &rule.category]
        )?;
        
        Ok(result)
    }

    pub async fn get_rules_by_category(&self, category: &str) -> Result<Vec<Rule>> {
        let mut conn = self.conn.lock().await;
        let results = rules::table
            .filter(rules::category.eq(category))
            .select(Rule::as_select())
            .load(&mut conn.diesel_conn)?;
        Ok(results)
    }

    // Embedding operations (using rusqlite directly)
    pub async fn insert_embedding(&self, embedding: NewEmbedding) -> Result<i64> {
        let mut conn = self.conn.lock().await;
        let now = Utc::now().to_rfc3339();
        
        // Insert into vec0 table
        let mut vec_stmt = conn.rusqlite_conn.prepare(
            "INSERT INTO embeddings (embedding) VALUES (?)"
        )?;
        let rowid = vec_stmt.insert(params![embedding.embedding_bytes()])?;
        
        // Insert metadata with the same rowid
        let mut meta_stmt = conn.rusqlite_conn.prepare(
            "INSERT INTO embedding_metadata (rowid, content_type, content_id, content_text, created_at) 
             VALUES (?, ?, ?, ?, ?)"
        )?;
        meta_stmt.execute(params![
            rowid,
            embedding.content_type,
            embedding.content_id,
            embedding.content_text,
            now
        ])?;
        
        Ok(rowid)
    }

    pub async fn vector_search_raw(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<VectorSearchResult>> {
        let mut conn = self.conn.lock().await;
        
        let mut stmt = conn.rusqlite_conn.prepare(
            "SELECT e.rowid, m.content_type, m.content_id, m.content_text, e.distance 
             FROM embeddings e
             JOIN embedding_metadata m ON e.rowid = m.rowid
             WHERE e.embedding MATCH ? AND k = ?
             ORDER BY e.distance"
        )?;
        
        let query_bytes = query_embedding.as_bytes();
        let rows = stmt.query_map(params![query_bytes, limit], |row| {
            Ok(VectorSearchResult {
                rowid: row.get(0)?,
                content_type: row.get(1)?,
                content_id: row.get(2)?,
                content_text: row.get(3)?,
                distance: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        
        Ok(results)
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn get(&self, _key: &str) -> Result<Option<Vec<u8>>> {
        // Simple key-value get - could be implemented with a separate table if needed
        // For now, this is a placeholder
        Ok(None)
    }

    async fn set(&self, _key: &str, _value: Vec<u8>) -> Result<()> {
        // Simple key-value set - could be implemented with a separate table if needed
        // For now, this is a placeholder
        Ok(())
    }

    async fn fts_search(&self, query: &str, table: FtsTable) -> Result<Vec<FtsResult>> {
        let mut conn = self.conn.lock().await;
        
        let (table_name, id_column) = match table {
            FtsTable::Rules => ("rules_fts", "rule_id"),
            FtsTable::Npcs => ("npcs_fts", "npc_id"),
            FtsTable::Plots => ("plots_fts", "plot_id"),
            FtsTable::Sessions => ("sessions_fts", "session_id"),
        };

        let sql = format!(
            "SELECT {}, 1.0 as rank FROM {} WHERE {} MATCH ? LIMIT 50",
            id_column, table_name, table_name
        );

        let mut stmt = conn.rusqlite_conn.prepare(&sql)?;
        let mut rows = stmt.query_map(params![query], |row| {
            Ok(FtsResult {
                id: row.get(0)?,
                table: table_name.to_string(),
                title: "".to_string(), // Could be enhanced
                content: "".to_string(), // Could be enhanced
                score: row.get::<_, f64>(1)? as f32,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        
        Ok(results)
    }

    async fn fts_search_all(&self, query: &str) -> Result<Vec<FtsResult>> {
        // Search across all FTS tables
        let mut results = Vec::new();
        
        for table in [FtsTable::Rules, FtsTable::Npcs, FtsTable::Plots, FtsTable::Sessions] {
            let mut table_results = self.fts_search(query, table).await?;
            results.append(&mut table_results);
        }
        
        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(results)
    }

    async fn vector_search(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>> {
        let vector_results = self.vector_search_raw(embedding, limit).await?;
        
        let results = vector_results
            .into_iter()
            .map(|r| SearchResult {
                id: r.content_id,
                content_type: r.content_type,
                score: (1.0 - r.distance as f32).max(0.0), // Convert distance to similarity
                content: r.content_text,
                metadata: HashMap::new(),
            })
            .collect();
            
        Ok(results)
    }

    async fn hybrid_search(&self, _query: &str, _options: SearchOptions) -> Result<Vec<SearchResult>> {
        // This would implement the hybrid search combining FTS5 and vector search
        // with the weighting formula: 0.5 * fts_score + 0.35 * vector_similarity + 0.15 * recency_weight
        // For now, return empty - this will be implemented when we have embeddings
        Ok(Vec::new())
    }
}