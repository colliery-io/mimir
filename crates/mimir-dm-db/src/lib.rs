pub mod connection;
pub mod models;
pub mod schema;
pub mod storage;

pub use connection::*;
pub use models::*;
pub use storage::*;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    use uuid::Uuid;
    use zerocopy::AsBytes;
    
    // Test fixture to ensure clean database for each test
    async fn setup_test_storage() -> anyhow::Result<SqliteStorage> {
        // Use a unique in-memory database for each test to avoid conflicts
        let db_name = format!(":memory:{}", uuid::Uuid::new_v4());
        SqliteStorage::new(&db_name)
    }

    #[tokio::test]
    async fn test_sqlite_vec_basic_functionality() -> anyhow::Result<()> {
        let storage = setup_test_storage().await?;
        let conn = storage.conn.lock().await;
        
        // Test basic sqlite-vec functionality like the example you provided
        let v: Vec<f32> = vec![0.1, 0.2, 0.3];

        let (vec_version, embedding): (String, String) = conn.rusqlite_conn.query_row(
            "select vec_version(), vec_to_json(?)",
            &[v.as_bytes()],
            |x| Ok((x.get(0)?, x.get(1)?)),
        )?;

        println!("vec_version={vec_version}, embedding={embedding}");
        assert!(!vec_version.is_empty());
        assert!(embedding.contains("0.1"));
        
        // Create a simple vec0 table
        conn.rusqlite_conn.execute(
            "CREATE VIRTUAL TABLE test_vec USING vec0(embedding float[3])",
            [],
        )?;
        
        // Insert a vector using the proper format
        conn.rusqlite_conn.execute(
            "INSERT INTO test_vec (rowid, embedding) VALUES (1, ?)",
            &[v.as_bytes()],
        )?;
        
        // Test vector search
        let query_vec: Vec<f32> = vec![0.11, 0.21, 0.31];
        let mut search_stmt = conn.rusqlite_conn.prepare(
            "SELECT rowid, distance FROM test_vec WHERE embedding MATCH ? ORDER BY distance LIMIT 1"
        )?;
        
        let search_result: (i64, f64) = search_stmt.query_row(&[query_vec.as_bytes()], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        
        assert_eq!(search_result.0, 1);
        println!("Vector search successful. Distance: {}", search_result.1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_campaign_crud() -> anyhow::Result<()> {
        let storage = setup_test_storage().await?;
        
        // Create campaign
        let campaign_id = Uuid::new_v4().to_string();
        let new_campaign = NewCampaign {
            id: campaign_id.clone(),
            name: "Test Campaign".to_string(),
            description: Some("A test campaign".to_string()),
            settings: None,
        };
        
        let created = storage.create_campaign(new_campaign).await?;
        assert_eq!(created.name, "Test Campaign");
        assert_eq!(created.id, campaign_id);
        
        // Get campaign
        let retrieved = storage.get_campaign(&campaign_id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Campaign");
        
        // Update campaign
        let update = UpdateCampaign {
            name: Some("Updated Campaign".to_string()),
            description: None,
            settings: None,
            updated_at: Utc::now().naive_utc(),
        };
        
        let updated = storage.update_campaign(&campaign_id, update).await?;
        assert_eq!(updated.name, "Updated Campaign");
        
        // Delete campaign
        let deleted = storage.delete_campaign(&campaign_id).await?;
        assert!(deleted);
        
        let not_found = storage.get_campaign(&campaign_id).await?;
        assert!(not_found.is_none());
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_npc_operations() -> anyhow::Result<()> {
        let storage = setup_test_storage().await?;
        
        // First create a campaign
        let campaign_id = Uuid::new_v4().to_string();
        let new_campaign = NewCampaign {
            id: campaign_id.clone(),
            name: "Test Campaign".to_string(),
            description: None,
            settings: None,
        };
        storage.create_campaign(new_campaign).await?;
        
        // Create NPC
        let npc_id = Uuid::new_v4().to_string();
        let new_npc = NewNpc {
            id: npc_id.clone(),
            campaign_id: campaign_id.clone(),
            name: "Gandalf".to_string(),
            description: Some("A wizard".to_string()),
            personality: Some("Wise and mysterious".to_string()),
            relationships: None,
            stats: None,
        };
        
        let created = storage.create_npc(new_npc).await?;
        assert_eq!(created.name, "Gandalf");
        assert_eq!(created.campaign_id, campaign_id);
        
        // Get NPC
        let retrieved = storage.get_npc(&npc_id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Gandalf");
        
        // Get NPCs by campaign
        let npcs = storage.get_npcs_by_campaign(&campaign_id).await?;
        assert_eq!(npcs.len(), 1);
        assert_eq!(npcs[0].name, "Gandalf");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_rule_operations() -> anyhow::Result<()> {
        let storage = setup_test_storage().await?;
        
        // Create rule
        let rule_id = Uuid::new_v4().to_string();
        let new_rule = NewRule {
            id: rule_id.clone(),
            title: "Fireball".to_string(),
            content: "A 3rd level evocation spell that deals fire damage".to_string(),
            category: "spell".to_string(),
            source: Some("PHB".to_string()),
            page_reference: Some("241".to_string()),
        };
        
        let created = storage.create_rule(new_rule).await?;
        assert_eq!(created.title, "Fireball");
        assert_eq!(created.category, "spell");
        
        // Get rules by category
        let spells = storage.get_rules_by_category("spell").await?;
        assert_eq!(spells.len(), 1);
        assert_eq!(spells[0].title, "Fireball");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_session_operations() -> anyhow::Result<()> {
        let storage = setup_test_storage().await?;
        
        // First create a campaign
        let campaign_id = Uuid::new_v4().to_string();
        let new_campaign = NewCampaign {
            id: campaign_id.clone(),
            name: "Test Campaign".to_string(),
            description: None,
            settings: None,
        };
        storage.create_campaign(new_campaign).await?;
        
        // Create session
        let session_id = Uuid::new_v4().to_string();
        let new_session = NewSession {
            id: session_id.clone(),
            campaign_id: campaign_id.clone(),
            session_number: 1,
            date: NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
            summary: Some("First session".to_string()),
            notes: Some("Party met in tavern".to_string()),
            participants: None,
        };
        
        let created = storage.create_session(new_session).await?;
        assert_eq!(created.session_number, 1);
        assert_eq!(created.campaign_id, campaign_id);
        
        // Get sessions by campaign
        let sessions = storage.get_sessions_by_campaign(&campaign_id).await?;
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].summary, Some("First session".to_string()));
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_vector_embedding_operations() -> anyhow::Result<()> {
        let storage = setup_test_storage().await?;
        
        // Create embedding with proper 384-dimensional vector (nomic-embed-text size)
        let embedding = NewEmbedding {
            content_type: "rule".to_string(),
            content_id: "spell-fireball".to_string(),
            content_text: "Fireball is a 3rd level evocation spell that deals fire damage".to_string(),
            embedding: (0..384).map(|i| (i as f32) * 0.001).collect(), // Generate 384-dim vector
        };
        
        // Insert embedding
        let rowid = storage.insert_embedding(embedding).await?;
        assert!(rowid > 0);
        println!("Successfully inserted embedding with rowid: {}", rowid);
        
        // Test vector search
        let query_embedding: Vec<f32> = (0..384).map(|i| (i as f32) * 0.001 + 0.0001).collect();
        let results = storage.vector_search_raw(&query_embedding, 1).await?;
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content_type, "rule");
        assert_eq!(results[0].content_id, "spell-fireball");
        println!("Vector search successful. Distance: {}", results[0].distance);
        
        // Test via Storage trait
        use mimir_dm_core::Storage;
        let trait_results = storage.vector_search(&query_embedding, 1).await?;
        assert_eq!(trait_results.len(), 1);
        assert_eq!(trait_results[0].id, "spell-fireball");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_fts_search() -> anyhow::Result<()> {
        use mimir_dm_core::{Storage, FtsTable};
        
        let storage = setup_test_storage().await?;
        
        // Create some rules first
        let rule1 = NewRule {
            id: Uuid::new_v4().to_string(),
            title: "Fireball".to_string(),
            content: "A bright streak flashes from your pointing finger to a point you choose".to_string(),
            category: "spell".to_string(),
            source: Some("PHB".to_string()),
            page_reference: Some("241".to_string()),
        };
        
        let rule2 = NewRule {
            id: Uuid::new_v4().to_string(),
            title: "Magic Missile".to_string(),
            content: "You create three glowing darts of magical force".to_string(),
            category: "spell".to_string(),
            source: Some("PHB".to_string()),
            page_reference: Some("257".to_string()),
        };
        
        storage.create_rule(rule1).await?;
        storage.create_rule(rule2).await?;
        
        // Test FTS search
        let results = storage.fts_search("fireball", FtsTable::Rules).await?;
        assert!(!results.is_empty());
        println!("FTS search found {} results for 'fireball'", results.len());
        
        // Test search all
        let all_results = storage.fts_search_all("spell").await?;
        println!("FTS search all found {} results for 'spell'", all_results.len());
        
        Ok(())
    }
}