//! Character management commands

use tauri::State;
use mimir_dm_core::models::character::{Character, CharacterData, CharacterVersion};
use mimir_dm_core::models::character::data::ClassLevel;
use mimir_dm_core::models::character::data::{InventoryItem, Personality};
use mimir_dm_core::services::CharacterService;
use mimir_dm_core::services::character::creation::{CharacterBuilder, AbilityScoreMethod};
use mimir_dm_core::services::character::level_up::{AsiOrFeat, LevelUpOptions, HpGainMethod};
use mimir_dm_core::services::character::spell_management::RestType;
use mimir_dm_core::services::character::renderer::{MarkdownRenderer, CharacterRenderer};
use crate::state::AppState;
use mimir_dm_core::services::{SpellService, ItemService};
use mimir_dm_core::models::catalog::Spell;
use std::collections::HashMap;
use tracing::error;
use serde::{Deserialize, Serialize};

// Frontend-friendly types for JSON serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterRequest {
    pub character_name: String,
    pub player_id: i32,
    pub race: String,
    pub race_source: String,
    pub subrace: Option<String>,
    pub class: String,
    pub class_source: String,
    pub subclass: Option<String>,
    pub background: String,
    pub background_source: String,
    pub ability_score_method: String, // "standard_array", "point_buy", or "manual"
    pub ability_scores: Option<AbilityScoresInput>,
    pub alignment: Option<String>,
    pub personality: Option<PersonalityInput>,
    pub skill_proficiencies: Option<Vec<String>>,
    pub equipment: Option<Vec<InventoryItemInput>>,
    pub cantrips: Option<Vec<String>>,
    pub known_spells: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityScoresInput {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalityInput {
    pub traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItemInput {
    pub name: String,
    pub source: Option<String>,
    pub quantity: i32,
    pub weight: f64,
    pub value: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelUpRequest {
    pub class_name: String,
    pub class_source: String,
    pub hit_points_roll: Option<i32>,
    pub take_average_hp: bool,
    pub subclass: Option<String>,
    pub ability_score_improvement: Option<String>, // JSON string with ASI data
    pub feat: Option<String>,
    pub new_spell_slots: Option<String>, // JSON string with spell slot updates
    pub new_known_spells: Option<Vec<String>>, // Updated known spells list
    pub new_cantrips: Option<Vec<String>>, // Updated cantrips list
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyUpdate {
    pub copper: Option<i32>,
    pub silver: Option<i32>,
    pub electrum: Option<i32>,
    pub gold: Option<i32>,
    pub platinum: Option<i32>,
}

/// Create a minimal character for MVP (placeholder until full wizard is implemented)
#[tauri::command]
pub async fn create_minimal_character(
    player_id: i32,
    character_name: String,
    race: String,
    class: String,
    background: String,
    campaign_id: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Character, String> {
    use chrono::Utc;
    use mimir_dm_core::models::character::data::{CharacterData, AbilityScores, Proficiencies, SpellData, Currency, Personality, EquippedItems};

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    // Create minimal character data with placeholder values
    let character_data = CharacterData {
        character_name: character_name.clone(),
        player_id,
        level: 1,
        experience_points: 0,
        version: 1,
        snapshot_reason: Some("Initial character creation".to_string()),
        created_at: Utc::now().to_rfc3339(),
        race,
        subrace: None,
        classes: vec![ClassLevel {
            class_name: class,
            level: 1,
            subclass: None,
            hit_dice_type: "d8".to_string(),
            hit_dice_remaining: 1,
        }],
        background,
        alignment: None,
        abilities: AbilityScores {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        },
        max_hp: 10,
        current_hp: 10,
        speed: 30,
        proficiencies: Proficiencies {
            skills: vec![],
            saves: vec![],
            armor: vec![],
            weapons: vec![],
            tools: vec![],
            languages: vec![],
        },
        class_features: vec![],
        feats: vec![],
        spells: SpellData::default(),
        inventory: vec![],
        currency: Currency::default(),
        equipped: EquippedItems::default(),
        personality: Personality::default(),
    };

    let mut char_service = CharacterService::new(&mut conn);
    char_service.create_character(campaign_id, player_id, "", character_data)
        .map_err(|e| format!("Failed to create character: {}", e))
}

/// Create a new character with full builder pattern
#[tauri::command]
pub async fn create_character(
    request: CreateCharacterRequest,
    state: State<'_, AppState>,
) -> Result<CharacterData, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut builder = CharacterBuilder::new(&mut conn);

    // Set identity
    builder = builder.set_identity(request.character_name, request.player_id);

    // Set race
    builder = builder.set_race(&request.race, &request.race_source, request.subrace)
        .map_err(|e| format!("Failed to set race: {}", e))?;

    // Set class
    builder = builder.set_class(&request.class, &request.class_source, request.subclass)
        .map_err(|e| format!("Failed to set class: {}", e))?;

    // Set background
    builder = builder.set_background(&request.background, &request.background_source)
        .map_err(|e| format!("Failed to set background: {}", e))?;

    // Set ability scores - both StandardArray and PointBuy require specifying assignment
    let ability_scores = request.ability_scores.as_ref()
        .ok_or_else(|| "Ability scores must be specified".to_string())?;

    let ability_method = match request.ability_score_method.as_str() {
        "standard_array" => AbilityScoreMethod::StandardArray {
            strength: ability_scores.strength,
            dexterity: ability_scores.dexterity,
            constitution: ability_scores.constitution,
            intelligence: ability_scores.intelligence,
            wisdom: ability_scores.wisdom,
            charisma: ability_scores.charisma,
        },
        "point_buy" => AbilityScoreMethod::PointBuy {
            strength: ability_scores.strength,
            dexterity: ability_scores.dexterity,
            constitution: ability_scores.constitution,
            intelligence: ability_scores.intelligence,
            wisdom: ability_scores.wisdom,
            charisma: ability_scores.charisma,
        },
        "manual" => AbilityScoreMethod::Manual {
            strength: ability_scores.strength,
            dexterity: ability_scores.dexterity,
            constitution: ability_scores.constitution,
            intelligence: ability_scores.intelligence,
            wisdom: ability_scores.wisdom,
            charisma: ability_scores.charisma,
        },
        _ => return Err(format!("Invalid ability score method: {}", request.ability_score_method)),
    };
    builder = builder.set_ability_scores(ability_method)
        .map_err(|e| format!("Failed to set ability scores: {}", e))?;

    // Set optional fields
    if let Some(alignment) = request.alignment {
        builder = builder.set_alignment(alignment);
    }

    if let Some(personality) = request.personality {
        builder = builder.set_personality(Personality {
            traits: personality.traits,
            ideals: personality.ideals,
            bonds: personality.bonds,
            flaws: personality.flaws,
        });
    }

    if let Some(skills) = request.skill_proficiencies {
        for skill in skills {
            builder = builder.add_skill_proficiency(skill);
        }
    }

    if let Some(equipment) = request.equipment {
        for item in equipment {
            builder = builder.add_equipment(InventoryItem {
                name: item.name,
                source: item.source,
                quantity: item.quantity,
                weight: item.weight,
                value: item.value,
                notes: item.notes,
            });
        }
    }

    // Build and validate
    let mut character_data = builder.build()
        .map_err(|e| format!("Failed to build character: {}", e))?;

    // Set spells if provided
    if let Some(cantrips) = request.cantrips {
        character_data.spells.cantrips = cantrips;
    }
    if let Some(known_spells) = request.known_spells {
        character_data.spells.known_spells = known_spells;
    }

    // Persist to database using CharacterService
    let mut char_service = CharacterService::new(&mut conn);
    char_service.create_character(
        None, // campaign_id - not assigned yet
        request.player_id,
        "", // base_directory - empty for unassigned characters
        character_data.clone(),
    )
    .map_err(|e| format!("Failed to save character: {}", e))?;

    Ok(character_data)
}

/// Store a created character in the database
#[tauri::command]
pub async fn store_character(
    campaign_id: Option<i32>,
    player_id: i32,
    base_directory: Option<String>,
    character_data: CharacterData,
    state: State<'_, AppState>,
) -> Result<Character, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    // Use empty string if no directory provided (for unassigned characters)
    let directory = base_directory.unwrap_or_default();

    let mut char_service = CharacterService::new(&mut conn);
    char_service.create_character(campaign_id, player_id, &directory, character_data)
        .map_err(|e| format!("Failed to store character: {}", e))
}

/// Get character by ID
#[tauri::command]
pub async fn get_character(
    character_id: i32,
    state: State<'_, AppState>,
) -> Result<(Character, CharacterData), String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.get_character(character_id)
        .map_err(|e| format!("Failed to get character: {}", e))
}

/// Get spell slots for a character based on class rules
#[tauri::command]
pub async fn get_character_spell_slots(
    character_id: i32,
    state: State<'_, AppState>,
) -> Result<std::collections::HashMap<i32, mimir_dm_core::models::character::data::SpellSlots>, String> {
    use mimir_dm_core::services::character::calculate_spell_slots;

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    // Get character data
    let mut char_service = CharacterService::new(&mut conn);
    let (_, char_data) = char_service.get_character(character_id)
        .map_err(|e| format!("Failed to get character: {}", e))?;

    // Calculate spell slots from class rules
    calculate_spell_slots(&mut conn, &char_data)
        .map_err(|e| format!("Failed to calculate spell slots: {}", e))
}

/// List all characters (including unassigned)
#[tauri::command]
pub async fn list_all_characters(
    state: State<'_, AppState>,
) -> Result<Vec<Character>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.list_all_characters()
        .map_err(|e| format!("Failed to list characters: {}", e))
}

/// List all characters for a campaign
#[tauri::command]
pub async fn list_characters_for_campaign(
    campaign_id: i32,
    state: State<'_, AppState>,
) -> Result<Vec<Character>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.list_characters_for_campaign(campaign_id)
        .map_err(|e| format!("Failed to list characters: {}", e))
}

/// Get all versions of a character
#[tauri::command]
pub async fn get_character_versions(
    character_id: i32,
    state: State<'_, AppState>,
) -> Result<Vec<CharacterVersion>, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.get_character_versions(character_id)
        .map_err(|e| format!("Failed to get character versions: {}", e))
}

/// Get a specific character version
#[tauri::command]
pub async fn get_character_version(
    character_id: i32,
    version: i32,
    state: State<'_, AppState>,
) -> Result<CharacterData, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.get_character_version(character_id, version)
        .map_err(|e| format!("Failed to get character version: {}", e))
}

/// Update character data directly
#[tauri::command]
pub async fn update_character(
    character_id: i32,
    character_data: CharacterData,
    snapshot_reason: Option<String>,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.update_character(character_id, character_data, snapshot_reason)
        .map_err(|e| format!("Failed to update character: {}", e))
}

/// Delete a character
#[tauri::command]
pub async fn delete_character(
    character_id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.delete_character(character_id)
        .map_err(|e| format!("Failed to delete character: {}", e))
}

/// Assign a character to a campaign
#[tauri::command]
pub async fn assign_character_to_campaign(
    character_id: i32,
    campaign_id: i32,
    state: State<'_, AppState>,
) -> Result<Character, String> {
    use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;

    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    // Get campaign directory
    let campaign_directory = {
        let mut campaign_repo = CampaignRepository::new(&mut conn);
        let campaign = campaign_repo.find_by_id(campaign_id)
            .map_err(|e| format!("Failed to find campaign: {}", e))?
            .ok_or_else(|| format!("Campaign with id {} not found", campaign_id))?;
        campaign.directory_path
    };

    let mut char_service = CharacterService::new(&mut conn);
    char_service.assign_to_campaign(character_id, campaign_id, &campaign_directory)
        .map_err(|e| format!("Failed to assign character to campaign: {}", e))
}

/// Level up a character
#[tauri::command]
pub async fn level_up_character(
    character_id: i32,
    request: LevelUpRequest,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    // Parse ability score improvement if provided
    let asi_or_feat = if let Some(asi_json) = request.ability_score_improvement {
        Some(serde_json::from_str::<AsiOrFeat>(&asi_json)
            .map_err(|e| format!("Invalid ability score improvement: {}", e))?)
    } else if let Some(feat_name) = request.feat {
        Some(AsiOrFeat::Feat(feat_name))
    } else {
        None
    };

    // Determine HP gain method
    let hp_method = if request.take_average_hp {
        HpGainMethod::Average
    } else if let Some(roll) = request.hit_points_roll {
        HpGainMethod::Roll(roll)
    } else {
        HpGainMethod::Average // default to average if not specified
    };

    // Build level up options
    let options = LevelUpOptions {
        class_name: request.class_name,
        class_source: request.class_source,
        hp_method,
        asi_or_feat,
        subclass_choice: request.subclass,
        snapshot_reason: None,
    };

    let mut char_service = CharacterService::new(&mut conn);
    let result = char_service.level_up_character(character_id, options)
        .map_err(|e| format!("Failed to level up character: {}", e))?;

    // Update spells if provided
    if request.new_known_spells.is_some() || request.new_cantrips.is_some() {
        // Get current character data
        let (_, mut char_data) = char_service.get_character(character_id)
            .map_err(|e| format!("Failed to get character for spell update: {}", e))?;

        // Update cantrips if provided
        if let Some(cantrips) = request.new_cantrips {
            char_data.spells.cantrips = cantrips;
        }

        // Update known spells if provided
        if let Some(known) = request.new_known_spells {
            char_data.spells.known_spells = known;
        }

        // Save the updated character
        char_service.update_character(character_id, char_data, Some("Spell selection".to_string()))
            .map_err(|e| format!("Failed to update spells: {}", e))?;
    }

    Ok(result)
}

/// Add a spell to known spells
#[tauri::command]
pub async fn add_spell_to_known(
    character_id: i32,
    spell_name: String,
    spell_source: String,
    is_cantrip: bool,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.add_spell_to_known(character_id, &spell_name, &spell_source, is_cantrip)
        .map_err(|e| format!("Failed to add spell: {}", e))
}

/// Prepare spells for the day
#[tauri::command]
pub async fn prepare_spells(
    character_id: i32,
    spell_names: Vec<String>,
    spellcasting_ability: String,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.prepare_spells(character_id, spell_names, &spellcasting_ability)
        .map_err(|e| format!("Failed to prepare spells: {}", e))
}

/// Cast a spell (expends spell slot)
#[tauri::command]
pub async fn cast_spell(
    character_id: i32,
    spell_name: String,
    spell_level: i32,
    is_ritual: bool,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.cast_spell(character_id, &spell_name, spell_level, is_ritual)
        .map_err(|e| format!("Failed to cast spell: {}", e))
}

/// Take a rest to recover resources
#[tauri::command]
pub async fn take_rest(
    character_id: i32,
    rest_type: String, // "short" or "long"
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let rest = match rest_type.as_str() {
        "short" => RestType::Short,
        "long" => RestType::Long,
        _ => return Err(format!("Invalid rest type: {}", rest_type)),
    };

    let mut char_service = CharacterService::new(&mut conn);
    char_service.rest(character_id, rest)
        .map_err(|e| format!("Failed to rest: {}", e))
}

/// Add an item to character inventory
#[tauri::command]
pub async fn add_item_to_inventory(
    character_id: i32,
    item_name: String,
    item_source: String,
    quantity: i32,
    notes: Option<String>,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.add_item(character_id, &item_name, &item_source, quantity, notes)
        .map_err(|e| format!("Failed to add item: {}", e))
}

/// Remove an item from character inventory
#[tauri::command]
pub async fn remove_item_from_inventory(
    character_id: i32,
    item_name: String,
    quantity: i32,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.remove_item(character_id, &item_name, quantity)
        .map_err(|e| format!("Failed to remove item: {}", e))
}

/// Update character currency
#[tauri::command]
pub async fn update_character_currency(
    character_id: i32,
    currency: CurrencyUpdate,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.update_currency(
        character_id,
        currency.copper.unwrap_or(0),
        currency.silver.unwrap_or(0),
        currency.electrum.unwrap_or(0),
        currency.gold.unwrap_or(0),
        currency.platinum.unwrap_or(0),
    ).map_err(|e| format!("Failed to update currency: {}", e))
}

/// Update character equipped items
#[tauri::command]
pub async fn update_character_equipped(
    character_id: i32,
    armor: Option<String>,
    shield: Option<String>,
    main_hand: Option<String>,
    off_hand: Option<String>,
    state: State<'_, AppState>,
) -> Result<CharacterVersion, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    char_service.update_equipped(character_id, armor, shield, main_hand, off_hand)
        .map_err(|e| format!("Failed to update equipped items: {}", e))
}

/// Render character sheet as markdown
#[tauri::command]
pub async fn render_character_sheet(
    character_id: i32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut conn = state.db.get_connection().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        format!("Database connection failed: {}", e)
    })?;

    let mut char_service = CharacterService::new(&mut conn);
    let (_character, char_data) = char_service.get_character(character_id)
        .map_err(|e| format!("Failed to get character: {}", e))?;

    // Fetch spell details for all character spells
    let mut spell_details: HashMap<String, Spell> = HashMap::new();

    // Collect all spell names
    let mut all_spell_names = Vec::new();
    all_spell_names.extend(char_data.spells.cantrips.iter().cloned());
    all_spell_names.extend(char_data.spells.known_spells.iter().cloned());

    // Fetch details for each spell from catalog
    for spell_name in all_spell_names {
        // Try to find the spell in the catalog (search by name, use first match)
        use mimir_dm_core::models::catalog::SpellFilters;
        let filters = SpellFilters {
            query: Some(spell_name.clone()),
            levels: vec![],
            schools: vec![],
            sources: vec![],
            tags: vec![],
            limit: Some(1),
            offset: None,
        };

        // First search to find the spell's source
        if let Ok(summaries) = SpellService::search_spells(&mut conn, filters) {
            if let Some(summary) = summaries.first() {
                // Now get full details with the correct source
                if let Ok(Some(spell)) = SpellService::get_spell_details(
                    &mut conn,
                    &summary.name,
                    &summary.source,
                ) {
                    spell_details.insert(spell_name, spell);
                }
            }
        }
    }

    // Fetch item details for all inventory items
    use mimir_dm_core::models::catalog::Item;
    let mut item_details: HashMap<String, Item> = HashMap::new();
    let mut item_service = ItemService::new(&mut conn);

    for item in &char_data.inventory {
        let source = item.source.as_deref().unwrap_or("PHB");
        let key = format!("{}:{}", item.name, source);

        if let Ok(Some(details)) = item_service.get_item_by_name_and_source(&item.name, source) {
            item_details.insert(key, details);
        }
    }

    let renderer = MarkdownRenderer::new();
    Ok(renderer.render_with_details(&char_data, &spell_details, &item_details))
}

/// Write text to a file
#[tauri::command]
pub async fn write_text_file(
    path: String,
    contents: String,
) -> Result<(), String> {
    std::fs::write(&path, contents)
        .map_err(|e| format!("Failed to write file: {}", e))
}
