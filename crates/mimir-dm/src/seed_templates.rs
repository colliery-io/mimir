//! Template seeding functionality - wrapper around mimir-dm-core's template seeder

use anyhow::Result;
use mimir_dm_core::DbConnection;
use tracing::info;

/// Seed the database with initial templates if they don't exist
pub fn seed_templates(conn: &mut DbConnection) -> Result<()> {
    // Use the seeder from mimir-dm-core
    match mimir_dm_core::seed::template_seeder::seed_templates(conn) {
        Ok(count) => {
            if count > 0 {
                info!("Successfully seeded {} templates", count);
            } else {
                info!("Templates already seeded");
            }
            Ok(())
        }
        Err(e) => match e {
            diesel::result::Error::QueryBuilderError(ref err) => {
                anyhow::bail!("Template file error: {}", err);
            }
            diesel::result::Error::DatabaseError(kind, info) => {
                anyhow::bail!("Database error during seeding: {:?} - {:?}", kind, info);
            }
            _ => {
                anyhow::bail!("Failed to seed templates: {}", e);
            }
        },
    }
}
