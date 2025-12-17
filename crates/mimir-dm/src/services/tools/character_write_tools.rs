//! Write-capable character tools for LLM interactions
//!
//! These tools allow LLMs to modify character data with user confirmation

use async_trait::async_trait;
use mimir_dm_core::dal::campaign::campaigns::CampaignRepository;
use mimir_dm_core::models::character::data::{InventoryItem, Personality};
use mimir_dm_core::services::character::creation::{AbilityScoreMethod, CharacterBuilder};
use mimir_dm_core::services::character::spell_management::RestType;
use mimir_dm_core::{services::CharacterService, DatabaseService};
use mimir_dm_llm::traits::{ActionDescription, ChangeDetail};
use mimir_dm_llm::ToolTrait;
use serde_json::{json, Value};
use std::error::Error;
use std::sync::Arc;
use tracing::debug;

/// Tool for updating character HP (damage/healing)
pub struct UpdateCharacterHpTool {
    db_service: Arc<DatabaseService>,
}

impl UpdateCharacterHpTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for UpdateCharacterHpTool {
    fn name(&self) -> &str {
        "update_character_hp"
    }

    fn description(&self) -> &str {
        "Update a character's current HP (apply damage or healing).

Usage:
- Provide character_id and new_hp value
- Optionally provide reason for HP change
- Creates new character version snapshot
- Respects max HP limits

When to use:
- After combat encounters
- When healing spells or potions are used
- Recording damage during session
- Tracking character health status

Output:
- Updated character with new HP value
- Character version created for history"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character to update"
                },
                "new_hp": {
                    "type": "integer",
                    "description": "New current HP value (0 to max_hp)"
                },
                "reason": {
                    "type": ["string", "null"],
                    "description": "Reason for HP change (e.g., 'Took 10 damage from goblin', 'Healed 8 HP from potion')"
                }
            },
            "required": ["character_id", "new_hp"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;
        let new_hp = arguments.get("new_hp")?.as_i64()?;
        let reason = arguments
            .get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("HP updated");

        // Try to get current character data for comparison
        let current_hp_info = if let Ok(mut conn) = self.db_service.get_connection() {
            let mut char_service = CharacterService::new(&mut conn);
            if let Ok((_, char_data)) = char_service.get_character(character_id as i32) {
                Some((
                    char_data.character_name.clone(),
                    char_data.current_hp,
                    char_data.max_hp,
                ))
            } else {
                None
            }
        } else {
            None
        };

        let description = if let Some((name, current, max)) = current_hp_info {
            let change = new_hp as i32 - current;
            let change_desc = if change > 0 {
                format!("heal {} HP", change)
            } else if change < 0 {
                format!("take {} damage", -change)
            } else {
                "no change".to_string()
            };

            format!(
                "Update {}'s HP from {}/{} to {}/{} ({})\nReason: {}",
                name, current, max, new_hp, max, change_desc, reason
            )
        } else {
            format!(
                "Update character {} HP to {}\nReason: {}",
                character_id, new_hp, reason
            )
        };

        Some(ActionDescription {
            title: "Update Character HP".to_string(),
            description,
            changes: ChangeDetail::Generic {
                items: vec![
                    format!("Character ID: {}", character_id),
                    format!("New HP: {}", new_hp),
                    format!("Reason: {}", reason),
                ],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments
            .get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'character_id' parameter")? as i32;

        let new_hp = arguments
            .get("new_hp")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'new_hp' parameter")? as i32;

        let reason = arguments
            .get("reason")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut conn = self
            .db_service
            .get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_, mut char_data) = char_service
            .get_character(character_id)
            .map_err(|e| format!("Failed to retrieve character: {}", e))?;

        let old_hp = char_data.current_hp;
        char_data.current_hp = new_hp.max(0).min(char_data.max_hp);

        let snapshot_reason = reason
            .unwrap_or_else(|| format!("HP updated from {} to {}", old_hp, char_data.current_hp));

        char_service
            .update_character(character_id, char_data.clone(), Some(snapshot_reason))
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "old_hp": old_hp,
            "new_hp": char_data.current_hp,
            "max_hp": char_data.max_hp,
            "message": format!("Updated {} HP from {} to {}", char_data.character_name, old_hp, char_data.current_hp)
        });

        debug!(
            "Updated character {} HP: {} -> {}",
            character_id, old_hp, char_data.current_hp
        );
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// Tool for adding items to character inventory
pub struct AddInventoryItemTool {
    db_service: Arc<DatabaseService>,
}

impl AddInventoryItemTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for AddInventoryItemTool {
    fn name(&self) -> &str {
        "add_inventory_item"
    }

    fn description(&self) -> &str {
        "Add an item to a character's inventory.

Usage:
- Provide character_id, item name, and quantity
- Optionally provide weight, value, and notes
- Creates new character version snapshot

When to use:
- After looting enemies or treasure
- When characters purchase items
- Recording quest rewards
- Adding starting equipment

Output:
- Updated character with new inventory item
- Character version created for history"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character"
                },
                "item_name": {
                    "type": "string",
                    "description": "Name of the item to add"
                },
                "item_source": {
                    "type": ["string", "null"],
                    "description": "Source book of the item (e.g., PHB, DMG)"
                },
                "quantity": {
                    "type": ["integer", "null"],
                    "description": "Quantity to add (default: 1)"
                },
                "weight": {
                    "type": ["number", "null"],
                    "description": "Weight per item in pounds (optional)"
                },
                "value": {
                    "type": ["number", "null"],
                    "description": "Value per item in gold pieces (optional)"
                },
                "notes": {
                    "type": ["string", "null"],
                    "description": "Additional notes about the item (optional)"
                }
            },
            "required": ["character_id", "item_name"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;
        let item_name = arguments.get("item_name")?.as_str()?;
        let quantity = arguments
            .get("quantity")
            .and_then(|v| v.as_i64())
            .unwrap_or(1);
        let weight = arguments.get("weight").and_then(|v| v.as_f64());
        let value = arguments.get("value").and_then(|v| v.as_f64());

        let mut details = vec![
            format!("Character ID: {}", character_id),
            format!("Item: {}", item_name),
            format!("Quantity: {}", quantity),
        ];

        if let Some(w) = weight {
            details.push(format!("Weight: {} lb", w));
        }
        if let Some(v) = value {
            details.push(format!("Value: {} gp", v));
        }

        Some(ActionDescription {
            title: "Add Inventory Item".to_string(),
            description: format!("Add {} × {} to character inventory", quantity, item_name),
            changes: ChangeDetail::Generic { items: details },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments
            .get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'character_id' parameter")? as i32;

        let item_name = arguments
            .get("item_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'item_name' parameter")?;

        let item_source = arguments
            .get("item_source")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let quantity = arguments
            .get("quantity")
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        let weight = arguments
            .get("weight")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let value = arguments
            .get("value")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let notes = arguments
            .get("notes")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut conn = self
            .db_service
            .get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_, mut char_data) = char_service
            .get_character(character_id)
            .map_err(|e| format!("Failed to retrieve character: {}", e))?;

        // Check if item already exists in inventory
        if let Some(existing) = char_data.inventory.iter_mut().find(|i| i.name == item_name) {
            existing.quantity += quantity;
        } else {
            char_data.inventory.push(InventoryItem {
                name: item_name.to_string(),
                source: item_source,
                quantity,
                weight,
                value,
                notes,
            });
        }

        let snapshot_reason = format!("Added {} × {} to inventory", quantity, item_name);
        char_service
            .update_character(character_id, char_data.clone(), Some(snapshot_reason))
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "item_added": item_name,
            "quantity": quantity,
            "message": format!("Added {} × {} to {}'s inventory", quantity, item_name, char_data.character_name)
        });

        debug!(
            "Added item to character {}: {} × {}",
            character_id, quantity, item_name
        );
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// Tool for casting spells (consumes spell slots)
pub struct CastSpellTool {
    db_service: Arc<DatabaseService>,
}

impl CastSpellTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for CastSpellTool {
    fn name(&self) -> &str {
        "cast_spell"
    }

    fn description(&self) -> &str {
        "Cast a spell and consume the appropriate spell slot.

Usage:
- Provide character_id, spell_name, and spell_level
- Automatically reduces available spell slots
- Creates new character version snapshot
- Validates character has available slots

When to use:
- During combat when spells are cast
- Recording spell usage in sessions
- Tracking spell slot consumption
- Before rest/long rest recovery

Output:
- Updated character with reduced spell slots
- Character version created for history"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character casting the spell"
                },
                "spell_name": {
                    "type": "string",
                    "description": "Name of the spell being cast"
                },
                "spell_level": {
                    "type": "integer",
                    "description": "Spell level (1-9, use 0 for cantrips)"
                }
            },
            "required": ["character_id", "spell_name", "spell_level"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;
        let spell_name = arguments.get("spell_name")?.as_str()?;
        let spell_level = arguments.get("spell_level")?.as_i64()?;

        // Try to get current character data for slot availability
        let slot_info = if let Ok(mut conn) = self.db_service.get_connection() {
            let mut char_service = CharacterService::new(&mut conn);
            if let Ok((_, char_data)) = char_service.get_character(character_id as i32) {
                char_data
                    .spells
                    .spell_slots
                    .get(&(spell_level as i32))
                    .map(|slots| (char_data.character_name.clone(), slots.current, slots.max))
            } else {
                None
            }
        } else {
            None
        };

        let description = if spell_level == 0 {
            format!("Cast cantrip: {} (no slot required)", spell_name)
        } else if let Some((name, current, max)) = slot_info {
            format!(
                "{} casts {} (level {})\nCurrent level {} slots: {}/{}\nAfter cast: {}/{}",
                name,
                spell_name,
                spell_level,
                spell_level,
                current,
                max,
                current - 1,
                max
            )
        } else {
            format!(
                "Cast {} (level {}) - consumes 1 spell slot",
                spell_name, spell_level
            )
        };

        Some(ActionDescription {
            title: "Cast Spell".to_string(),
            description,
            changes: ChangeDetail::Generic {
                items: vec![
                    format!("Character ID: {}", character_id),
                    format!("Spell: {}", spell_name),
                    format!("Level: {}", spell_level),
                    if spell_level > 0 {
                        "Consumes 1 spell slot".to_string()
                    } else {
                        "Cantrip (no slot consumed)".to_string()
                    },
                ],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments
            .get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'character_id' parameter")? as i32;

        let spell_name = arguments
            .get("spell_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'spell_name' parameter")?;

        let spell_level = arguments
            .get("spell_level")
            .and_then(|v| v.as_i64())
            .ok_or("Missing 'spell_level' parameter")? as i32;

        if spell_level == 0 {
            return Ok(json!({
                "success": true,
                "message": format!("Cast cantrip {} (no slot consumed)", spell_name)
            })
            .to_string());
        }

        let mut conn = self
            .db_service
            .get_connection()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_, mut char_data) = char_service
            .get_character(character_id)
            .map_err(|e| format!("Failed to retrieve character: {}", e))?;

        // Check if character has spell slots for this level
        let slots = char_data
            .spells
            .spell_slots
            .get_mut(&spell_level)
            .ok_or(format!(
                "Character has no level {} spell slots",
                spell_level
            ))?;

        if slots.current <= 0 {
            return Err(format!(
                "No level {} spell slots remaining (0/{})",
                spell_level, slots.max
            )
            .into());
        }

        // Consume spell slot
        slots.current -= 1;

        // Capture values before the borrow ends
        let slots_remaining = slots.current;
        let slots_max = slots.max;

        let snapshot_reason = format!(
            "Cast {} (level {}) - {} slots remaining",
            spell_name, spell_level, slots_remaining
        );

        char_service
            .update_character(character_id, char_data.clone(), Some(snapshot_reason))
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "spell_cast": spell_name,
            "spell_level": spell_level,
            "slots_remaining": slots_remaining,
            "slots_max": slots_max,
            "message": format!(
                "{} cast {} (level {}). {} slots remaining.",
                char_data.character_name, spell_name, spell_level, slots_remaining
            )
        });

        debug!(
            "Character {} cast spell: {} (level {})",
            character_id, spell_name, spell_level
        );
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// Tool for creating a new character
pub struct CreateCharacterTool {
    db_service: Arc<DatabaseService>,
}

impl CreateCharacterTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for CreateCharacterTool {
    fn name(&self) -> &str {
        "create_character"
    }

    fn description(&self) -> &str {
        "Create a new D&D 5e character with full rule support.

IMPORTANT: Use this tool instead of templates/files when creating characters. This tool handles all D&D 5e rules automatically and stores characters in the database.

Usage:
- First use list_players to find the correct player_id
- Provide player_id, character_name, race, race_source, class, class_source, background, background_source
- Provide ability_scores as object with strength, dexterity, constitution, intelligence, wisdom, charisma
- Optionally provide campaign_id, subrace, subclass, alignment, personality traits
- Source is typically 'PHB' for Player's Handbook content
- If campaign_id is not provided, the character is created in the general character pool

When to use:
- When a player wants to create a new character
- Setting up characters for a new campaign
- Creating NPCs that need full character sheets
- Replacing a dead character mid-campaign

Character creation includes:
- Racial traits and ability bonuses applied automatically
- Class features and proficiencies from class/background
- Starting HP calculated from class hit dice + CON modifier
- Spell slots calculated for spellcasting classes
- Speed and other racial attributes

Output:
- Created character with database ID
- Character name, level, race, and class confirmed
- Character is immediately available for gameplay"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "player_id": {
                    "type": "integer",
                    "description": "ID of the player who owns this character"
                },
                "campaign_id": {
                    "type": ["integer", "null"],
                    "description": "Optional campaign ID to associate with this character (omit or null for general character pool)"
                },
                "character_name": {
                    "type": "string",
                    "description": "Name of the character"
                },
                "race": {
                    "type": "string",
                    "description": "Character race (e.g., Human, Elf, Dwarf)"
                },
                "race_source": {
                    "type": "string",
                    "description": "Source book for race (e.g., PHB)"
                },
                "subrace": {
                    "type": ["string", "null"],
                    "description": "Character subrace if applicable"
                },
                "class": {
                    "type": "string",
                    "description": "Character class (e.g., Fighter, Wizard)"
                },
                "class_source": {
                    "type": "string",
                    "description": "Source book for class (e.g., PHB)"
                },
                "subclass": {
                    "type": ["string", "null"],
                    "description": "Character subclass if applicable"
                },
                "background": {
                    "type": "string",
                    "description": "Character background (e.g., Soldier, Sage)"
                },
                "background_source": {
                    "type": "string",
                    "description": "Source book for background (e.g., PHB)"
                },
                "ability_scores": {
                    "type": "object",
                    "description": "Ability scores",
                    "properties": {
                        "strength": { "type": "integer" },
                        "dexterity": { "type": "integer" },
                        "constitution": { "type": "integer" },
                        "intelligence": { "type": "integer" },
                        "wisdom": { "type": "integer" },
                        "charisma": { "type": "integer" }
                    },
                    "required": ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"]
                },
                "alignment": {
                    "type": ["string", "null"],
                    "description": "Character alignment (e.g., Lawful Good)"
                },
                "personality": {
                    "type": ["object", "null"],
                    "description": "Personality traits",
                    "properties": {
                        "traits": { "type": ["string", "null"] },
                        "ideals": { "type": ["string", "null"] },
                        "bonds": { "type": ["string", "null"] },
                        "flaws": { "type": ["string", "null"] }
                    }
                }
            },
            "required": ["player_id", "character_name", "race", "race_source", "class", "class_source", "background", "background_source", "ability_scores"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_name = arguments.get("character_name")?.as_str()?;
        let race = arguments.get("race")?.as_str()?;
        let class = arguments.get("class")?.as_str()?;

        Some(ActionDescription {
            title: "Create Character".to_string(),
            description: format!(
                "Create new character: {} the {} {}",
                character_name, race, class
            ),
            changes: ChangeDetail::Generic {
                items: vec![
                    format!("Name: {}", character_name),
                    format!("Race: {}", race),
                    format!("Class: {}", class),
                ],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let player_id = arguments
            .get("player_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing player_id")? as i32;

        let campaign_id = arguments
            .get("campaign_id")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let character_name = arguments
            .get("character_name")
            .and_then(|v| v.as_str())
            .ok_or("Missing character_name")?
            .to_string();

        let race = arguments
            .get("race")
            .and_then(|v| v.as_str())
            .ok_or("Missing race")?
            .to_string();

        let race_source = arguments
            .get("race_source")
            .and_then(|v| v.as_str())
            .ok_or("Missing race_source")?
            .to_string();

        let subrace = arguments
            .get("subrace")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let class = arguments
            .get("class")
            .and_then(|v| v.as_str())
            .ok_or("Missing class")?
            .to_string();

        let class_source = arguments
            .get("class_source")
            .and_then(|v| v.as_str())
            .ok_or("Missing class_source")?
            .to_string();

        let subclass = arguments
            .get("subclass")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let background = arguments
            .get("background")
            .and_then(|v| v.as_str())
            .ok_or("Missing background")?
            .to_string();

        let background_source = arguments
            .get("background_source")
            .and_then(|v| v.as_str())
            .ok_or("Missing background_source")?
            .to_string();

        let ability_scores = arguments
            .get("ability_scores")
            .ok_or("Missing ability_scores")?;

        let alignment = arguments
            .get("alignment")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let personality = arguments.get("personality").map(|p| Personality {
            traits: p
                .get("traits")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            ideals: p
                .get("ideals")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            bonds: p
                .get("bonds")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            flaws: p
                .get("flaws")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        });

        let mut conn = self
            .db_service
            .get_connection()
            .map_err(|e| format!("Database error: {}", e))?;

        // Look up campaign directory if campaign_id is provided
        let base_directory = if let Some(cid) = campaign_id {
            let mut campaign_repo = CampaignRepository::new(&mut conn);
            let campaign = campaign_repo
                .find_by_id(cid)
                .map_err(|e| format!("Failed to find campaign: {}", e))?
                .ok_or_else(|| format!("Campaign with id {} not found", cid))?;
            campaign.directory_path
        } else {
            String::new()
        };

        // Build ability scores
        let scores = AbilityScoreMethod::Manual {
            strength: ability_scores
                .get("strength")
                .and_then(|v| v.as_i64())
                .unwrap_or(10) as i32,
            dexterity: ability_scores
                .get("dexterity")
                .and_then(|v| v.as_i64())
                .unwrap_or(10) as i32,
            constitution: ability_scores
                .get("constitution")
                .and_then(|v| v.as_i64())
                .unwrap_or(10) as i32,
            intelligence: ability_scores
                .get("intelligence")
                .and_then(|v| v.as_i64())
                .unwrap_or(10) as i32,
            wisdom: ability_scores
                .get("wisdom")
                .and_then(|v| v.as_i64())
                .unwrap_or(10) as i32,
            charisma: ability_scores
                .get("charisma")
                .and_then(|v| v.as_i64())
                .unwrap_or(10) as i32,
        };

        // Create character using builder
        let mut builder = CharacterBuilder::new(&mut conn)
            .set_identity(character_name.clone(), Some(player_id))
            .set_race(&race, &race_source, subrace)
            .map_err(|e| format!("Failed to set race: {}", e))?
            .set_class(&class, &class_source, subclass)
            .map_err(|e| format!("Failed to set class: {}", e))?
            .set_ability_scores(scores)
            .map_err(|e| format!("Failed to set ability scores: {}", e))?
            .set_background(&background, &background_source)
            .map_err(|e| format!("Failed to set background: {}", e))?;

        if let Some(align) = alignment {
            builder = builder.set_alignment(align);
        }

        if let Some(pers) = personality {
            builder = builder.set_personality(pers);
        }

        let char_data = builder
            .build()
            .map_err(|e| format!("Failed to create character: {}", e))?;

        // Store the character in the database
        let mut char_service = CharacterService::new(&mut conn);
        let character = char_service
            .create_character(campaign_id, Some(player_id), false, &base_directory, char_data.clone())
            .map_err(|e| format!("Failed to store character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character.id,
            "character_name": char_data.character_name,
            "level": char_data.level,
            "race": char_data.race,
            "class": char_data.classes[0].class_name,
            "message": format!("Created {} - Level {} {} {}",
                char_data.character_name,
                char_data.level,
                char_data.race,
                char_data.classes[0].class_name
            )
        });

        debug!(
            "Created character: {} (ID: {})",
            char_data.character_name, character.id
        );
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// Tool for updating character details
pub struct UpdateCharacterTool {
    db_service: Arc<DatabaseService>,
}

impl UpdateCharacterTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for UpdateCharacterTool {
    fn name(&self) -> &str {
        "update_character"
    }

    fn description(&self) -> &str {
        "Update character details like name, alignment, or personality traits.

Usage:
- Provide character_id (required)
- Provide any fields to update: character_name, alignment, personality
- Only provided fields are updated; others remain unchanged
- Personality is an object with: traits, ideals, bonds, flaws
- Creates version snapshot for history tracking

When to use:
- Character development or story changes (alignment shift)
- Correcting character information
- Adding or updating personality traits during roleplay
- Player requests to rename character
- Recording character growth from campaign events

NOT for:
- HP changes (use update_character_hp)
- Inventory changes (use add_inventory_item)
- Spell slot usage (use cast_spell)
- Level ups (handled through separate level up flow)

Output:
- Confirmation of updated character
- New version snapshot created
- Changes immediately reflected in character sheet"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character to update"
                },
                "character_name": {
                    "type": ["string", "null"],
                    "description": "New character name"
                },
                "alignment": {
                    "type": ["string", "null"],
                    "description": "New alignment"
                },
                "personality": {
                    "type": ["object", "null"],
                    "description": "Personality traits to update",
                    "properties": {
                        "traits": { "type": ["string", "null"] },
                        "ideals": { "type": ["string", "null"] },
                        "bonds": { "type": ["string", "null"] },
                        "flaws": { "type": ["string", "null"] }
                    }
                }
            },
            "required": ["character_id"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;

        let mut items = vec![format!("Character ID: {}", character_id)];

        if let Some(name) = arguments.get("character_name").and_then(|v| v.as_str()) {
            items.push(format!("New name: {}", name));
        }

        if let Some(alignment) = arguments.get("alignment").and_then(|v| v.as_str()) {
            items.push(format!("New alignment: {}", alignment));
        }

        if arguments.get("personality").is_some() {
            items.push("Personality traits updated".to_string());
        }

        Some(ActionDescription {
            title: "Update Character".to_string(),
            description: format!("Update character {} details", character_id),
            changes: ChangeDetail::Generic { items },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments
            .get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing character_id")? as i32;

        let mut conn = self
            .db_service
            .get_connection()
            .map_err(|e| format!("Database error: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_character, mut char_data) = char_service
            .get_character(character_id)
            .map_err(|e| format!("Character not found: {}", e))?;

        // Update fields if provided
        if let Some(name) = arguments.get("character_name").and_then(|v| v.as_str()) {
            char_data.character_name = name.to_string();
        }

        if let Some(alignment) = arguments.get("alignment").and_then(|v| v.as_str()) {
            char_data.alignment = Some(alignment.to_string());
        }

        if let Some(personality) = arguments.get("personality") {
            if let Some(traits) = personality.get("traits").and_then(|v| v.as_str()) {
                char_data.personality.traits = Some(traits.to_string());
            }
            if let Some(ideals) = personality.get("ideals").and_then(|v| v.as_str()) {
                char_data.personality.ideals = Some(ideals.to_string());
            }
            if let Some(bonds) = personality.get("bonds").and_then(|v| v.as_str()) {
                char_data.personality.bonds = Some(bonds.to_string());
            }
            if let Some(flaws) = personality.get("flaws").and_then(|v| v.as_str()) {
                char_data.personality.flaws = Some(flaws.to_string());
            }
        }

        char_service
            .update_character(
                character_id,
                char_data.clone(),
                Some("AI-assisted update".to_string()),
            )
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "message": format!("Updated character: {}", char_data.character_name)
        });

        debug!(
            "Updated character: {} (ID: {})",
            char_data.character_name, character_id
        );
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// Tool for taking a rest (short or long)
pub struct TakeRestTool {
    db_service: Arc<DatabaseService>,
}

impl TakeRestTool {
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self { db_service }
    }
}

#[async_trait]
impl ToolTrait for TakeRestTool {
    fn name(&self) -> &str {
        "take_rest"
    }

    fn description(&self) -> &str {
        "Have a character take a short or long rest to restore resources.

Usage:
- Provide character_id and rest_type ('short' or 'long')
- Creates version snapshot for history tracking

Short rest effects:
- Currently records the rest (hit dice spending can be done manually)
- Certain class features may restore on short rest

Long rest effects:
- Restores all HP to maximum
- Restores all hit dice
- Restores all spell slots to maximum
- Resets daily abilities

When to use:
- After combat encounters when party rests
- End of adventuring day (long rest)
- Mid-dungeon recovery (short rest)
- Before major encounters to ensure full resources
- Tracking passage of time in the campaign

Output:
- HP before and after rest
- Confirmation of rest completion
- All restored resources noted
- Version snapshot created for session history"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "character_id": {
                    "type": "integer",
                    "description": "ID of the character taking a rest"
                },
                "rest_type": {
                    "type": "string",
                    "enum": ["short", "long"],
                    "description": "Type of rest (short or long)"
                }
            },
            "required": ["character_id", "rest_type"]
        })
    }

    fn requires_confirmation(&self) -> bool {
        true
    }

    fn describe_action(&self, arguments: &Value) -> Option<ActionDescription> {
        let character_id = arguments.get("character_id")?.as_i64()?;
        let rest_type = arguments.get("rest_type")?.as_str()?;

        Some(ActionDescription {
            title: format!(
                "{} Rest",
                if rest_type == "long" { "Long" } else { "Short" }
            ),
            description: format!("Character {} takes a {} rest", character_id, rest_type),
            changes: ChangeDetail::Generic {
                items: vec![
                    format!("Character ID: {}", character_id),
                    format!("Rest type: {}", rest_type),
                ],
            },
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let character_id = arguments
            .get("character_id")
            .and_then(|v| v.as_i64())
            .ok_or("Missing character_id")? as i32;

        let rest_type_str = arguments
            .get("rest_type")
            .and_then(|v| v.as_str())
            .ok_or("Missing rest_type")?;

        let rest_type = match rest_type_str {
            "short" => RestType::Short,
            "long" => RestType::Long,
            _ => return Err("Invalid rest_type, must be 'short' or 'long'".into()),
        };

        let mut conn = self
            .db_service
            .get_connection()
            .map_err(|e| format!("Database error: {}", e))?;

        let mut char_service = CharacterService::new(&mut conn);
        let (_character, mut char_data) = char_service
            .get_character(character_id)
            .map_err(|e| format!("Character not found: {}", e))?;

        let old_hp = char_data.current_hp;

        match rest_type {
            RestType::Short => {
                // Short rest: can spend hit dice (simplified - just note the rest)
            }
            RestType::Long => {
                // Long rest: restore all HP
                char_data.current_hp = char_data.max_hp;

                // Restore all hit dice
                for class in &mut char_data.classes {
                    class.hit_dice_remaining = class.level;
                }

                // Restore all spell slots
                for slots in char_data.spells.spell_slots.values_mut() {
                    slots.current = slots.max;
                }
            }
        }

        let snapshot_reason = format!(
            "{} rest",
            if rest_type == RestType::Long {
                "Long"
            } else {
                "Short"
            }
        );
        char_service
            .update_character(character_id, char_data.clone(), Some(snapshot_reason))
            .map_err(|e| format!("Failed to update character: {}", e))?;

        let result = json!({
            "success": true,
            "character_id": character_id,
            "character_name": char_data.character_name,
            "rest_type": rest_type_str,
            "hp_before": old_hp,
            "hp_after": char_data.current_hp,
            "message": format!("{} completed {} rest. HP: {}/{}",
                char_data.character_name,
                rest_type_str,
                char_data.current_hp,
                char_data.max_hp
            )
        });

        debug!("Character {} took {} rest", character_id, rest_type_str);
        Ok(serde_json::to_string_pretty(&result)?)
    }
}
