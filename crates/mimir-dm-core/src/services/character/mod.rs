//! Character management services

pub mod renderer;

pub use renderer::{CharacterRenderer, MarkdownRenderer};

use crate::{
    connection::DbConnection,
    dal::character::{CharacterRepository, CharacterVersionRepository},
    error::{DbError, Result},
    models::character::{Character, CharacterData, CharacterVersion, NewCharacter, NewCharacterVersion, UpdateCharacter},
};
use std::fs;
use std::path::{Path, PathBuf};

/// Service for character management operations
pub struct CharacterService<'a> {
    conn: &'a mut DbConnection,
}

impl<'a> CharacterService<'a> {
    /// Create a new character service
    pub fn new(conn: &'a mut DbConnection) -> Self {
        Self { conn }
    }

    /// Create a new character
    pub fn create_character(
        &mut self,
        campaign_id: i32,
        player_id: i32,
        campaign_directory: &str,
        character_data: CharacterData,
    ) -> Result<Character> {
        // Validate inputs
        if character_data.character_name.trim().is_empty() {
            return Err(DbError::InvalidData("Character name cannot be empty".to_string()));
        }

        // Create character directory
        let char_dir = self.create_character_directory(campaign_directory, &character_data.character_name)?;
        let directory_path = char_dir.to_string_lossy().to_string();

        // Create initial version
        let version_number = 1;
        let file_path = self.get_version_file_path(&char_dir, &character_data.character_name, version_number);

        // Serialize character data to YAML
        let yaml_data = serde_yaml::to_string(&character_data)
            .map_err(|e| DbError::InvalidData(format!("Failed to serialize character data: {}", e)))?;

        // Generate markdown
        let renderer = MarkdownRenderer::new();
        let markdown = renderer.render(&character_data);

        // Write files
        self.write_character_files(&file_path, &yaml_data, &markdown)?;

        // Create database record for character
        let mut char_repo = CharacterRepository::new(self.conn);
        let new_character = NewCharacter {
            campaign_id,
            player_id,
            character_name: character_data.character_name.clone(),
            is_npc: Some(0), // Default to PC
            directory_path,
        };

        let character = char_repo.create(new_character)?;

        // Create version record
        let mut ver_repo = CharacterVersionRepository::new(self.conn);
        let new_version = NewCharacterVersion {
            character_id: character.id,
            version_number,
            file_path: file_path.to_string_lossy().to_string(),
            character_data: yaml_data,
            snapshot_reason: character_data.snapshot_reason.clone(),
            level: character_data.level,
        };

        ver_repo.create(new_version)?;

        Ok(character)
    }

    /// Get a character by ID with its latest version data
    pub fn get_character(&mut self, character_id: i32) -> Result<(Character, CharacterData)> {
        let mut char_repo = CharacterRepository::new(self.conn);
        let character = char_repo.find_by_id(character_id)?
            .ok_or_else(|| DbError::NotFound {
                entity_type: "Character".to_string(),
                id: character_id.to_string(),
            })?;

        let mut ver_repo = CharacterVersionRepository::new(self.conn);
        let version = ver_repo.find_latest(character_id)?
            .ok_or_else(|| DbError::NotFound {
                entity_type: "CharacterVersion".to_string(),
                id: format!("character_id={}", character_id),
            })?;

        let character_data: CharacterData = serde_yaml::from_str(&version.character_data)
            .map_err(|e| DbError::InvalidData(format!("Failed to parse character data: {}", e)))?;

        Ok((character, character_data))
    }

    /// Update a character (creates a new version)
    pub fn update_character(
        &mut self,
        character_id: i32,
        character_data: CharacterData,
        snapshot_reason: Option<String>,
    ) -> Result<CharacterVersion> {
        // Get character and version number
        let character = {
            let mut char_repo = CharacterRepository::new(self.conn);
            char_repo.find_by_id(character_id)?
                .ok_or_else(|| DbError::NotFound {
                    entity_type: "Character".to_string(),
                    id: character_id.to_string(),
                })?
        };

        let version_number = {
            let mut ver_repo = CharacterVersionRepository::new(self.conn);
            ver_repo.get_next_version_number(character_id)?
        };

        // Get character directory
        let char_dir = Path::new(&character.directory_path);
        let file_path = self.get_version_file_path(char_dir, &character_data.character_name, version_number);

        // Serialize character data to YAML
        let yaml_data = serde_yaml::to_string(&character_data)
            .map_err(|e| DbError::InvalidData(format!("Failed to serialize character data: {}", e)))?;

        // Generate markdown
        let renderer = MarkdownRenderer::new();
        let markdown = renderer.render(&character_data);

        // Write files
        self.write_character_files(&file_path, &yaml_data, &markdown)?;

        // Create version record
        let new_version = NewCharacterVersion {
            character_id,
            version_number,
            file_path: file_path.to_string_lossy().to_string(),
            character_data: yaml_data,
            snapshot_reason,
            level: character_data.level,
        };

        let version = {
            let mut ver_repo = CharacterVersionRepository::new(self.conn);
            ver_repo.create(new_version)?
        };

        // Update character metadata
        let update = UpdateCharacter {
            character_name: None,
            is_npc: None,
            current_level: Some(character_data.level),
            current_version: Some(version_number),
            last_updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        let mut char_repo = CharacterRepository::new(self.conn);
        char_repo.update(character_id, update)?;

        Ok(version)
    }

    /// Delete a character and all its files
    pub fn delete_character(&mut self, character_id: i32) -> Result<()> {
        let mut char_repo = CharacterRepository::new(self.conn);
        let character = char_repo.find_by_id(character_id)?
            .ok_or_else(|| DbError::NotFound {
                entity_type: "Character".to_string(),
                id: character_id.to_string(),
            })?;

        // Delete directory and all files
        let char_dir = Path::new(&character.directory_path);
        if char_dir.exists() {
            fs::remove_dir_all(char_dir)?;
        }

        // Delete from database (cascades to character_versions)
        char_repo.delete(character_id)?;

        Ok(())
    }

    /// List all characters for a campaign
    pub fn list_characters_for_campaign(&mut self, campaign_id: i32) -> Result<Vec<Character>> {
        let mut char_repo = CharacterRepository::new(self.conn);
        char_repo.list_for_campaign(campaign_id)
    }

    /// Get all versions for a character
    pub fn get_character_versions(&mut self, character_id: i32) -> Result<Vec<CharacterVersion>> {
        let mut ver_repo = CharacterVersionRepository::new(self.conn);
        ver_repo.list_for_character(character_id)
    }

    /// Get a specific version of a character
    pub fn get_character_version(
        &mut self,
        character_id: i32,
        version_number: i32,
    ) -> Result<CharacterData> {
        let mut ver_repo = CharacterVersionRepository::new(self.conn);
        let version = ver_repo.find_by_character_and_version(character_id, version_number)?
            .ok_or_else(|| DbError::NotFound {
                entity_type: "CharacterVersion".to_string(),
                id: format!("character_id={}, version={}", character_id, version_number),
            })?;

        let character_data: CharacterData = serde_yaml::from_str(&version.character_data)
            .map_err(|e| DbError::InvalidData(format!("Failed to parse character data: {}", e)))?;

        Ok(character_data)
    }

    // Helper methods

    fn create_character_directory(&self, campaign_dir: &str, character_name: &str) -> Result<PathBuf> {
        let campaign_path = Path::new(campaign_dir);
        let characters_dir = campaign_path.join("characters");

        // Create characters directory if it doesn't exist
        if !characters_dir.exists() {
            fs::create_dir_all(&characters_dir)?;
        }

        // Create character-specific directory
        let char_dir = characters_dir.join(character_name);
        if !char_dir.exists() {
            fs::create_dir_all(&char_dir)?;
        }

        Ok(char_dir)
    }

    fn get_version_file_path(&self, char_dir: &Path, character_name: &str, version: i32) -> PathBuf {
        char_dir.join(format!("{}-{:03}.md", character_name, version))
    }

    fn write_character_files(&self, file_path: &Path, yaml_data: &str, markdown: &str) -> Result<()> {
        // Write YAML data as a comment at the top of the markdown file
        let full_content = format!("<!--\nCharacter Data (YAML):\n{}\n-->\n\n{}", yaml_data, markdown);

        fs::write(file_path, full_content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection;
    use crate::models::character::data::*;
    use diesel::prelude::*;
    use tempfile::TempDir;

    fn setup_test_db() -> DbConnection {
        let mut conn = establish_connection(":memory:").expect("Failed to create in-memory DB");
        crate::run_migrations(&mut conn).expect("Failed to run migrations");
        conn
    }

    fn create_test_campaign(conn: &mut DbConnection) -> i32 {
        diesel::insert_into(crate::schema::campaigns::table)
            .values((
                crate::schema::campaigns::name.eq("Test Campaign"),
                crate::schema::campaigns::status.eq("active"),
                crate::schema::campaigns::directory_path.eq("/test"),
            ))
            .returning(crate::models::campaign::Campaign::as_returning())
            .get_result(conn)
            .expect("Failed to create campaign")
            .id
    }

    fn create_test_player(conn: &mut DbConnection) -> i32 {
        diesel::insert_into(crate::schema::players::table)
            .values((
                crate::schema::players::name.eq("Test Player"),
            ))
            .returning(crate::models::player::Player::as_returning())
            .get_result(conn)
            .expect("Failed to create player")
            .id
    }

    fn create_test_character_data() -> CharacterData {
        CharacterData {
            character_name: "Test Character".to_string(),
            player_id: 1,
            level: 1,
            experience_points: 0,
            version: 1,
            snapshot_reason: Some("Initial creation".to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
            race: "Human".to_string(),
            subrace: None,
            class: "Fighter".to_string(),
            subclass: None,
            background: "Soldier".to_string(),
            alignment: Some("Lawful Good".to_string()),
            abilities: AbilityScores {
                strength: 16,
                dexterity: 14,
                constitution: 15,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
            },
            max_hp: 12,
            current_hp: 12,
            hit_dice_remaining: 1,
            hit_dice_type: "d10".to_string(),
            proficiencies: Proficiencies {
                skills: vec!["Athletics".to_string(), "Intimidation".to_string()],
                saves: vec!["Strength".to_string(), "Constitution".to_string()],
                armor: vec!["Light Armor".to_string(), "Medium Armor".to_string()],
                weapons: vec!["Simple Weapons".to_string(), "Martial Weapons".to_string()],
                tools: vec![],
                languages: vec!["Common".to_string()],
            },
            class_features: vec![],
            feats: vec![],
            spells: SpellData::default(),
            inventory: vec![],
            equipped: EquippedItems {
                armor: None,
                shield: None,
                main_hand: None,
                off_hand: None,
            },
            personality: Personality {
                traits: None,
                ideals: None,
                bonds: None,
                flaws: None,
            },
        }
    }

    #[test]
    fn test_create_character() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.player_id = player_id;

        let character = service
            .create_character(campaign_id, player_id, campaign_dir, character_data)
            .expect("Failed to create character");

        assert_eq!(character.character_name, "Test Character");
        assert_eq!(character.campaign_id, campaign_id);
        assert_eq!(character.player_id, player_id);
        assert_eq!(character.is_npc, 0);
        assert_eq!(character.current_level, 1);
        assert_eq!(character.current_version, 1);

        // Verify directory structure was created
        let char_dir = Path::new(campaign_dir).join("characters").join("Test Character");
        assert!(char_dir.exists());

        // Verify file was created
        let file_path = char_dir.join("Test Character-001.md");
        assert!(file_path.exists());

        // Verify file content contains both YAML and markdown
        let content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert!(content.contains("Character Data (YAML)"));
        assert!(content.contains("# Test Character"));
    }

    #[test]
    fn test_create_character_empty_name() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.character_name = "".to_string();
        character_data.player_id = player_id;

        let result = service.create_character(campaign_id, player_id, campaign_dir, character_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_character() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.player_id = player_id;

        let created = service
            .create_character(campaign_id, player_id, campaign_dir, character_data)
            .expect("Failed to create character");

        let (character, data) = service
            .get_character(created.id)
            .expect("Failed to get character");

        assert_eq!(character.id, created.id);
        assert_eq!(data.character_name, "Test Character");
        assert_eq!(data.level, 1);
        assert_eq!(data.race, "Human");
        assert_eq!(data.class, "Fighter");
    }

    #[test]
    fn test_get_character_not_found() {
        let mut conn = setup_test_db();
        let mut service = CharacterService::new(&mut conn);

        let result = service.get_character(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_character() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.player_id = player_id;

        let created = service
            .create_character(campaign_id, player_id, campaign_dir, character_data.clone())
            .expect("Failed to create character");

        // Update character (level up)
        character_data.level = 2;
        character_data.experience_points = 300;
        character_data.max_hp = 20;
        character_data.current_hp = 20;

        let version = service
            .update_character(created.id, character_data, Some("Level up to 2".to_string()))
            .expect("Failed to update character");

        assert_eq!(version.version_number, 2);
        assert_eq!(version.snapshot_reason, Some("Level up to 2".to_string()));
        assert_eq!(version.level, 2);

        // Verify character metadata was updated
        let (character, data) = service.get_character(created.id).expect("Failed to get character");
        assert_eq!(character.current_level, 2);
        assert_eq!(character.current_version, 2);
        assert_eq!(data.level, 2);
        assert_eq!(data.experience_points, 300);

        // Verify version 2 file was created
        let char_dir = Path::new(&character.directory_path);
        let file_path = char_dir.join("Test Character-002.md");
        assert!(file_path.exists());
    }

    #[test]
    fn test_delete_character() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.player_id = player_id;

        let created = service
            .create_character(campaign_id, player_id, campaign_dir, character_data)
            .expect("Failed to create character");

        let char_dir = Path::new(&created.directory_path);
        assert!(char_dir.exists());

        service.delete_character(created.id).expect("Failed to delete character");

        // Verify directory was removed
        assert!(!char_dir.exists());

        // Verify database record was removed
        let result = service.get_character(created.id);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_characters_for_campaign() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);

        // Create multiple characters
        let mut char1 = create_test_character_data();
        char1.character_name = "Character 1".to_string();
        char1.player_id = player_id;
        service.create_character(campaign_id, player_id, campaign_dir, char1).unwrap();

        let mut char2 = create_test_character_data();
        char2.character_name = "Character 2".to_string();
        char2.player_id = player_id;
        service.create_character(campaign_id, player_id, campaign_dir, char2).unwrap();

        let characters = service
            .list_characters_for_campaign(campaign_id)
            .expect("Failed to list characters");

        assert_eq!(characters.len(), 2);
    }

    #[test]
    fn test_get_character_versions() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.player_id = player_id;

        let created = service
            .create_character(campaign_id, player_id, campaign_dir, character_data.clone())
            .expect("Failed to create character");

        // Create a second version
        character_data.level = 2;
        service
            .update_character(created.id, character_data, Some("Level 2".to_string()))
            .unwrap();

        let versions = service
            .get_character_versions(created.id)
            .expect("Failed to get versions");

        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version_number, 1);
        assert_eq!(versions[1].version_number, 2);
    }

    #[test]
    fn test_get_character_version() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.player_id = player_id;

        let created = service
            .create_character(campaign_id, player_id, campaign_dir, character_data.clone())
            .expect("Failed to create character");

        // Create a second version
        character_data.level = 2;
        service
            .update_character(created.id, character_data, Some("Level 2".to_string()))
            .unwrap();

        // Get version 1
        let v1_data = service
            .get_character_version(created.id, 1)
            .expect("Failed to get version 1");
        assert_eq!(v1_data.level, 1);

        // Get version 2
        let v2_data = service
            .get_character_version(created.id, 2)
            .expect("Failed to get version 2");
        assert_eq!(v2_data.level, 2);
    }

    #[test]
    fn test_get_character_version_not_found() {
        let mut conn = setup_test_db();
        let campaign_id = create_test_campaign(&mut conn);
        let player_id = create_test_player(&mut conn);

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let campaign_dir = temp_dir.path().to_str().unwrap();

        let mut service = CharacterService::new(&mut conn);
        let mut character_data = create_test_character_data();
        character_data.player_id = player_id;

        let created = service
            .create_character(campaign_id, player_id, campaign_dir, character_data)
            .expect("Failed to create character");

        // Try to get non-existent version
        let result = service.get_character_version(created.id, 999);
        assert!(result.is_err());
    }
}
