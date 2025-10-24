---
id: character-models-dal
level: task
title: "Character Models & DAL"
created_at: 2025-08-17T02:39:00.000000+00:00
updated_at: 2025-08-17T02:39:00.000000+00:00
parent: character-creation-integration
blocked_by: ["character-database-schema"]
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
---

# Character Models & DAL

## Overview
Build Rust models and Data Access Layer (DAL) for character management, following the existing patterns in mimir-dm-core. This provides type-safe access to character data throughout the application.

## Acceptance Criteria

- [ ] PlayerCharacter model with all fields
- [ ] PlayerCharacterClass model for multiclassing
- [ ] PlayerCharacterInventory model
- [ ] PlayerCharacterSpell model
- [ ] CharacterState struct for JSON data
- [ ] CharacterRepository with CRUD operations
- [ ] Integration with existing DAL traits
- [ ] Comprehensive unit tests

## Model Definitions

### Core Models
```rust
// models/characters/player_character.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub id: i32,
    pub campaign_id: Option<i32>,
    pub rule_system_id: String,
    pub name: String,
    pub player_name: Option<String>,
    pub race_id: Option<String>,
    pub background_id: Option<String>,
    pub alignment: Option<String>,
    pub experience_points: i32,
    pub level: i32,
    pub hit_points_max: Option<i32>,
    pub armor_class: Option<i32>,
    pub initiative_bonus: Option<i32>,
    pub speed: i32,
    pub ability_scores: String, // JSON
    pub character_state: String, // JSON
    pub notes: Option<String>,
    pub backstory: Option<String>,
    pub appearance: Option<String>,
    pub personality_traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Character state for JSON field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterState {
    pub ability_scores: AbilityScores,
    pub proficiencies: Proficiencies,
    pub features: Features,
    pub spell_casting: Option<SpellCasting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityScores {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}
```

### Repository Implementation
```rust
// dal/characters/player_characters.rs
pub struct PlayerCharacterRepository {
    db_url: String,
}

impl PlayerCharacterRepository {
    pub fn new(db_url: String) -> Self {
        Self { db_url }
    }
    
    pub async fn create(&self, character: NewPlayerCharacter) -> Result<PlayerCharacter> {
        // Implementation
    }
    
    pub async fn get_by_id(&self, id: i32) -> Result<Option<PlayerCharacter>> {
        // Implementation
    }
    
    pub async fn get_by_campaign(&self, campaign_id: i32) -> Result<Vec<PlayerCharacter>> {
        // Implementation
    }
    
    pub async fn update(&self, id: i32, character: UpdatePlayerCharacter) -> Result<PlayerCharacter> {
        // Implementation
    }
    
    pub async fn delete(&self, id: i32) -> Result<()> {
        // Implementation
    }
    
    // Character-specific methods
    pub async fn add_class(&self, character_id: i32, class: PlayerCharacterClass) -> Result<()> {
        // Implementation
    }
    
    pub async fn add_item(&self, character_id: i32, item: PlayerCharacterInventoryItem) -> Result<()> {
        // Implementation
    }
    
    pub async fn prepare_spells(&self, character_id: i32, spell_ids: Vec<String>) -> Result<()> {
        // Implementation
    }
}
```

## Implementation Steps

1. **Create model modules**
   - Add `models/characters/mod.rs`
   - Define all character-related models
   - Implement serialization/deserialization

2. **Create DAL modules**
   - Add `dal/characters/mod.rs`
   - Implement repository pattern
   - Follow existing AsyncRepository trait

3. **Add builder patterns**
   - NewPlayerCharacter for creation
   - UpdatePlayerCharacter for updates
   - Validation helpers

4. **Integrate with existing system**
   - Export from mimir-dm-core
   - Update mod.rs files
   - Ensure consistency with other DAL

5. **Add convenience methods**
   - Character sheet generation helpers
   - Party composition queries
   - Level progression helpers

## Testing Requirements

- [ ] Unit tests for all models
- [ ] Repository CRUD tests
- [ ] JSON serialization tests
- [ ] Integration tests with real database
- [ ] Error handling tests

## Dependencies
- Database schema must exist
- Diesel ORM setup
- Existing DAL traits

## Estimated Effort
2-3 days

## Notes
- Follow existing patterns from rules DAL
- Consider performance for party queries
- Prepare for future character builder UI